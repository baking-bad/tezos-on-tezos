// SPDX-FileCopyrightText: 2023 Baking Bad <hello@bakingbad.dev>
//
// SPDX-License-Identifier: MIT

use std::fmt::Display;

use crate::types::{BigMapDiff, InternalContent, OperationItem};

impl OperationItem {
    pub fn new(content: InternalContent) -> Self {
        Self {
            content,
            big_map_diff: Vec::new(),
        }
    }

    pub fn into_content(self) -> InternalContent {
        self.content
    }

    pub fn aggregate_diff(&mut self, big_map_diff: &mut Vec<BigMapDiff>) {
        big_map_diff.append(&mut self.big_map_diff)
    }
}

impl PartialEq for OperationItem {
    fn eq(&self, other: &Self) -> bool {
        // For testing purposes
        self.content == other.content
    }
}

impl Display for OperationItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Operation")
    }
}
