// Copyright (c) The Move Contributors
// SPDX-License-Identifier: Apache-2.0

use move_symbol_pool::Symbol;

use crate::{
    command_line::compiler::Visitor,
    diagnostics::codes::WarningFilter,
    linters::{
        abort_constant::AssertAbortNamedConstants,
        absurd_extreme_comparisons::LikelyComparisonMistake, almost_swapped::SwapSequence,
        bool_comparison::BoolComparison, collapsible_else_if::CollapsibleElseIf,
        collapsible_nested_if::CollapsibleNestedIf, combinable_bool_conditions::CombinableBool,
        constant_naming::ConstantNamingVisitor, div_before_mul::DivisionBeforeMultiplication,
        double_comparisons::DoubleComparisonCheck, empty_loop::EmptyLoop,
        excessive_nesting::ExcessiveNesting, freezing_capability::WarnFreezeCapability,
        ifs_same_cond::ConsecutiveIfs, impossible_comparisons::ImpossibleDoubleComparison,
        meaningless_math_operation::MeaninglessMathOperation, missing_key::MissingKey,
        multiplication_overflow::MultiplicationOverflow, needless_else::EmptyElseBranch,
        out_of_bounds_indexing::OutOfBoundsArrayIndexing,
        public_mut_tx_context::RequireMutableTxContext, redundant_assert::AssertTrueFals,
        redundant_conditional::RedundantConditional, redundant_ref_deref::RedundantRefDerefVisitor,
        self_assignment::SelfAssignmentCheck, shift_overflow::ShiftOperationOverflow,
        too_many_arguments::ExcessiveParametersCheck,
        unnecessary_mut_params::UnusedMutableParamsCheck, unnecessary_while_loop::WhileTrueToLoop,
    },
    typing::visitor::TypingVisitor,
};
pub mod abort_constant;
pub mod absurd_extreme_comparisons;
pub mod almost_swapped;
pub mod bool_comparison;
pub mod collapsible_else_if;
pub mod collapsible_nested_if;
pub mod combinable_bool_conditions;
pub mod constant_naming;
pub mod div_before_mul;
pub mod double_comparisons;
pub mod empty_loop;
pub mod eq_op;
pub mod excessive_nesting;
pub mod freezing_capability;
pub mod ifs_same_cond;
pub mod impossible_comparisons;
pub mod meaningless_math_operation;
pub mod missing_key;
pub mod multiplication_overflow;
pub mod needless_else;
pub mod out_of_bounds_indexing;
pub mod public_mut_tx_context;
pub mod redundant_assert;
pub mod redundant_boolean_expressions;
pub mod redundant_conditional;
pub mod redundant_ref_deref;
pub mod self_assignment;
pub mod shift_overflow;
pub mod too_many_arguments;
pub mod unnecessary_mut_params;
pub mod unnecessary_while_loop;

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
pub enum LinterDiagnosticCategory {
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

pub const REDUNDANT_ASSERT_FILTER_NAME: &str = "redundant_assert";
pub const REDUNDANT_ASSERT_DIAG_CODE: u8 = 12;

pub const EXCESSIVE_NESTING_FILTER_NAME: &str = "excessive_nesting";
pub const EXCESSIVE_NESTING_DIAG_CODE: u8 = 13;

pub const MULTIPLICATION_OVERFLOW_FILTER_NAME: &str = "multiplication_overflow";
pub const MULTIPLICATION_OVERFLOW_DIAG_CODE: u8 = 14;

pub const EXCESSIVE_PARAMS_FILTER_NAME: &str = "excessive_params";
pub const EXCESSIVE_PARAMS_DIAG_CODE: u8 = 15;

pub const DOUBLE_COMPARISON_FILTER_NAME: &str = "double_comparison";
pub const DOUBLE_COMPARISON_DIAG_CODE: u8 = 16;

pub const BOOL_COMPARISON_FILTER_NAME: &str = "bool_comparison";
pub const BOOL_COMPARISON_DIAG_CODE: u8 = 17;

pub const COLLAPSIBLE_ELSE_FILTER_NAME: &str = "collapsible_else_if";
pub const COLLAPSIBLE_ELSE_DIAG_CODE: u8 = 18;

pub const REDUNDANT_REF_DEREF_FILTER_NAME: &str = "redundant_ref_deref";
pub const REDUNDANT_REF_DEREF_DIAG_CODE: u8 = 19;

pub const WARN_FREEZE_CAPABILITY_FILTER_NAME: &str = "freezing_capability";
pub const WARN_FREEZE_CAPABILITY_DIAG_CODE: u8 = 20;

pub const COLLAPSIBLE_NESTED_IF_FILTER_NAME: &str = "collapsible_nested_if";
pub const COLLAPSIBLE_NESTED_IF_DIAG_CODE: u8 = 21;

pub const REQUIRE_MUTABLE_TX_CONTEXT_FILTER_NAME: &str = "public_mut_tx_context";
pub const REQUIRE_MUTABLE_TX_CONTEXT_DIAG_CODE: u8 = 22;

pub const WHILE_TRUE_TO_LOOP_FILTER_NAME: &str = "unnecessary_while_loop";
pub const WHILE_TRUE_TO_LOOP_DIAG_CODE: u8 = 23;

pub const IMPOSSIBLE_COMPARISON_FILTER_NAME: &str = "impossible_comparison";
pub const IMPOSSIBLE_COMPARISON_DIAG_CODE: u8 = 24;

pub const UNUSED_MUT_PARAMS_FILTER_NAME: &str = "unused_mut_params";
pub const UNUSED_MUT_PARAMS_DIAG_CODE: u8 = 25;

pub const DIV_BEFORE_MUL_FILTER_NAME: &str = "div_before_mul";
pub const DIV_BEFORE_MUL_DIAG_CODE: u8 = 26;

pub const MEANINGLESS_MATH_OP_FILTER_NAME: &str = "meaningless_math_op";
pub const MEANINGLESS_MATH_OP_DIAG_CODE: u8 = 27;

pub const COMBINABLE_BOOL_FILTER_NAME: &str = "combinable_bool_conditions";
pub const COMBINABLE_BOOL_DIAG_CODE: u8 = 28;

pub const ABSURD_EXTREME_COMPARISON_FILTER_NAME: &str = "absurd_extreme_comparisons";
pub const ABSURD_EXTREME_COMPARISON_DIAG_CODE: u8 = 29;

pub const REDUNDANT_BOOLEAN_EXP_FILTER_NAME: &str = "redundant_boolean_expressions";
pub const REDUNDANT_BOOLEAN_EXP_DIAG_CODE: u8 = 30;

pub const EQUAL_OPERANDS_FILTER_NAME: &str = "equal_operands";
pub const EQUAL_OPERANDS_DIAG_CODE: u8 = 31;

pub fn known_filters() -> (Option<Symbol>, Vec<WarningFilter>) {
    (
        Some(ALLOW_ATTR_CATEGORY.into()),
        vec![
            WarningFilter::code(
                Some(LINT_WARNING_PREFIX),
                LinterDiagnosticCategory::Style as u8,
                CONSTANT_NAMING_DIAG_CODE,
                Some(OUT_OF_BOUNDS_INDEXING_FILTER_NAME),
            ),
            WarningFilter::code(
                Some(LINT_WARNING_PREFIX),
                LinterDiagnosticCategory::Correctness as u8,
                OUT_OF_BOUNDS_INDEXING_DIAG_CODE,
                Some(OUT_OF_BOUNDS_INDEXING_FILTER_NAME),
            ),
            WarningFilter::code(
                Some(LINT_WARNING_PREFIX),
                LinterDiagnosticCategory::Correctness as u8,
                SWAP_SEQUENCE_DIAG_CODE,
                Some(SWAP_SEQUENCE_FILTER_NAME),
            ),
            WarningFilter::code(
                Some(LINT_WARNING_PREFIX),
                LinterDiagnosticCategory::Correctness as u8,
                EMPTY_LOOP_DIAG_CODE,
                Some(EMPTY_LOOP_FILTER_NAME),
            ),
            WarningFilter::code(
                Some(LINT_WARNING_PREFIX),
                LinterDiagnosticCategory::Correctness as u8,
                MISSING_KEY_DIAG_CODE,
                Some(MISSING_KEY_FILTER_NAME),
            ),
            WarningFilter::code(
                Some(LINT_WARNING_PREFIX),
                LinterDiagnosticCategory::Complexity as u8,
                REDUNDANT_CONDITIONAL_DIAG_CODE,
                Some(REDUNDANT_CONDITIONAL_FILTER_NAME),
            ),
            WarningFilter::code(
                Some(LINT_WARNING_PREFIX),
                LinterDiagnosticCategory::Complexity as u8,
                EMPTY_ELSE_BRANCH_DIAG_CODE,
                Some(EMPTY_ELSE_BRANCH_FILTER_NAME),
            ),
            WarningFilter::code(
                Some(LINT_WARNING_PREFIX),
                LinterDiagnosticCategory::Correctness as u8,
                CONSECUTIVE_IFS_DIAG_CODE,
                Some(CONSECUTIVE_IFS_FILTER_NAME),
            ),
            WarningFilter::code(
                Some(LINT_WARNING_PREFIX),
                LinterDiagnosticCategory::Style as u8,
                ABORT_CONSTANT_DIAG_CODE,
                Some(ABORT_CONSTANT_FILTER_NAME),
            ),
            WarningFilter::code(
                Some(LINT_WARNING_PREFIX),
                LinterDiagnosticCategory::Correctness as u8,
                SHILF_OVERFLOW_DIAG_CODE,
                Some(SHILF_OVERFLOW_FILTER_NAME),
            ),
            WarningFilter::code(
                Some(LINT_WARNING_PREFIX),
                LinterDiagnosticCategory::Suspicious as u8,
                SELF_ASSIGNMENT_DIAG_CODE,
                Some(SELF_ASSIGNMENT_FILTER_NAME),
            ),
            WarningFilter::code(
                Some(LINT_WARNING_PREFIX),
                LinterDiagnosticCategory::Correctness as u8,
                REDUNDANT_ASSERT_DIAG_CODE,
                Some(REDUNDANT_ASSERT_FILTER_NAME),
            ),
            WarningFilter::code(
                Some(LINT_WARNING_PREFIX),
                LinterDiagnosticCategory::Complexity as u8,
                EXCESSIVE_NESTING_DIAG_CODE,
                Some(EXCESSIVE_NESTING_FILTER_NAME),
            ),
            WarningFilter::code(
                Some(LINT_WARNING_PREFIX),
                LinterDiagnosticCategory::Suspicious as u8,
                MULTIPLICATION_OVERFLOW_DIAG_CODE,
                Some(MULTIPLICATION_OVERFLOW_FILTER_NAME),
            ),
            WarningFilter::code(
                Some(LINT_WARNING_PREFIX),
                LinterDiagnosticCategory::Complexity as u8,
                EXCESSIVE_PARAMS_DIAG_CODE,
                Some(EXCESSIVE_PARAMS_FILTER_NAME),
            ),
            WarningFilter::code(
                Some(LINT_WARNING_PREFIX),
                LinterDiagnosticCategory::Complexity as u8,
                DOUBLE_COMPARISON_DIAG_CODE,
                Some(DOUBLE_COMPARISON_FILTER_NAME),
            ),
            WarningFilter::code(
                Some(LINT_WARNING_PREFIX),
                LinterDiagnosticCategory::Correctness as u8,
                BOOL_COMPARISON_DIAG_CODE,
                Some(BOOL_COMPARISON_FILTER_NAME),
            ),
            WarningFilter::code(
                Some(LINT_WARNING_PREFIX),
                LinterDiagnosticCategory::Style as u8,
                COLLAPSIBLE_ELSE_DIAG_CODE,
                Some(COLLAPSIBLE_ELSE_FILTER_NAME),
            ),
            WarningFilter::code(
                Some(LINT_WARNING_PREFIX),
                LinterDiagnosticCategory::Correctness as u8,
                REDUNDANT_REF_DEREF_DIAG_CODE,
                Some(REDUNDANT_REF_DEREF_FILTER_NAME),
            ),
            WarningFilter::code(
                Some(LINT_WARNING_PREFIX),
                LinterDiagnosticCategory::Suspicious as u8,
                WARN_FREEZE_CAPABILITY_DIAG_CODE,
                Some(WARN_FREEZE_CAPABILITY_FILTER_NAME),
            ),
            WarningFilter::code(
                Some(LINT_WARNING_PREFIX),
                LinterDiagnosticCategory::Style as u8,
                COLLAPSIBLE_NESTED_IF_DIAG_CODE,
                Some(COLLAPSIBLE_NESTED_IF_FILTER_NAME),
            ),
            WarningFilter::code(
                Some(LINT_WARNING_PREFIX),
                LinterDiagnosticCategory::Correctness as u8,
                REQUIRE_MUTABLE_TX_CONTEXT_DIAG_CODE,
                Some(REQUIRE_MUTABLE_TX_CONTEXT_FILTER_NAME),
            ),
            WarningFilter::code(
                Some(LINT_WARNING_PREFIX),
                LinterDiagnosticCategory::Style as u8,
                WHILE_TRUE_TO_LOOP_DIAG_CODE,
                Some(WHILE_TRUE_TO_LOOP_FILTER_NAME),
            ),
            WarningFilter::code(
                Some(LINT_WARNING_PREFIX),
                LinterDiagnosticCategory::Correctness as u8,
                IMPOSSIBLE_COMPARISON_DIAG_CODE,
                Some(IMPOSSIBLE_COMPARISON_FILTER_NAME),
            ),
            WarningFilter::code(
                Some(LINT_WARNING_PREFIX),
                LinterDiagnosticCategory::Suspicious as u8,
                UNUSED_MUT_PARAMS_DIAG_CODE,
                Some(UNUSED_MUT_PARAMS_FILTER_NAME),
            ),
            WarningFilter::code(
                Some(LINT_WARNING_PREFIX),
                LinterDiagnosticCategory::Suspicious as u8,
                DIV_BEFORE_MUL_DIAG_CODE,
                Some(DIV_BEFORE_MUL_FILTER_NAME),
            ),
            WarningFilter::code(
                Some(LINT_WARNING_PREFIX),
                LinterDiagnosticCategory::Complexity as u8,
                MEANINGLESS_MATH_OP_DIAG_CODE,
                Some(MEANINGLESS_MATH_OP_FILTER_NAME),
            ),
            WarningFilter::code(
                Some(LINT_WARNING_PREFIX),
                LinterDiagnosticCategory::Complexity as u8,
                COMBINABLE_BOOL_DIAG_CODE,
                Some(COMBINABLE_BOOL_FILTER_NAME),
            ),
            WarningFilter::code(
                Some(LINT_WARNING_PREFIX),
                LinterDiagnosticCategory::Correctness as u8,
                ABSURD_EXTREME_COMPARISON_FILTER_NAME,
                Some(ABSURD_EXTREME_COMPARISON_FILTER_NAME),
            ),
            WarningFilter::code(
                Some(LINT_WARNING_PREFIX),
                LinterDiagnosticCategory::Complexity as u8,
                LINTER_DEFAULT_DIAG_CODE,
                Some(REDUNDANT_BOOLEAN_EXP_FILTER_NAME),
            ),
            WarningFilter::code(
                Some(LINT_WARNING_PREFIX),
                LinterDiagnosticCategory::Suspicious as u8,
                EQUAL_OPERANDS_DIAG_CODE,
                Some(EQUAL_OPERANDS_FILTER_NAME),
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
                redundant_assert::AssertTrueFals::visitor(AssertTrueFals),
                excessive_nesting::ExcessiveNesting::visitor(ExcessiveNesting),
                multiplication_overflow::MultiplicationOverflow::visitor(MultiplicationOverflow),
                too_many_arguments::ExcessiveParametersCheck::visitor(ExcessiveParametersCheck),
                double_comparisons::DoubleComparisonCheck::visitor(DoubleComparisonCheck),
                bool_comparison::BoolComparison::visitor(BoolComparison),
                collapsible_else_if::CollapsibleElseIf::visitor(CollapsibleElseIf),
                redundant_ref_deref::RedundantRefDerefVisitor::visitor(RedundantRefDerefVisitor),
                freezing_capability::WarnFreezeCapability::visitor(WarnFreezeCapability),
                collapsible_nested_if::CollapsibleNestedIf::visitor(CollapsibleNestedIf),
                public_mut_tx_context::RequireMutableTxContext::visitor(RequireMutableTxContext),
                unnecessary_while_loop::WhileTrueToLoop::visitor(WhileTrueToLoop),
                impossible_comparisons::ImpossibleDoubleComparison::visitor(
                    ImpossibleDoubleComparison,
                ),
                unnecessary_mut_params::UnusedMutableParamsCheck::visitor(UnusedMutableParamsCheck),
                div_before_mul::DivisionBeforeMultiplication::visitor(DivisionBeforeMultiplication),
                meaningless_math_operation::MeaninglessMathOperation::visitor(
                    MeaninglessMathOperation,
                ),
                combinable_bool_conditions::CombinableBool::visitor(CombinableBool),
                absurd_extreme_comparisons::LikelyComparisonMistake::visitor(
                    LikelyComparisonMistake,
                ),
                redundant_boolean_expressions::RedundantBooleanExp::visitor(RedundantBooleanExp),
                eq_op::EqualOperandsCheck::visitor(EqualOperandsCheck),
            ]
        }
    }
}
