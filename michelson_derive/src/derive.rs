use proc_macro2::TokenStream;
use syn::{self, Ident, PathArguments, Type, DataStruct, DataEnum, token::PathSep};
use quote::quote;

fn expand_field_type(ty: Type) -> TokenStream {
    match ty {
        Type::Path(mut path_ty) => {
            path_ty.path.segments.iter_mut().for_each(|seg| {
                match &mut seg.arguments {
                    PathArguments::AngleBracketed(args) => {
                        if args.colon2_token.is_none() {
                            // E.g. turn Vec<T> to Vec::<T>
                            args.colon2_token = Some(PathSep::default());
                        }
                    },
                    _ => {}
                }
            });
            quote! { #path_ty }
        },
        Type::Tuple(tuple) => quote! { <#tuple> },
        ty => unimplemented!("Not a supported field type: {:?}", ty),
    }
}

fn expand_field_ident(ident: Option<Ident>, idx: usize) -> TokenStream {
    match ident {
        Some(ident) => quote! { #ident },
        None => format!("{}", idx).parse::<TokenStream>().unwrap()
    }
}

pub fn expand_michelson_pair(name: Ident, data: DataStruct) -> TokenStream {
    let fields_ty = data.fields
        .clone()
        .into_iter()
        .map(|field| {
            let field_type = expand_field_type(field.ty);
            let field_name = match field.ident {
                Some(ident) => {
                    let string = ident.to_string();
                    quote! { Some(#string.into()) }
                },
                None => quote! { None }
            };
            quote! { #field_type::michelson_type(#field_name) }
        });

    let fields_to = data.fields
        .clone()
        .into_iter()
        .enumerate()
        .map(|(idx, field)| {
            let field_ident = expand_field_ident(field.ident, idx);
            quote! { self.#field_ident.to_michelson()? }
        });

    let fields_from = data.fields
        .into_iter()
        .enumerate()
        .map(|(idx, field)| {
            let field_type = expand_field_type(field.ty);
            let field_ident = expand_field_ident(field.ident, idx);
            quote! { #field_ident: #field_type::from_michelson(pair.values.remove(0))? }
        });

    quote! {
        impl michelson_interop::MichelsonInterop for #name {
            fn michelson_type(field_name: Option<String>) -> tezos_michelson::michelson::types::Type {
                let ty = tezos_michelson::michelson::types::Pair::new(vec![#( #fields_ty ),*], None);
                match field_name {
                    Some(value) => ty.with_field_annotation(value),
                    None => ty.into()
                }
            }

            fn to_michelson(&self) -> michelson_interop::Result<tezos_michelson::michelson::data::Data> {
                Ok(tezos_michelson::michelson::data::pair(vec![#( #fields_to ),*]))
            }

            fn from_michelson(data: tezos_michelson::michelson::data::Data) -> michelson_interop::Result<Self> {
                let mut pair = michelson_interop::adt::flatten_pair(Self::michelson_type(None), data)?;
                Ok(Self {#( #fields_from ),*})
            }
        }
    }
}

fn expand_michelson_or(name: Ident, data: DataEnum) -> TokenStream {
    todo!();
}

pub fn expand_michelson_interop(input: syn::DeriveInput) -> TokenStream {
    let res = match input.data {
        syn::Data::Enum(data) => expand_michelson_or(input.ident, data),
        syn::Data::Struct(data) => expand_michelson_pair(input.ident, data),
        syn::Data::Union(_) => unimplemented!("Unions are not supported"),
    };

    println!("{}", res.to_string());

    res
}

pub fn expand_michelson_tuple(input: syn::DeriveInput) -> TokenStream {
    match input.data {
        syn::Data::Enum(data) => unreachable!("Expected "),
        syn::Data::Struct(data) => expand_michelson_pair(input.ident, data),
        syn::Data::Union(_) => unreachable!("Unions are not supported"),
    }
}
