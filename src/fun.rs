#![allow(dead_code)]
use expr::*;

pub trait Fun1<T: Expr> {
    type Out: Expr;
}

impl <T: Expr>Fun1<T>  for Stuck {
    type Out = Stuck;
}

pub trait Fun2<T1: Expr, T2: Expr> {
    type Out: Expr;
}

impl <T1: Expr, T2: Expr>Fun2<T1, T2>  for Stuck {
    type Out = Stuck;
}
