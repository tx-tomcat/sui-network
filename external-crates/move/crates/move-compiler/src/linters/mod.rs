// Copyright (c) The Move Contributors
// SPDX-License-Identifier: Apache-2.0

use move_symbol_pool::Symbol;

use crate::{
    command_line::compiler::Visitor, diagnostics::codes::WarningFilter,
    linters::ifs_same_cond::ConsecutiveIfs, typing::visitor::TypingVisitor,
};
pub mod ifs_same_cond;
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
pub const CONSECUTIVE_IFS_FILTER_NAME: &str = "consecutive_ifs";

pub const LINTER_DEFAULT_DIAG_CODE: u8 = 1;
pub const CONSECUTIVE_IFS_DIAG_CODE: u8 = 10;
pub enum LinterDiagCategory {
    Correctness,
}

pub fn known_filters() -> (Option<Symbol>, Vec<WarningFilter>) {
    (
        Some(ALLOW_ATTR_CATEGORY.into()),
        vec![WarningFilter::code(
            Some(LINT_WARNING_PREFIX),
            LinterDiagCategory::Correctness as u8,
            CONSECUTIVE_IFS_DIAG_CODE,
            Some(CONSECUTIVE_IFS_FILTER_NAME),
        )],
    )
}

pub fn linter_visitors(level: LintLevel) -> Vec<Visitor> {
    match level {
        LintLevel::None => vec![],
        LintLevel::Default | LintLevel::All => {
            vec![ifs_same_cond::ConsecutiveIfs::visitor(ConsecutiveIfs)]
        }
    }
}
