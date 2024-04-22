// Copyright (c) The Move Contributors
// SPDX-License-Identifier: Apache-2.0

use move_symbol_pool::Symbol;

use crate::{
    command_line::compiler::Visitor, diagnostics::codes::WarningFilter,
    linters::freezing_capability::WarnFreezeCapability, typing::visitor::TypingVisitor,
};
pub mod freezing_capability;
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
pub const WARN_FREEZE_CAPABILITY_FILTER_NAME: &str = "freezing_capability";

pub const LINTER_DEFAULT_DIAG_CODE: u8 = 1;
pub const WARN_FREEZE_CAPABILITY_DIAG_CODE: u8 = 16;

pub enum LinterDiagCategory {
    Suspicous,
}

pub fn known_filters() -> (Option<Symbol>, Vec<WarningFilter>) {
    (
        Some(ALLOW_ATTR_CATEGORY.into()),
        vec![WarningFilter::code(
            Some(LINT_WARNING_PREFIX),
            LinterDiagCategory::Suspicous as u8,
            WARN_FREEZE_CAPABILITY_DIAG_CODE,
            Some(WARN_FREEZE_CAPABILITY_FILTER_NAME),
        )],
    )
}

pub fn linter_visitors(level: LintLevel) -> Vec<Visitor> {
    match level {
        LintLevel::None => vec![],
        LintLevel::Default | LintLevel::All => {
            vec![freezing_capability::WarnFreezeCapability::visitor(
                WarnFreezeCapability,
            )]
        }
    }
}
