use heck::SnakeCase;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    self, token::PathSep, DataEnum, DataStruct, Fields, Ident, PathArguments, Type, Variant,
};

fn expand_field_type(ty: Type) -> TokenStream {
    match ty {
        Type::Path(mut path_ty) => {
            path_ty.path.segments.iter_mut().for_each(|seg| {
                match &mut seg.arguments {
                    PathArguments::AngleBracketed(args) => {
                        if args.colon2_token.is_none() {
                            // E.g. turn Vec<T> into Vec::<T>
                            args.colon2_token = Some(PathSep::default());
                        }
                    }
                    _ => {}
                }
            });
            quote! { #path_ty }
        }
        Type::Tuple(tuple) => quote! { <#tuple> },
        ty => unimplemented!("Not a supported field type: {:?}", ty),
    }
}

fn expand_field_ident(ident: Option<Ident>, idx: usize, prefix: TokenStream) -> TokenStream {
    let res = match ident {
        Some(ident) => format!("{}{}", prefix, ident),
        None => format!("{}{}", prefix, idx),
    };
    res.parse::<TokenStream>().unwrap()
}

fn expand_struct_inner_types(fields: Fields) -> Vec<TokenStream> {
    let res: Vec<TokenStream> = fields
        .into_iter()
        .map(|field| {
            let field_type = expand_field_type(field.ty);
            let field_name = match field.ident {
                Some(ident) => {
                    let string = ident.to_string();
                    quote! { Some(#string.into()) }
                }
                None => quote! { None },
            };
            quote! { #field_type::michelson_type(#field_name) }
        })
        .collect();
    if res.len() == 1 {
        unimplemented!("Single struct field is not allowed")
    }
    res
}

fn expand_struct_michelson_type(fields: Fields, field_name: TokenStream) -> TokenStream {
    match fields {
        Fields::Unit => quote! { michelson_interop::adt::create_unit(#field_name) },
        _ => {
            let inner_types = expand_struct_inner_types(fields);
            quote! { michelson_interop::adt::create_pair(vec![#( #inner_types ),*], #field_name) }
        }
    }
}

fn expand_struct_to_michelson(fields: Fields, prefix: TokenStream) -> TokenStream {
    match fields {
        Fields::Unit => quote! { tezos_michelson::michelson::data::unit() },
        _ => {
            let inner_data = fields.into_iter().enumerate().map(|(idx, field)| {
                let field_ident = expand_field_ident(field.ident, idx, prefix.clone());
                quote! { #field_ident.to_michelson()? }
            });
            quote! { tezos_michelson::michelson::data::pair(vec![#( #inner_data ),*]) }
        }
    }
}

fn expand_struct_from_michelson(fields: Fields, constructor: TokenStream) -> TokenStream {
    let michelson_type = expand_struct_michelson_type(fields.clone(), quote! {None});
    match fields {
        Fields::Unit => quote! {
            <()>::from_michelson(data)?;
            #constructor
        },
        Fields::Named(_) => {
            let inner_values = fields.into_iter().map(|field| {
                let field_type = expand_field_type(field.ty);
                let field_ident = field.ident.unwrap();
                quote! { #field_ident: #field_type::from_michelson(pair.values.remove(0))? }
            });
            quote! {
                let mut pair = michelson_interop::adt::flatten_pair(#michelson_type, data)?;
                #constructor { #( #inner_values ),* }
            }
        }
        Fields::Unnamed(_) => {
            let inner_values = fields.into_iter().map(|field| {
                let field_type = expand_field_type(field.ty);
                quote! { #field_type::from_michelson(pair.values.remove(0))? }
            });
            quote! {
                let mut pair = michelson_interop::adt::flatten_pair(#michelson_type, data)?;
                #constructor ( #( #inner_values ),* )
            }
        }
    }
}

pub fn expand_michelson_pair(name: Ident, data: DataStruct) -> TokenStream {
    let michelson_type = expand_struct_michelson_type(data.fields.clone(), quote! {field_name});
    let to_michelson = expand_struct_to_michelson(data.fields.clone(), quote! {self.});
    let from_michelson = expand_struct_from_michelson(data.fields, quote! {Self});
    quote! {
        impl michelson_interop::MichelsonInterop for #name {
            fn michelson_type(field_name: Option<String>) -> tezos_michelson::michelson::types::Type {
                #michelson_type
            }

            fn to_michelson(&self) -> michelson_interop::Result<tezos_michelson::michelson::data::Data> {
                Ok({#to_michelson})
            }

            fn from_michelson(data: tezos_michelson::michelson::data::Data) -> michelson_interop::Result<Self> {
                Ok({#from_michelson})
            }
        }
    }
}

fn expand_variant_to_michelson(
    variant: Variant,
    enum_ident: Ident,
    index: usize,
    total: usize,
) -> TokenStream {
    let prefix = match variant.fields {
        Fields::Unnamed(_) => quote! {_},
        _ => TokenStream::new(),
    };
    let inner_fields = variant
        .fields
        .clone()
        .clone()
        .into_iter()
        .enumerate()
        .map(|(idx, field)| expand_field_ident(field.ident, idx, prefix.clone()));
    let var_signature = match variant.fields.clone() {
        Fields::Unit => TokenStream::new(),
        Fields::Unnamed(_) => quote! { ( #( #inner_fields ),* ) },
        Fields::Named(_) => quote! { { #( #inner_fields ),* } },
    };
    let var_data = expand_struct_to_michelson(variant.fields.clone(), prefix);
    let var_ident = variant.ident;
    quote! {
        #enum_ident::#var_ident #var_signature => {
            Ok(michelson_interop::adt::wrap_or_variant(#var_data, #index, #total))
        }
    }
}

fn expand_variant_from_michelson(variant: Variant, enum_ident: Ident, index: usize) -> TokenStream {
    let var_ident = variant.ident;
    let var_body = expand_struct_from_michelson(variant.fields, quote! {#enum_ident::#var_ident});
    quote! { (data, #index) => { Ok({ #var_body }) } }
}

fn expand_michelson_or(name: Ident, data: DataEnum) -> TokenStream {
    let total = data.variants.len();

    let inner_types = data.variants.clone().into_iter().map(|variant| {
        let field_name = variant.ident.to_string().to_snake_case();
        expand_struct_michelson_type(variant.fields.clone(), quote! {Some(#field_name.into())})
    });

    let inner_to = data
        .variants
        .clone()
        .into_iter()
        .enumerate()
        .map(|(index, variant)| expand_variant_to_michelson(variant, name.clone(), index, total));

    let inner_from = data
        .variants
        .into_iter()
        .enumerate()
        .map(|(index, variant)| expand_variant_from_michelson(variant, name.clone(), index));

    quote! {
        impl michelson_interop::MichelsonInterop for #name {
            fn michelson_type(field_name: Option<String>) -> tezos_michelson::michelson::types::Type {
                let ty = michelson_interop::adt::make_nested_or(vec![#( #inner_types ),*]);
                match field_name {
                    Some(value) => ty.with_field_annotation(value),
                    None => ty.into()
                }
            }

            fn to_michelson(&self) -> michelson_interop::Result<tezos_michelson::michelson::data::Data> {
                match self { #( #inner_to ),* }
            }

            fn from_michelson(data: tezos_michelson::michelson::data::Data) -> michelson_interop::Result<Self> {
                match michelson_interop::adt::unwrap_or_variant(data, #total) {
                    #( #inner_from ),*
                    (_, index) => Err(michelson_interop::Error::InvalidEnumVariant { index })
                }
            }
        }
    }
}

pub fn expand_michelson_interop(input: syn::DeriveInput) -> TokenStream {
    match input.data {
        syn::Data::Enum(data) => expand_michelson_or(input.ident, data),
        syn::Data::Struct(data) => expand_michelson_pair(input.ident, data),
        syn::Data::Union(_) => unimplemented!("Unions are not supported"),
    }
}
