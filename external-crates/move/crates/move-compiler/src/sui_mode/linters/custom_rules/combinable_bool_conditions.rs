use move_ir_types::location::Loc;

use crate::sui_mode::linters::{LinterDiagCategory, LINTER_DEFAULT_DIAG_CODE, LINT_WARNING_PREFIX};
use crate::{
    diag,
    diagnostics::{
        codes::{custom, DiagnosticInfo, Severity},
        WarningFilters,
    },
    parser::ast::BinOp_,
    shared::{program_info::TypingProgramInfo, CompilationEnv},
    typing::{
        ast::{self as T, UnannotatedExp_},
        visitor::{TypingVisitorConstructor, TypingVisitorContext},
    },
};

const COMBINABLE_BOOL_COND_DIAG: DiagnosticInfo = custom(
    LINT_WARNING_PREFIX,
    Severity::Warning,
    LinterDiagCategory::CombinableBool as u8,
    LINTER_DEFAULT_DIAG_CODE,
    "",
);

pub struct CombinableBoolVisitor;

pub struct Context<'a> {
    env: &'a mut CompilationEnv,
}

impl TypingVisitorConstructor for CombinableBoolVisitor {
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
        eprintln!("Visiting exp {:?}", exp);
        if let UnannotatedExp_::BinopExp(e1, _op, _, e2) = &exp.exp.value {
            if let (
                UnannotatedExp_::BinopExp(_, op1, _, _),
                UnannotatedExp_::BinopExp(_, op2, _, _),
            ) = (&e1.exp.value, &e2.exp.value)
            {
                // Check if operands are the same
                match (&op1.value, &op2.value) {
                    // Existing simplification cases
                    (BinOp_::Eq, BinOp_::Lt) | (BinOp_::Lt, BinOp_::Eq) => {
                        add_replaceable_comparison_diag(
                            self.env,
                            exp.exp.loc,
                            "Consider simplifying to `<=`.",
                        );
                    }
                    (BinOp_::Eq, BinOp_::Gt) | (BinOp_::Gt, BinOp_::Eq) => {
                        add_replaceable_comparison_diag(
                            self.env,
                            exp.exp.loc,
                            "Consider simplifying to `>=`.",
                        );
                    }
                    // New cases for removing unnecessary `!=`
                    (BinOp_::Neq, BinOp_::Lt)
                    | (BinOp_::Lt, BinOp_::Neq)
                    | (BinOp_::Neq, BinOp_::Gt)
                    | (BinOp_::Gt, BinOp_::Neq) => {
                        add_replaceable_comparison_diag(
                            self.env, exp.exp.loc,"Unequal (!=) condition is unnecessary and can be removed for simplicity.");
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

fn add_replaceable_comparison_diag(env: &mut CompilationEnv, loc: Loc, message: &str) {
    let d = diag!(COMBINABLE_BOOL_COND_DIAG, (loc, message));
    env.add_diag(d);
}
