// Copyright (c) The Move Contributors
// SPDX-License-Identifier: Apache-2.0

use move_symbol_pool::Symbol;

use crate::{
    command_line::compiler::Visitor, diagnostics::codes::WarningFilter,
    linters::shift_overflow::ShiftOperationOverflow, typing::visitor::TypingVisitor,
};
pub mod shift_overflow;

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

pub const SHILF_OVERFLOW_FILTER_NAME: &str = "shift_overflow";

pub const LINTER_DEFAULT_DIAG_CODE: u8 = 1;
pub const MOVE_LINT_WARNING_SHIFT_OVERFLOW: u8 = 2;
pub enum LinterDiagCategory {
    PotentialOverflow,
}
pub fn known_filters() -> (Option<Symbol>, Vec<WarningFilter>) {
    (
        Some(ALLOW_ATTR_CATEGORY.into()),
        vec![WarningFilter::code(
            Some(LINT_WARNING_PREFIX),
            LinterDiagCategory::PotentialOverflow as u8,
            MOVE_LINT_WARNING_SHIFT_OVERFLOW,
            Some(SHILF_OVERFLOW_FILTER_NAME),
        )],
    )
}

pub fn linter_visitors(level: LintLevel) -> Vec<Visitor> {
    match level {
        LintLevel::None => vec![],
        LintLevel::Default | LintLevel::All => {
            vec![shift_overflow::ShiftOperationOverflow::visitor(
                ShiftOperationOverflow,
            )]
        }
    }
}
