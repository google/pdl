// Copyright 2025 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::{cell::RefCell, cmp::max};

use genco::{lang::Java, quote, quote_in, tokens::FormatInto, Tokens};

use crate::backends::java::Integral;

pub type ExprId = usize;

#[derive(Debug, Copy, Clone)]
enum BinOp {
    ShiftLeft,
    ShiftRight,
    BitAnd,
    BitOr,
    Add,
    Sub,
    Multiply,
    Divide,
}

impl FormatInto<Java> for BinOp {
    fn format_into(self, tokens: &mut Tokens<Java>) {
        quote_in! { *tokens =>
            $(match self {
                BinOp::ShiftLeft => <<,
                BinOp::ShiftRight => >>>,
                BinOp::BitAnd => &,
                BinOp::BitOr => |,
                BinOp::Add => +,
                BinOp::Sub => -,
                BinOp::Multiply => *,
                BinOp::Divide => /,
            })
        }
    }
}

#[derive(Debug)]
enum ExprNode {
    Symbol(Tokens<Java>, Integral),
    Number(usize),
    HexNumber(usize),
    BinOp(ExprId, BinOp, ExprId),
    Paren(ExprId),
    Cast(ExprId, Integral),
}

pub fn cast_symbol(symbol: Tokens<Java>, from: Integral, to: Integral) -> Tokens<Java> {
    let t = ExprTree::new();
    t.gen_expr(t.cast(t.symbol(symbol, from), to))
}

pub fn literal(ty: Integral, val: usize) -> Tokens<Java> {
    let t = ExprTree::new();
    t.gen_expr(t.cast(t.num(val), ty))
}

fn gen_mask_val(width: usize) -> usize {
    ((1_u128 << width) - 1).try_into().expect("width must be <= sizeof(usize)")
}

/// A tree representation of a numeric Java expression. This data structure abstracts away most
/// minutiae of generating expressions:
///
/// - Avoids implicit widening by explicitly sign-safe casting all operands where necessary.
/// - Automatically adds parentheses where necessary.
/// - Prunes basic no-op expressions, ie, shift/and/mul by literal 0, apply mask to value with
///   width equal to its type, etc.
///
#[derive(Debug, Default)]
/// **API contract**: At least one argument to all binary operators must be a non-literal (that is,
///  not a `num(..)`).
pub struct ExprTree(RefCell<Vec<ExprNode>>);

impl ExprTree {
    pub fn new() -> Self {
        Self(RefCell::new(Vec::new()))
    }

    /// Convert the tree to Java tokens.
    pub fn gen_expr(&self, id: ExprId) -> Tokens<Java> {
        match self.0.borrow().get(id).unwrap() {
            ExprNode::Symbol(val, ..) => quote!($val),
            ExprNode::HexNumber(val) => quote!($(format!("0x{:x}", val))),
            ExprNode::Number(val) => quote!($(*val)),
            ExprNode::Cast(expr, to) => self.gen_cast(*expr, *to),
            ExprNode::BinOp(lhs, op, rhs) => {
                quote!($(self.gen_expr(*lhs)) $(*op) $(self.gen_expr(*rhs)))
            }
            ExprNode::Paren(expr) => quote!(($(self.gen_expr(*expr)))),
        }
    }

    pub fn clear(&self) {
        self.0.borrow_mut().clear();
    }

    pub fn num(&self, val: usize) -> ExprId {
        self.0.borrow_mut().push(ExprNode::Number(val));
        self.get_root()
    }

    pub fn hex_num(&self, val: usize) -> ExprId {
        self.0.borrow_mut().push(ExprNode::HexNumber(val));
        self.get_root()
    }

    pub fn symbol(&self, val: impl FormatInto<Java>, ty: Integral) -> ExprId {
        self.0.borrow_mut().push(ExprNode::Symbol(quote!($val), ty));
        self.get_root()
    }

    #[allow(dead_code)]
    pub fn add(&self, lhs: ExprId, rhs: ExprId) -> ExprId {
        if self.is_literal(lhs, 0) {
            rhs
        } else if self.is_literal(rhs, 0) {
            lhs
        } else {
            self.op(lhs, BinOp::Add, rhs)
        }
    }

    pub fn sum(&self, exprs: Vec<ExprId>) -> ExprId {
        self.op_all(exprs, BinOp::Add)
    }

    pub fn sub(&self, lhs: ExprId, rhs: ExprId) -> ExprId {
        if self.is_literal(rhs, 0) {
            lhs
        } else {
            self.op(lhs, BinOp::Sub, rhs)
        }
    }

    pub fn mul(&self, lhs: ExprId, rhs: ExprId) -> ExprId {
        if self.is_literal(lhs, 1) {
            rhs
        } else if self.is_literal(rhs, 1) {
            lhs
        } else {
            self.op(lhs, BinOp::Multiply, rhs)
        }
    }

    pub fn div(&self, lhs: ExprId, rhs: ExprId) -> ExprId {
        if self.is_literal(rhs, 1) {
            lhs
        } else {
            self.op(lhs, BinOp::Divide, rhs)
        }
    }

    pub fn lshift(&self, lhs: ExprId, rhs: ExprId) -> ExprId {
        if self.is_literal(lhs, 0) || self.is_literal(rhs, 0) {
            lhs
        } else {
            self.op(lhs, BinOp::ShiftLeft, rhs)
        }
    }

    pub fn rshift(&self, lhs: ExprId, rhs: ExprId) -> ExprId {
        if self.is_literal(lhs, 0) || self.is_literal(rhs, 0) {
            lhs
        } else {
            self.op(lhs, BinOp::ShiftRight, rhs)
        }
    }

    pub fn and(&self, lhs: ExprId, rhs: ExprId) -> ExprId {
        if self.is_literal(lhs, 0) || self.is_literal(rhs, 0) {
            self.num(0)
        } else {
            self.op(lhs, BinOp::BitAnd, rhs)
        }
    }

    #[allow(dead_code)]
    pub fn or(&self, lhs: ExprId, rhs: ExprId) -> ExprId {
        if self.is_literal(lhs, 0) {
            rhs
        } else if self.is_literal(rhs, 0) {
            lhs
        } else {
            self.op(lhs, BinOp::BitOr, rhs)
        }
    }

    pub fn or_all(&self, exprs: Vec<ExprId>) -> ExprId {
        self.op_all(exprs, BinOp::BitOr)
    }

    pub fn paren(&self, expr: ExprId) -> ExprId {
        if matches!(self.0.borrow().get(expr).unwrap(), ExprNode::BinOp(..)) {
            self.0.borrow_mut().push(ExprNode::Paren(expr));
            self.get_root()
        } else {
            expr
        }
    }

    pub fn cast(&self, expr: ExprId, to: Integral) -> ExprId {
        if self.ty(expr) == to {
            expr
        } else {
            let expr = self.paren(expr);
            self.0.borrow_mut().push(ExprNode::Cast(expr, to));
            self.get_root()
        }
    }

    pub fn mask(&self, expr: ExprId, offset: usize, width: usize) -> ExprId {
        let expr = self.paren(self.rshift(expr, self.num(offset)));
        if self.leaf_ty(expr).is_some_and(|ty| ty.width() == width) {
            expr
        } else {
            self.cast(self.and(expr, self.hex_num(gen_mask_val(width))), Integral::fitting(width))
        }
    }

    pub fn compare(&self, expr: ExprId, to: ExprId) -> Tokens<Java> {
        let ty = max(self.ty(expr), self.ty(to)).limit_to_int();

        quote! {
            $(ty.boxed()).compareUnsigned(
                $(self.gen_expr(self.cast(expr, ty))),
                $(self.gen_expr(self.cast(to, ty)))
            )
        }
    }

    pub fn compare_width(&self, expr: ExprId, width: usize) -> Tokens<Java> {
        self.compare(expr, self.cast(self.hex_num(gen_mask_val(width)), self.ty(expr)))
    }

    fn get_root(&self) -> ExprId {
        self.0.borrow().len() - 1
    }

    fn op(&self, lhs: ExprId, op: BinOp, rhs: ExprId) -> ExprId {
        let ty = max(self.ty(lhs), self.ty(rhs)).limit_to_int();

        let node =
            ExprNode::BinOp(self.paren(self.cast(lhs, ty)), op, self.paren(self.cast(rhs, ty)));
        self.0.borrow_mut().push(node);
        self.get_root()
    }

    fn op_all(&self, exprs: Vec<ExprId>, op: BinOp) -> ExprId {
        if exprs.is_empty() {
            self.get_root()
        } else {
            let ty = exprs.iter().map(|expr| self.ty(*expr)).max().unwrap();
            exprs
                .into_iter()
                .map(|expr| self.cast(expr, ty))
                .reduce(|acc, expr| {
                    self.0.borrow_mut().push(ExprNode::BinOp(acc, op, expr));
                    self.get_root()
                })
                .unwrap()
        }
    }

    fn ty(&self, expr: ExprId) -> Integral {
        match self.0.borrow().get(expr).unwrap() {
            ExprNode::Symbol(_, ty) => *ty,
            ExprNode::Number(..) | ExprNode::HexNumber(..) => Integral::Int,
            ExprNode::Cast(_, to) => *to,
            ExprNode::BinOp(lhs, ..) => self.ty(*lhs),
            ExprNode::Paren(expr) => self.ty(*expr),
        }
    }

    fn leaf_ty(&self, expr: ExprId) -> Option<Integral> {
        match self.0.borrow().get(expr).unwrap() {
            ExprNode::Symbol(_, ty) => Some(*ty),
            ExprNode::Number(..) | ExprNode::HexNumber(..) => Some(Integral::Int),
            ExprNode::Cast(expr, ..) => self.leaf_ty(*expr),
            ExprNode::BinOp(..) => None,
            ExprNode::Paren(..) => None,
        }
    }

    fn is_literal(&self, expr: ExprId, literal: usize) -> bool {
        match self.0.borrow().get(expr).unwrap() {
            ExprNode::Symbol(..) | ExprNode::BinOp(..) => false,
            ExprNode::Number(val) | ExprNode::HexNumber(val) => *val == literal,
            ExprNode::Cast(expr, ..) | ExprNode::Paren(expr) => self.is_literal(*expr, literal),
        }
    }

    fn gen_cast(&self, expr: usize, to: Integral) -> Tokens<Java> {
        let from = self.ty(expr);
        match self.0.borrow().get(expr).unwrap() {
            ExprNode::Number(..) | ExprNode::HexNumber(..) if to == Integral::Long => {
                quote!($(self.gen_expr(expr))L)
            }
            ExprNode::Number(..) | ExprNode::HexNumber(..) if to == Integral::Int => {
                quote!($(self.gen_expr(expr)))
            }
            ExprNode::Number(..) | ExprNode::HexNumber(..) => {
                quote!(($to) $(self.gen_expr(expr)))
            }
            _ if from < to => match (from, to) {
                (Integral::Byte, Integral::Short) => {
                    quote!((short) Byte.toUnsignedInt($(self.gen_expr(expr))))
                }
                (_, Integral::Int) => {
                    quote!($(from.boxed()).toUnsignedInt($(self.gen_expr(expr))))
                }
                (_, Integral::Long) => {
                    quote!($(from.boxed()).toUnsignedLong($(self.gen_expr(expr))))
                }
                _ => unreachable!(),
            },
            _ if from > to => quote!(($to) $(self.gen_expr(expr))),
            _ => quote!($(self.gen_expr(expr))),
        }
    }
}

impl FormatInto<Java> for ExprTree {
    fn format_into(self, tokens: &mut Tokens<Java>) {
        quote_in! { *tokens => $(self.gen_expr(self.get_root()))}
    }
}
