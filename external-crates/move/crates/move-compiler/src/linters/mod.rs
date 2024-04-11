// Copyright (c) The Move Contributors
// SPDX-License-Identifier: Apache-2.0

use move_symbol_pool::Symbol;

use crate::{
    command_line::compiler::Visitor, diagnostics::codes::WarningFilter,
    linters::out_of_bounds_indexing::OutOfBoundsArrayIndexing, typing::visitor::TypingVisitor,
};
pub mod out_of_bounds_indexing;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LintLevel {
    // No linters
    None,
    // Run only the default linters
    Default,
    // Run all linters
    All,
}

pub const ALLOW_ATTR_CATEGORY: &str = "lint";
pub const LINT_WARNING_PREFIX: &str = "Lint ";
pub const OUT_OF_BOUNDS_INDEXING_FILTER_NAME: &str = "out_of_bounds_indexing";

pub const LINTER_DEFAULT_DIAG_CODE: u8 = 1;
pub const LINTER_OUT_OF_BOUNDS_INDEXING_DIAG_CODE: u8 = 12;

pub enum LinterDiagCategory {
    Correctness,
}

pub fn known_filters() -> (Option<Symbol>, Vec<WarningFilter>) {
    (
        Some(ALLOW_ATTR_CATEGORY.into()),
        vec![WarningFilter::code(
            Some(LINT_WARNING_PREFIX),
            LinterDiagCategory::Correctness as u8,
            LINTER_OUT_OF_BOUNDS_INDEXING_DIAG_CODE,
            Some(OUT_OF_BOUNDS_INDEXING_FILTER_NAME),
        )],
    )
}

pub fn linter_visitors(level: LintLevel) -> Vec<Visitor> {
    match level {
        LintLevel::None => vec![],
        LintLevel::Default | LintLevel::All => {
            vec![out_of_bounds_indexing::OutOfBoundsArrayIndexing::visitor(
                OutOfBoundsArrayIndexing,
            )]
        }
    }
}
