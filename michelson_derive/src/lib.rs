// SPDX-FileCopyrightText: 2023 Baking Bad <hello@bakingbad.dev>
//
// SPDX-License-Identifier: MIT

mod derive;
mod tuple;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(MichelsonInterop)]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    derive::expand_michelson_interop(input).into()
}

#[proc_macro]
pub fn michelson_tuple(input: TokenStream) -> TokenStream {
    let idents = parse_macro_input!(input as tuple::Idents);
    tuple::expand_michelson_tuple(idents).into()
}
