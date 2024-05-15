//! This rules detects and warns about unnecessary boolean operation such as x || true.
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

use super::{LinterDiagnosticCategory, LINT_WARNING_PREFIX, REDUNDANT_ASSERT_DIAG_CODE};

const REDUNDANT_BOOLEAN_EXP_DIAG: DiagnosticInfo = custom(
    LINT_WARNING_PREFIX,
    Severity::Warning,
    LinterDiagnosticCategory::Complexity as u8,
    REDUNDANT_ASSERT_DIAG_CODE,
    "",
);

pub struct RedundantBooleanExp;

pub struct Context<'a> {
    env: &'a mut CompilationEnv,
}

impl TypingVisitorConstructor for RedundantBooleanExp {
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
        if let UnannotatedExp_::BinopExp(e1, op, _, e2) = &exp.exp.value {
            // Check if the operation is a logical OR
            if let BinOp_::Or = &op.value {
                match (&e1.exp.value, &e2.exp.value) {
                    (UnannotatedExp_::Value(bool), UnannotatedExp_::Copy { var, .. })
                    | (UnannotatedExp_::Copy { var, .. }, UnannotatedExp_::Value(bool)) => {
                        if &Value_::Bool(true) == &bool.value {
                            add_redundant_bool_expr_diag(self.env,exp.exp.loc, "true", "This expression always evaluates to true regardless of the other operand.");
                            return true;
                        }
                        let Var_ { name, .. } = var.value;
                        add_redundant_bool_expr_diag(self.env,exp.exp.loc, name.as_str(), "This expression always evaluates to true regardless of the other operand.");
                    }
                    _ => {}
                }
            }
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

fn add_redundant_bool_expr_diag(
    env: &mut CompilationEnv,
    loc: Loc,
    simplified: &str,
    message: &str,
) {
    let d = diag!(
        REDUNDANT_BOOLEAN_EXP_DIAG,
        (
            loc,
            format!("{}.Consider refactoring it to {}", message, simplified)
        )
    );
    env.add_diag(d);
}
