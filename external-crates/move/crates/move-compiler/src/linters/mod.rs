// Copyright (c) The Move Contributors
// SPDX-License-Identifier: Apache-2.0

use move_symbol_pool::Symbol;

use crate::{
    command_line::compiler::Visitor, diagnostics::codes::WarningFilter,
    linters::almost_swapped::SwapSequence, typing::visitor::TypingVisitor,
};
pub mod almost_swapped;
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
pub const SWAP_SEQUENCE_FILTER_NAME: &str = "swap_sequence";

pub const LINTER_DEFAULT_DIAG_CODE: u8 = 1;
pub const SWAP_SEQUENCE_DIAG_CODE: u8 = 8;
pub enum LinterDiagCategory {
    Correctness,
}

pub fn known_filters() -> (Option<Symbol>, Vec<WarningFilter>) {
    (
        Some(ALLOW_ATTR_CATEGORY.into()),
        vec![WarningFilter::code(
            Some(LINT_WARNING_PREFIX),
            LinterDiagCategory::Correctness as u8,
            SWAP_SEQUENCE_DIAG_CODE,
            Some(SWAP_SEQUENCE_FILTER_NAME),
        )],
    )
}

pub fn linter_visitors(level: LintLevel) -> Vec<Visitor> {
    match level {
        LintLevel::None => vec![],
        LintLevel::Default | LintLevel::All => {
            vec![almost_swapped::SwapSequence::visitor(SwapSequence)]
        }
    }
}
