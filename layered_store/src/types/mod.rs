// SPDX-FileCopyrightText: 2023 Baking Bad <hello@bakingbad.dev>
//
// SPDX-License-Identifier: MIT

pub mod generic;

#[cfg(any(test, feature = "tezos"))]
pub mod tezos;
