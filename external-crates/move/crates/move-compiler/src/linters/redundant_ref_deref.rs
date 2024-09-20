// Copyright (c) The Move Contributors
// SPDX-License-Identifier: Apache-2.0

// Implements lint rule for Move code to detect redundant ref/deref patterns.
// It identifies and reports unnecessary temporary borrow followed by a deref, suggesting either
// removal or conversion to `copy`.

use crate::linters::StyleCodes;
use crate::{
    diag,
    diagnostics::WarningFilters,
    shared::CompilationEnv,
    typing::{
        ast::{self as T, Exp, UnannotatedExp_ as TE},
        visitor::{TypingVisitorConstructor, TypingVisitorContext},
    },
};

pub struct RedundantRefDerefVisitor;

pub struct Context<'a> {
    env: &'a mut CompilationEnv,
}

impl TypingVisitorConstructor for RedundantRefDerefVisitor {
    type Context<'a> = Context<'a>;

    fn context<'a>(env: &'a mut CompilationEnv, _program: &T::Program) -> Self::Context<'a> {
        Context { env }
    }
}

impl TypingVisitorContext for Context<'_> {
    fn add_warning_filter_scope(&mut self, filter: WarningFilters) {
        self.env.add_warning_filter_scope(filter)
    }
    fn pop_warning_filter_scope(&mut self) {
        self.env.pop_warning_filter_scope()
    }

    fn visit_exp_custom(&mut self, exp: &Exp) -> bool {
        self.check_redundant_ref_deref(exp);
        false
    }
}

impl Context<'_> {
    // Check for &* pattern
    fn check_redundant_ref_deref(&mut self, exp: &Exp) {
        let TE::Dereference(deref_exp) = &exp.exp.value else {
            return;
        };
        // This is a carve-out to handle cases like `&(s.value), which generate a field borrow and
        // dereference to perform a  `copy`. In those cases, the location information is reused for
        // both, meaning it was generated by typing, and thus not subject to warnings.
        // TODO(cswords): In the future, we should mark which of these are generated during typing
        // to handle paths.
        if exp.exp.loc == deref_exp.exp.loc {
            return;
        }
        match &deref_exp.exp.value {
            TE::TempBorrow(_, inner) if is_simple_deref_ref_exp(inner) => self.env.add_diag(diag!(
                StyleCodes::RedundantRefDeref.diag_info(),
                (
                    exp.exp.loc,
                    "Redundant borrow-dereference detected. \
                     Remove this borrow-deref and use the expression directly."
                )
            )),
            TE::TempBorrow(_, inner) if all_deref_borrow(inner) => self.env.add_diag(diag!(
                StyleCodes::RedundantRefDeref.diag_info(),
                (
                    exp.exp.loc,
                    "Redundant borrow-dereference detected. \
                     Use the inner expression directly."
                )
            )),
            TE::Borrow(false, _, _) if exp.exp.loc != deref_exp.exp.loc => {
                self.env.add_diag(diag!(
                    StyleCodes::RedundantRefDeref.diag_info(),
                    (
                        exp.exp.loc,
                        "Redundant borrow-dereference detected. \
                         Use the field access directly."
                    )
                ))
            }
            TE::Borrow(_, _, _) | TE::BorrowLocal(_, _) => self.env.add_diag(diag!(
                StyleCodes::RedundantRefDeref.diag_info(),
                (
                    exp.exp.loc,
                    "Redundant borrow-dereference detected. \
                     Replace this borrow-deref with 'copy'."
                )
            )),
            _ => (),
        }
    }
}

/// Indicates if the expression is of the form `[&*]+....e`
fn all_deref_borrow(exp: &Exp) -> bool {
    let TE::Dereference(deref_exp) = &exp.exp.value else {
        return false;
    };
    match &deref_exp.exp.value {
        TE::TempBorrow(_, inner) => all_deref_borrow(inner),
        TE::Borrow(_, _, _) | TE::BorrowLocal(_, _) => true,
        _ => false,
    }
}

/// Indicates if the expression at hand is of a form where `&*&` can always be reduces to `&`, such
/// as function calls and constants.
fn is_simple_deref_ref_exp(exp: &Exp) -> bool {
    match &exp.exp.value {
        TE::Value(_) => true,
        TE::Constant(_, _) => true,
        TE::ErrorConstant { .. } => true,
        TE::ModuleCall(_) => true,
        TE::Vector(_, _, _, _) => true,
        TE::Copy { .. } => true,

        TE::Cast(inner, _) | TE::Annotate(inner, _) => is_simple_deref_ref_exp(inner),

        TE::Move { .. } => false, // Copy case
        TE::Use(_) => todo!(),
        TE::IfElse(_, _, _)
        | TE::Match(_, _)
        | TE::VariantMatch(_, _, _)
        | TE::Loop { .. }
        | TE::NamedBlock(_, _)
        | TE::Block(_)
        | TE::Dereference(_)
        | TE::UnaryExp(_, _)
        | TE::BinopExp(_, _, _, _)
        | TE::Pack(_, _, _, _)
        | TE::PackVariant(_, _, _, _, _)
        | TE::ExpList(_)
        | TE::Borrow(_, _, _)
        | TE::TempBorrow(_, _)
        | TE::BorrowLocal(_, _) => false,
        // These are already errors
        TE::Unit { .. }
        | TE::Builtin(_, _)
        | TE::While(_, _, _)
        | TE::Assign(_, _, _)
        | TE::Return(_)
        | TE::Abort(_)
        | TE::Mutate(_, _)
        | TE::Give(_, _)
        | TE::Continue(_)
        | TE::UnresolvedError => false,
    }
}
