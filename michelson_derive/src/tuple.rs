// SPDX-FileCopyrightText: 2023 Baking Bad <hello@bakingbad.dev>
//
// SPDX-License-Identifier: MIT

use find_crate::find_crate;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{parse::Parse, punctuated::Punctuated, Ident, Result, Token};

pub struct Idents(pub Punctuated<Ident, Token![,]>);

impl Parse for Idents {
    fn parse(input: syn::parse::ParseStream) -> Result<Self> {
        Ok(Idents(Punctuated::parse_terminated(input)?))
    }
}

pub fn expand_michelson_tuple(idents: Idents) -> TokenStream {
    let indices = idents
        .0
        .clone()
        .into_iter()
        .enumerate()
        .map(|(idx, _)| format!("{}", idx).parse::<TokenStream>().unwrap());

    let generics = idents.0.clone().into_iter().map(|ident| {
        quote! { #ident: MichelsonInterop }
    });

    let types = idents.0.clone().into_iter();

    let inner_types = idents.0.clone().into_iter().map(|ident| {
        quote! { #ident::michelson_type(None) }
    });

    let values = idents.0.clone().into_iter().map(|ident| {
        quote! { #ident::from_michelson(pair.values.remove(0))? }
    });

    let crate_name = match find_crate(|s| s == "michelson_interop") {
        Ok(pkg) => pkg.name,
        Err(_) => "crate".into(),
    };
    let michelson_interop = Ident::new(&crate_name, Span::call_site());

    quote! {
        impl< #( #generics ),* > MichelsonInterop for ( #( #types ),* ) {
            fn michelson_type(field_name: Option<String>) -> Type {
                let ty = types::Pair::new(vec![ #( #inner_types ),* ], None);
                match field_name {
                    Some(name) => ty.with_field_annotation(name),
                    None => ty.into()
                }
            }

            fn to_michelson(&self) -> Result<data::Data> {
                Ok(data::pair(vec![ #( self.#indices.to_michelson()? ),* ]))
            }

            fn from_michelson(data: Data) -> Result<Self> {
                let mut pair = #michelson_interop::adt::flatten_pair(Self::michelson_type(None), data)?;
                Ok(( #( #values ),* ))
            }
        }
    }
}
