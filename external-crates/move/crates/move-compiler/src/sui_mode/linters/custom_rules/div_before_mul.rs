//! This linter rule detects division operations followed directly by multiplication within the same expression.
//! It warns that this order may amplify rounding errors in integer arithmetic, suggesting an operation reorder.
use crate::{
    diag,
    diagnostics::{
        codes::{custom, DiagnosticInfo, Severity},
        WarningFilters,
    },
    parser::ast::BinOp_,
    shared::{program_info::TypingProgramInfo, CompilationEnv},
    sui_mode::linters::{LinterDiagCategory, LINTER_DEFAULT_DIAG_CODE, LINT_WARNING_PREFIX},
    typing::{
        ast::{self as T, UnannotatedExp_},
        visitor::{TypingVisitorConstructor, TypingVisitorContext},
    },
};
use move_ir_types::location::Loc;

const DIV_BEFORE_MUL_DIAG: DiagnosticInfo = custom(
    LINT_WARNING_PREFIX,
    Severity::Warning,
    LinterDiagCategory::DivisionBeforeMultiplication as u8,
    LINTER_DEFAULT_DIAG_CODE,
    "Division before multiplication may lead to amplified rounding errors. Consider changing the order of operations.",
);

pub struct DivisionBeforeMultiplicationVisitor;

pub struct Context<'a> {
    env: &'a mut CompilationEnv,
}

impl TypingVisitorConstructor for DivisionBeforeMultiplicationVisitor {
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
        if let UnannotatedExp_::BinopExp(lhs, op, _, rhs) = &exp.exp.value {
            match op.value {
                BinOp_::Mul => {
                    // Check if the RHS is a multiplication operation
                    if let UnannotatedExp_::BinopExp(_, rhs_op, _, _) = lhs.exp.value {
                        if matches!(rhs_op.value, BinOp_::Div) {
                            report_div_before_mul(self.env, op.loc);
                        }
                    }
                }
                _ => {}
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

fn report_div_before_mul(env: &mut CompilationEnv, loc: Loc) {
    let diag = diag!(
        DIV_BEFORE_MUL_DIAG,
        (loc, "Division before multiplication may lead to amplified rounding errors. Consider changing the order of operations.")
    );
    env.add_diag(diag);
}
