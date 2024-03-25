//! Detect potential overflow scenarios where the number of bits being shifted exceeds the bit width of
//! the variable being shifted, which could lead to unintended behavior or loss of data. If such a
//! potential overflow is detected, a warning is generated to alert the developer.
use crate::{
    diag,
    diagnostics::{
        codes::{custom, DiagnosticInfo, Severity},
        WarningFilters,
    },
    expansion::ast::Value_,
    naming::ast::{BuiltinTypeName_, TypeName_, Type_},
    parser::ast::BinOp_,
    shared::{program_info::TypingProgramInfo, CompilationEnv},
    typing::{
        ast::{self as T, UnannotatedExp_},
        visitor::{TypingVisitorConstructor, TypingVisitorContext},
    },
};
use move_ir_types::location::Loc;
use std::str::FromStr;

use super::{LinterDiagCategory, LINTER_DEFAULT_DIAG_CODE, LINT_WARNING_PREFIX};

const REDUNDANT_CONDITIONAL_DIAG: DiagnosticInfo = custom(
    LINT_WARNING_PREFIX,
    Severity::Warning,
    LinterDiagCategory::RedundantConditional as u8,
    LINTER_DEFAULT_DIAG_CODE,
    "Potential overflow detected. The number of bits being shifted exceeds the bit width of the variable being shifted.",
);

pub struct RedundantConditional;

pub struct Context<'a> {
    env: &'a mut CompilationEnv,
}

impl TypingVisitorConstructor for RedundantConditional {
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
        match exp.exp.value {
            // Match against an if-else expression pattern
            UnannotatedExp_::IfElse(condition, if_block, else_block) => {
                // Check if the if and else blocks are simply returning true and false respectively
                // let if_block_returns_true = matches!(
                //     if_block.exp.value,
                //     UnannotatedExp_::Value(Value_::Bool(true))
                // );
                // let else_block_returns_false =
                //     matches!(else_block.exp, UnannotatedExp_::Value(Value_::Bool(false)));

                // // Check the reverse case: if returns false and else returns true
                // let if_block_returns_false =
                //     matches!(if_block.exp, UnannotatedExp_::Value(Value_::Bool(false)));
                // let else_block_returns_true =
                //     matches!(else_block.exp, UnannotatedExp_::Value(Value_::Bool(true)));

                // if (if_block_returns_true && else_block_returns_false)
                //     || (if_block_returns_false && else_block_returns_true)
                // {
                //     report_redundant_conditional(self.env, exp.exp.loc, &condition);
                // }
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

fn report_overflow(env: &mut CompilationEnv, shift_amount: u128, bit_width: u128, loc: Loc) {
    let msg = format!(
        "The {} of bits being shifted exceeds the {} bit width of the variable being shifted.",
        shift_amount, bit_width
    );
    let diag = diag!(REDUNDANT_CONDITIONAL_DIAG, (loc, msg));
    env.add_diag(diag);
}
