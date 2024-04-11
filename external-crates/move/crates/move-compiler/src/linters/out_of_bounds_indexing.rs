// Copyright (c) The Move Contributors
// SPDX-License-Identifier: Apache-2.0

//! Detects out-of-bounds array (or vector) indexing with a constant index in Move code.
//! This lint aims to statically identify instances where an index access on an array or vector
//! exceeds the bounds known at compile time, potentially indicating logical errors in the code.
use crate::{
    diag,
    diagnostics::{
        codes::{custom, DiagnosticInfo, Severity},
        WarningFilters,
    },
    expansion::ast::{ModuleIdent, Value_},
    naming::ast::Var_,
    parser::ast::FunctionName,
    shared::{program_info::TypingProgramInfo, CompilationEnv},
    typing::{
        ast::{self as T, ExpListItem, LValue_, ModuleCall, UnannotatedExp_},
        visitor::{TypingVisitorConstructor, TypingVisitorContext},
    },
};
use move_ir_types::location::Loc;
use std::collections::BTreeMap;

use super::{LinterDiagCategory, LINTER_DEFAULT_DIAG_CODE, LINT_WARNING_PREFIX};

const SHIFT_OPERATION_OVERFLOW_DIAG: DiagnosticInfo = custom(
    LINT_WARNING_PREFIX,
    Severity::Warning,
    LinterDiagCategory::Correctness as u8,
    LINTER_DEFAULT_DIAG_CODE,
    "Potential overflow detected. The number of bits being shifted exceeds the bit width of the variable being shifted.",
);

pub struct OutOfBoundsArrayIndexing;

pub struct Context<'a> {
    env: &'a mut CompilationEnv,
    array_list: BTreeMap<Var_, usize>,
}

impl TypingVisitorConstructor for OutOfBoundsArrayIndexing {
    type Context<'a> = Context<'a>;

    fn context<'a>(
        env: &'a mut CompilationEnv,
        _program_info: &'a TypingProgramInfo,
        _program: &T::Program_,
    ) -> Self::Context<'a> {
        Context {
            env,
            array_list: BTreeMap::new(),
        }
    }
}

impl TypingVisitorContext for Context<'_> {
    fn visit_function_custom(
        &mut self,
        _module: ModuleIdent,
        _function_name: FunctionName,
        fdef: &mut T::Function,
    ) -> bool {
        if let T::FunctionBody_::Defined(seq) = &mut fdef.body.value {
            seq.1.iter().for_each(|seq_item| {
                if let T::SequenceItem_::Bind(value_list, _, seq_exp) = &seq_item.value {
                    if let UnannotatedExp_::Vector(_, size, _, _) = &seq_exp.exp.value {
                        if let Some(sp!(_, LValue_::Var { var, .. })) = &value_list.value.get(0) {
                            self.array_list.insert(var.value, *size);
                        }
                    }
                }
            });
        }
        false
    }
    fn visit_exp_custom(&mut self, exp: &mut T::Exp) -> bool {
        if let UnannotatedExp_::ModuleCall(module_call) = &exp.exp.value {
            let a = is_vector_borrow(module_call);
            if let UnannotatedExp_::ExpList(exp_list) = &module_call.arguments.exp.value {
                if let Some(ExpListItem::Single(arr_arg_exp, _)) = &exp_list.get(0) {
                    if let UnannotatedExp_::BorrowLocal(_, sp!(_, array_arg)) =
                        &arr_arg_exp.exp.value
                    {
                        if let Some(array_size) = self.array_list.get(array_arg) {
                            if let Some(ExpListItem::Single(value_exp, _)) = &exp_list.get(1) {
                                if let UnannotatedExp_::Value(sp!(_, size)) = &value_exp.exp.value {
                                    let index = extract_value(&size);
                                    if index - 1 > *array_size as u128 {
                                        report_out_of_bounds_indexing(
                                            self.env,
                                            array_arg,
                                            index,
                                            exp.exp.loc,
                                        );
                                    }
                                }
                            }
                        }
                    }
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

fn is_vector_borrow(module_call: &ModuleCall) -> bool {
    (module_call.name.0.value.as_str() == "borrow"
        || module_call.name.0.value.as_str() == "borrow_mut")
        && module_call.module.value.module.0.value.as_str() == "vector"
}

fn extract_value(value: &Value_) -> u128 {
    match value {
        Value_::U8(v) => *v as u128,
        Value_::U16(v) => *v as u128,
        Value_::U32(v) => *v as u128,
        Value_::U64(v) => *v as u128,
        Value_::U128(v) => *v,
        _ => 0,
    }
}
fn report_out_of_bounds_indexing(env: &mut CompilationEnv, var: &Var_, index: u128, loc: Loc) {
    let msg = format!(
        "Array index out of bounds: attempting to access index {} in array '{}' with size known at compile time.",
        index, var.name.as_str()
    );
    let diag = diag!(SHIFT_OPERATION_OVERFLOW_DIAG, (loc, msg));
    env.add_diag(diag);
}
