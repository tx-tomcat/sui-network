// Copyright (c) The Move Contributors
// SPDX-License-Identifier: Apache-2.0

use move_symbol_pool::Symbol;

use crate::{
    command_line::compiler::Visitor, diagnostics::codes::WarningFilter,
    linters::bool_comparison::BoolComparison, typing::visitor::TypingVisitor,
};
pub mod bool_comparison;
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
pub const BOOL_COMPARISON_FILTER_NAME: &str = "bool_comparison";

pub const LINTER_DEFAULT_DIAG_CODE: u8 = 1;
pub const LINTER_WARNING_REDUNDANT_BOOL_CODE: u8 = 4;
pub enum LinterDiagCategory {
    Redundancy,
}

pub fn known_filters() -> (Option<Symbol>, Vec<WarningFilter>) {
    (
        Some(ALLOW_ATTR_CATEGORY.into()),
        vec![WarningFilter::code(
            Some(LINT_WARNING_PREFIX),
            LinterDiagCategory::Redundancy as u8,
            LINTER_WARNING_REDUNDANT_BOOL_CODE,
            Some(BOOL_COMPARISON_FILTER_NAME),
        )],
    )
}

pub fn linter_visitors(level: LintLevel) -> Vec<Visitor> {
    match level {
        LintLevel::Default | LintLevel::None => vec![],
        LintLevel::All => {
            vec![bool_comparison::BoolComparison::visitor(BoolComparison)]
        }
    }
}
