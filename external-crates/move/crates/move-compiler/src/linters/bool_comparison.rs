// Copyright (c) The Move Contributors
// SPDX-License-Identifier: Apache-2.0

//! Detects comparisons where a variable is compared to 'true' or 'false' using
//! equality (==) or inequality (!=) operators and provides suggestions to simplify the comparisons.
//! Examples: if (x == true) can be simplified to if (x), if (x == false) can be simplified to if (!x)
use crate::{
    diag,
    diagnostics::{
        codes::{custom, DiagnosticInfo, Severity},
        WarningFilters,
    },
    expansion::ast::Value_,
    naming::ast::Var_,
    parser::ast::BinOp_,
    shared::{program_info::TypingProgramInfo, CompilationEnv},
    typing::{
        ast::{self as T, UnannotatedExp_},
        visitor::{TypingVisitorConstructor, TypingVisitorContext},
    },
};
use move_ir_types::location::Loc;

use super::{LinterDiagCategory, LINTER_WARNING_REDUNDANT_BOOL_CODE, LINT_WARNING_PREFIX};

const BOOL_COMPARISON_DIAG: DiagnosticInfo = custom(
    LINT_WARNING_PREFIX,
    Severity::Warning,
    LinterDiagCategory::Redundancy as u8,
    LINTER_WARNING_REDUNDANT_BOOL_CODE,
    "unnecessary boolean comparison to true or false",
);

pub struct BoolComparison;

pub struct Context<'a> {
    env: &'a mut CompilationEnv,
}

impl TypingVisitorConstructor for BoolComparison {
    type Context<'a> = Context<'a>;

    fn context<'a>(
        env: &'a mut CompilationEnv,
        _program_info: &'a TypingProgramInfo,
        _program: &T::Program_,
    ) -> Self::Context<'a> {
        Context { env }
    }
}

impl TypingVisitorContext for Context<'_> {
    fn visit_exp_custom(&mut self, exp: &mut T::Exp) -> bool {
        let UnannotatedExp_::BinopExp(e1, op, _, e2) = &exp.exp.value else {
            return false;
        };
        // Check if the operation is a logical OR
        match &op.value {
            BinOp_::Or => {
                // Check if one of the operands is a boolean literal
                if let (UnannotatedExp_::Value(bool), UnannotatedExp_::Copy { var, .. })
                | (UnannotatedExp_::Copy { var, .. }, UnannotatedExp_::Value(bool)) =
                    (&e1.exp.value, &e2.exp.value)
                {
                    // Check if the boolean literal is true
                    if &Value_::Bool(true) == &bool.value {
                        add_bool_comparison_diag(self.env, exp.exp.loc, "This expression always evaluates to true regardless of the other operand.");
                        return true;
                    } else {
                        let Var_ { name, .. } = var.value;
                        add_bool_comparison_diag(self.env, exp.exp.loc, format!(
                                    "This expression always evaluates to {} regardless of the other operand.",
                                    name.as_str()
                                ).as_str());
                    }
                }
            }
            BinOp_::And => {
                // Check if one of the operands is a boolean literal
                if let (UnannotatedExp_::Value(bool), UnannotatedExp_::Copy { var, .. })
                | (UnannotatedExp_::Copy { var, .. }, UnannotatedExp_::Value(bool)) =
                    (&e1.exp.value, &e2.exp.value)
                {
                    // Check if the boolean literal is false
                    if &Value_::Bool(false) == &bool.value {
                        add_bool_comparison_diag(self.env, exp.exp.loc, "This expression always evaluates to false regardless of the other operand.");
                        return true;
                    } else {
                        let Var_ { name, .. } = var.value;
                        add_bool_comparison_diag(self.env, exp.exp.loc, format!(
                                    "This expression always evaluates to {} regardless of the other operand.",
                                    name.as_str()
                                ).as_str());
                    }
                }
            }
            BinOp_::Eq | BinOp_::Neq => {
                // Check if one side is a boolean literal and the other is a boolean expression
                let ((UnannotatedExp_::Value(sp!(_, Value_::Bool(b))), _)
                | (_, UnannotatedExp_::Value(sp!(_, Value_::Bool(b))))) =
                    (&e1.exp.value, &e2.exp.value)
                else {
                    return false;
                };

                let simplification = match (op.value, b) {
                    (BinOp_::Eq, true) | (BinOp_::Neq, false) => {
                        "Consider simplifying this expression to the variable or function itself."
                    }
                    (BinOp_::Eq, false) | (BinOp_::Neq, true) => {
                        "Consider simplifying this expression using logical negation '!'."
                    }
                    _ => return false, // This case should not occur
                };

                if !simplification.is_empty() {
                    add_bool_comparison_diag(self.env, exp.exp.loc, simplification);
                }
            }
            _ => {}
        }

        false
    }
    fn add_warning_filter_scope(&mut self, filter: WarningFilters) {
        self.env.add_warning_filter_scope(filter)
    }

    fn pop_warning_filter_scope(&mut self) {
        self.env.pop_warning_filter_scope()
    }
}

fn add_bool_comparison_diag(env: &mut CompilationEnv, loc: Loc, message: &str) {
    let d = diag!(
        BOOL_COMPARISON_DIAG,
        (
            loc,
            format!("This boolean comparison is unnecessary. {}", message)
        )
    );
    env.add_diag(d);
}
