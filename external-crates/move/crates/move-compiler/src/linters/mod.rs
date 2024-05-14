// Copyright (c) The Move Contributors
// SPDX-License-Identifier: Apache-2.0

use move_symbol_pool::Symbol;

use crate::{
    command_line::compiler::Visitor,
    diagnostics::codes::WarningFilter,
    linters::{
        abort_constant::AssertAbortNamedConstants, almost_swapped::SwapSequence,
        constant_naming::ConstantNamingVisitor, empty_loop::EmptyLoop,
        ifs_same_cond::ConsecutiveIfs, missing_key::MissingKey, needless_else::EmptyElseBranch,
        out_of_bounds_indexing::OutOfBoundsArrayIndexing,
        redundant_conditional::RedundantConditional, shift_overflow::ShiftOperationOverflow,
    },
    typing::visitor::TypingVisitor,
};
pub mod abort_constant;
pub mod almost_swapped;
pub mod constant_naming;
pub mod empty_loop;
pub mod ifs_same_cond;
pub mod missing_key;
pub mod needless_else;
pub mod out_of_bounds_indexing;
pub mod redundant_conditional;
pub mod self_assignment;
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

pub const REDUNDANT_CONDITIONAL_FILTER_NAME: &str = "redundant_conditional";
pub const REDUNDANT_CONDITIONAL_DIAG_CODE: u8 = 2;

pub const EMPTY_ELSE_BRANCH_FILTER_NAME: &str = "needless_else";
pub const EMPTY_ELSE_BRANCH_DIAG_CODE: u8 = 3;

pub const SHILF_OVERFLOW_FILTER_NAME: &str = "shift_overflow";
pub const SHILF_OVERFLOW_DIAG_CODE: u8 = 4;

pub const ABORT_CONSTANT_FILTER_NAME: &str = "abort_constant";
pub const ABORT_CONSTANT_DIAG_CODE: u8 = 5;

pub const MISSING_KEY_FILTER_NAME: &str = "missing_key";
pub const MISSING_KEY_DIAG_CODE: u8 = 6;

pub const EMPTY_LOOP_FILTER_NAME: &str = "empty_loop";
pub const EMPTY_LOOP_DIAG_CODE: u8 = 7;

pub const SWAP_SEQUENCE_FILTER_NAME: &str = "swap_sequence";
pub const SWAP_SEQUENCE_DIAG_CODE: u8 = 8;

pub const CONSECUTIVE_IFS_FILTER_NAME: &str = "consecutive_ifs";
pub const CONSECUTIVE_IFS_DIAG_CODE: u8 = 9;

pub const OUT_OF_BOUNDS_INDEXING_FILTER_NAME: &str = "out_of_bounds_indexing";
pub const OUT_OF_BOUNDS_INDEXING_DIAG_CODE: u8 = 10;

pub const SELF_ASSIGNMENT_FILTER_NAME: &str = "self-assignment";
pub const SELF_ASSIGNMENT_DIAG_CODE: u8 = 11;

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
                OUT_OF_BOUNDS_INDEXING_DIAG_CODE,
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
            WarningFilter::code(
                Some(LINT_WARNING_PREFIX),
                LinterDiagCategory::Complexity as u8,
                REDUNDANT_CONDITIONAL_DIAG_CODE,
                Some(REDUNDANT_CONDITIONAL_FILTER_NAME),
            ),
            WarningFilter::code(
                Some(LINT_WARNING_PREFIX),
                LinterDiagCategory::Complexity as u8,
                EMPTY_ELSE_BRANCH_DIAG_CODE,
                Some(EMPTY_ELSE_BRANCH_FILTER_NAME),
            ),
            WarningFilter::code(
                Some(LINT_WARNING_PREFIX),
                LinterDiagCategory::Correctness as u8,
                CONSECUTIVE_IFS_DIAG_CODE,
                Some(CONSECUTIVE_IFS_FILTER_NAME),
            ),
            WarningFilter::code(
                Some(LINT_WARNING_PREFIX),
                LinterDiagCategory::Style as u8,
                ABORT_CONSTANT_DIAG_CODE,
                Some(ABORT_CONSTANT_FILTER_NAME),
            ),
            WarningFilter::code(
                Some(LINT_WARNING_PREFIX),
                LinterDiagCategory::Correctness as u8,
                SHILF_OVERFLOW_DIAG_CODE,
                Some(SHILF_OVERFLOW_FILTER_NAME),
            ),
            WarningFilter::code(
                Some(LINT_WARNING_PREFIX),
                LinterDiagCategory::Suspicious as u8,
                SELF_ASSIGNMENT_DIAG_CODE,
                Some(SELF_ASSIGNMENT_FILTER_NAME),
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
                redundant_conditional::RedundantConditional::visitor(RedundantConditional),
                needless_else::EmptyElseBranch::visitor(EmptyElseBranch),
                ifs_same_cond::ConsecutiveIfs::visitor(ConsecutiveIfs),
                abort_constant::AssertAbortNamedConstants::visitor(AssertAbortNamedConstants),
                shift_overflow::ShiftOperationOverflow::visitor(ShiftOperationOverflow),
                self_assignment::SelfAssignmentCheck::visitor(SelfAssignmentCheck),
            ]
        }
    }
}
