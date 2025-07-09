/// The JLS specifies that operands of certain operators including
///  - [shifts](https://docs.oracle.com/javase/specs/jls/se8/html/jls-15.html#jls-15.19)
///  - [bitwise operators](https://docs.oracle.com/javase/specs/jls/se8/html/jls-15.html#jls-15.22.1)
///
/// are subject to [widening primitive conversion](https://docs.oracle.com/javase/specs/jls/se8/html/jls-5.html#jls-5.1.2). Effectively,
/// this means that `byte` or `short` operands are casted to `int` before the operation. Furthermore, Java does not have unsigned types,
/// so:
///
/// > A widening conversion of a signed integer value to an integral type T simply sign-extends the two's-complement representation of the integer value to fill the wider format.
///
/// In other words, bitwise operations on smaller types can change the binary representation of the value before the operation.
/// To get around this, we must sign-safe cast every smaller operand to int or long.
use std::{cell::RefCell, cmp::max};

use genco::{lang::Java, quote, quote_in, tokens::FormatInto, Tokens};

use crate::backends::java::Integral;

type ExprId = usize;

#[derive(Copy, Clone, Debug)]
enum BinOp {
    ShiftLeft,
    ShiftRight,
    And,
    Or,
}

impl FormatInto<Java> for BinOp {
    fn format_into(self, tokens: &mut Tokens<Java>) {
        quote_in! { *tokens =>
            $(match self {
                BinOp::ShiftLeft => <<,
                BinOp::ShiftRight => >>>,
                BinOp::And => &,
                BinOp::Or => |,
            })
        }
    }
}

#[derive(Debug)]
enum ExprNode {
    Symbol { val: Tokens<Java>, ty: Integral },
    Number { val: usize, gen_hex: bool },
    BinOp { lhs: ExprId, op: BinOp, rhs: ExprId },
    Paren { expr: ExprId },
    Cast { expr: ExprId, to: Integral },
}

/// Convert the tree to Java tokens.
pub fn gen_expr(tree: &ExprTree, root: ExprId) -> Tokens<Java> {
    quote!($(tree.gen_expr(root)))
}

pub fn cast_symbol(symbol: Tokens<Java>, from: Integral, to: Integral) -> Tokens<Java> {
    let t = ExprTree::new();
    gen_expr(&t, t.cast(t.symbol(symbol, from), to))
}

pub fn gen_mask(width: usize, ty: Integral) -> Tokens<Java> {
    let t = ExprTree::new();
    gen_expr(&t, t.cast(t.hex_num(gen_mask_val(width)), ty))
}

fn gen_mask_val(width: usize) -> usize {
    ((1_u128 << width) - 1).try_into().expect("width must be <= sizeof(usize)")
}

/// A tree representation of a bitwise Java expression. This data structure abstracts away most
/// minutiae of generating expressions:
///
/// - Avoids implicit widening by explicitly sign-safe casting all operands where necessary.
/// - Automatically adds parentheses where necessary.
/// - Prunes basic no-op expressions, ie, shift/and by literal 0, etc.  
#[derive(Debug)]
pub struct ExprTree(RefCell<Vec<ExprNode>>);

impl ExprTree {
    pub fn new() -> Self {
        Self(RefCell::new(Vec::new()))
    }

    pub fn num(&self, val: usize) -> ExprId {
        self.0.borrow_mut().push(ExprNode::Number { val, gen_hex: false });
        self.get_root()
    }

    pub fn hex_num(&self, val: usize) -> ExprId {
        self.0.borrow_mut().push(ExprNode::Number { val, gen_hex: true });
        self.get_root()
    }

    pub fn symbol(&self, val: impl FormatInto<Java>, ty: Integral) -> ExprId {
        self.0.borrow_mut().push(ExprNode::Symbol { val: quote!($val), ty });
        self.get_root()
    }

    pub fn lshift(&self, lhs: ExprId, rhs: ExprId) -> ExprId {
        if self.is_zero(rhs) {
            lhs
        } else {
            self.op(lhs, BinOp::ShiftLeft, rhs)
        }
    }

    pub fn rshift(&self, lhs: ExprId, rhs: ExprId) -> ExprId {
        if self.is_zero(rhs) {
            lhs
        } else {
            self.op(lhs, BinOp::ShiftRight, rhs)
        }
    }

    pub fn and(&self, lhs: ExprId, rhs: ExprId) -> ExprId {
        if self.is_zero(lhs) {
            rhs
        } else if self.is_zero(rhs) {
            lhs
        } else {
            self.op(lhs, BinOp::And, rhs)
        }
    }

    pub fn or(&self, lhs: ExprId, rhs: ExprId) -> ExprId {
        self.op(lhs, BinOp::Or, rhs)
    }

    pub fn mask(&self, expr: ExprId, width: usize) -> ExprId {
        if self.leaf_ty(expr).is_some_and(|ty| ty.width() == width) {
            expr
        } else {
            self.paren(self.and(expr, self.hex_num(gen_mask_val(width))))
        }
    }

    pub fn or_all(&self, exprs: Vec<ExprId>) -> ExprId {
        exprs
            .into_iter()
            .reduce(|acc, expr| {
                self.0.borrow_mut().push(ExprNode::BinOp { lhs: expr, op: BinOp::Or, rhs: acc });
                self.get_root()
            })
            .unwrap()
    }

    pub fn paren(&self, expr: ExprId) -> ExprId {
        if matches!(self.0.borrow().get(expr).unwrap(), ExprNode::BinOp { .. }) {
            self.0.borrow_mut().push(ExprNode::Paren { expr });
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
            self.0.borrow_mut().push(ExprNode::Cast { expr, to });
            self.get_root()
        }
    }

    pub fn clear(&self) {
        self.0.borrow_mut().clear();
    }

    fn get_root(&self) -> ExprId {
        self.0.borrow().len() - 1
    }

    fn op(&self, lhs: ExprId, op: BinOp, rhs: ExprId) -> ExprId {
        let ty = max(self.ty(lhs), self.ty(rhs)).limit_to_int();

        let node = ExprNode::BinOp {
            lhs: self.paren(self.cast(lhs, ty)),
            op,
            rhs: self.paren(self.cast(rhs, ty)),
        };
        self.0.borrow_mut().push(node);
        self.get_root()
    }

    fn ty(&self, expr: ExprId) -> Integral {
        match self.0.borrow().get(expr).unwrap() {
            ExprNode::Symbol { ty, .. } => *ty,
            ExprNode::Number { .. } => Integral::Int,
            ExprNode::Cast { to, .. } => *to,
            ExprNode::BinOp { lhs, .. } => self.ty(*lhs),
            ExprNode::Paren { expr } => self.ty(*expr),
        }
    }

    fn leaf_ty(&self, expr: ExprId) -> Option<Integral> {
        match self.0.borrow().get(expr).unwrap() {
            ExprNode::Symbol { ty, .. } => Some(*ty),
            ExprNode::Number { .. } => Some(Integral::Int),
            ExprNode::Cast { expr, .. } => self.leaf_ty(*expr),
            ExprNode::BinOp { .. } => None,
            ExprNode::Paren { .. } => None,
        }
    }

    fn is_zero(&self, expr: ExprId) -> bool {
        match self.0.borrow().get(expr).unwrap() {
            ExprNode::Symbol { .. } => false,
            ExprNode::Number { val, .. } => *val == 0,
            ExprNode::Cast { expr, .. } => self.is_zero(*expr),
            ExprNode::BinOp { lhs, op: BinOp::ShiftLeft, .. }
            | ExprNode::BinOp { lhs, op: BinOp::ShiftRight, .. } => self.is_zero(*lhs),
            ExprNode::BinOp { lhs, op: BinOp::And, rhs } => {
                self.is_zero(*lhs) || self.is_zero(*rhs)
            }
            ExprNode::BinOp { lhs, op: BinOp::Or, rhs } => self.is_zero(*lhs) && self.is_zero(*rhs),
            ExprNode::Paren { expr } => self.is_zero(*expr),
        }
    }

    fn gen_expr(&self, id: ExprId) -> Tokens<Java> {
        match self.0.borrow().get(id).unwrap() {
            ExprNode::Symbol { val, .. } => quote!($val),
            ExprNode::Number { val, gen_hex } => quote!(
                $(if *gen_hex {
                    $(format!("0x{:x}", val))
                } else {
                    $(*val)
                })
            ),
            ExprNode::Cast { expr, to } => self.gen_cast(*expr, *to),
            ExprNode::BinOp { lhs, op, rhs } => {
                quote!($(self.gen_expr(*lhs)) $(*op) $(self.gen_expr(*rhs)))
            }
            ExprNode::Paren { expr } => quote!(($(self.gen_expr(*expr)))),
        }
    }

    fn gen_cast(&self, expr: usize, to: Integral) -> Tokens<Java> {
        let from = self.ty(expr);
        match self.0.borrow().get(expr).unwrap() {
            ExprNode::Number { .. } if to == Integral::Long => quote!($(self.gen_expr(expr))L),
            ExprNode::Number { .. } => quote!($(self.gen_expr(expr))),
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
