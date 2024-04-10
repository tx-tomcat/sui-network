// Copyright (c) The Move Contributors
// SPDX-License-Identifier: Apache-2.0

use move_symbol_pool::Symbol;

use crate::{
    command_line::compiler::Visitor, diagnostics::codes::WarningFilter,
    linters::eq_op::EqualOperandsCheck, typing::visitor::TypingVisitor,
};
pub mod eq_op;
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
pub const EQUAL_OPERANDS_FILTER_NAME: &str = "equal_operands";

pub const LINTER_DEFAULT_DIAG_CODE: u8 = 1;
pub const EQUAL_OPERANDS_DIAG_CODE: u8 = 9;

pub enum LinterDiagCategory {
    Suspicious,
}

pub fn known_filters() -> (Option<Symbol>, Vec<WarningFilter>) {
    (
        Some(ALLOW_ATTR_CATEGORY.into()),
        vec![WarningFilter::code(
            Some(LINT_WARNING_PREFIX),
            LinterDiagCategory::Suspicious as u8,
            EQUAL_OPERANDS_DIAG_CODE,
            Some(EQUAL_OPERANDS_FILTER_NAME),
        )],
    )
}

pub fn linter_visitors(level: LintLevel) -> Vec<Visitor> {
    match level {
        LintLevel::None => vec![],
        LintLevel::Default | LintLevel::All => {
            vec![eq_op::EqualOperandsCheck::visitor(EqualOperandsCheck)]
        }
    }
}
