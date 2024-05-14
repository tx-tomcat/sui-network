// Copyright (c) The Move Contributors
// SPDX-License-Identifier: Apache-2.0

use move_symbol_pool::Symbol;

use crate::{
    command_line::compiler::Visitor,
    diagnostics::codes::WarningFilter,
    linters::{
        almost_swapped::SwapSequence, constant_naming::ConstantNamingVisitor,
        empty_loop::EmptyLoop, missing_key::MissingKey,
        out_of_bounds_indexing::OutOfBoundsArrayIndexing,
    },
    typing::visitor::TypingVisitor,
};
pub mod almost_swapped;
pub mod constant_naming;
pub mod empty_loop;
pub mod missing_key;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum LinterDiagCategory {
    Correctness,
    Complexity,
    Suspicious,
    Deprecated,
    Style,
    Sui = 99,
}

pub const ALLOW_ATTR_CATEGORY: &str = "lint";
pub const LINT_WARNING_PREFIX: &str = "Lint ";
pub const CONSTANT_NAMING_FILTER_NAME: &str = "constant_naming";
pub const CONSTANT_NAMING_DIAG_CODE: u8 = 1;
pub const OUT_OF_BOUNDS_INDEXING_FILTER_NAME: &str = "out_of_bounds_indexing";
pub const LINTER_OUT_OF_BOUNDS_INDEXING_DIAG_CODE: u8 = 12;
pub const SWAP_SEQUENCE_FILTER_NAME: &str = "swap_sequence";
pub const SWAP_SEQUENCE_DIAG_CODE: u8 = 8;
pub const EMPTY_LOOP_FILTER_NAME: &str = "empty_loop";
pub const EMPTY_LOOP_DIAG_CODE: u8 = 7;
pub const MISSING_KEY_FILTER_NAME: &str = "missing_key";
pub const MISSING_KEY_DIAG_CODE: u8 = 6;

pub fn known_filters() -> (Option<Symbol>, Vec<WarningFilter>) {
    (
        Some(ALLOW_ATTR_CATEGORY.into()),
        vec![
            WarningFilter::code(
                Some(LINT_WARNING_PREFIX),
                LinterDiagCategory::Style as u8,
                CONSTANT_NAMING_DIAG_CODE,
                Some(OUT_OF_BOUNDS_INDEXING_FILTER_NAME),
            ),
            WarningFilter::code(
                Some(LINT_WARNING_PREFIX),
                LinterDiagCategory::Correctness as u8,
                LINTER_OUT_OF_BOUNDS_INDEXING_DIAG_CODE,
                Some(OUT_OF_BOUNDS_INDEXING_FILTER_NAME),
            ),
            WarningFilter::code(
                Some(LINT_WARNING_PREFIX),
                LinterDiagCategory::Correctness as u8,
                SWAP_SEQUENCE_DIAG_CODE,
                Some(SWAP_SEQUENCE_FILTER_NAME),
            ),
            WarningFilter::code(
                Some(LINT_WARNING_PREFIX),
                LinterDiagCategory::Correctness as u8,
                EMPTY_LOOP_DIAG_CODE,
                Some(EMPTY_LOOP_FILTER_NAME),
            ),
            WarningFilter::code(
                Some(LINT_WARNING_PREFIX),
                LinterDiagCategory::Correctness as u8,
                MISSING_KEY_DIAG_CODE,
                Some(MISSING_KEY_FILTER_NAME),
            ),
        ],
    )
}

pub fn linter_visitors(level: LintLevel) -> Vec<Visitor> {
    match level {
        LintLevel::None => vec![],
        LintLevel::Default => vec![],
        LintLevel::All => {
            vec![
                constant_naming::ConstantNamingVisitor::visitor(ConstantNamingVisitor),
                out_of_bounds_indexing::OutOfBoundsArrayIndexing::visitor(OutOfBoundsArrayIndexing),
                almost_swapped::SwapSequence::visitor(SwapSequence),
                empty_loop::EmptyLoop::visitor(EmptyLoop),
                missing_key::MissingKey::visitor(MissingKey),
            ]
        }
    }
}
