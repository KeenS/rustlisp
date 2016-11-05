#![allow(dead_code)]
use std::marker::PhantomData;
use expr::*;
use fun::*;
use symbol::*;

pub struct Nil{}
impl Expr for Nil {
    fn to_string() -> String {
        "nil".to_string()
    }
}
pub struct ConsCell<T1: Expr, T2: Expr> {
    p1: PhantomData<T1>,
    p2: PhantomData<T2>,
}

impl <T1: Expr, T2: Expr> Expr for ConsCell<T1, T2>{
    fn to_string() -> String {
        format!("({} . {})", T1::to_string(), T2::to_string())
    }
}

pub type List1<T1: Expr> = ConsCell<T1, Nil>;
pub type List2<T1: Expr, T2: Expr> = ConsCell<T1, List1<T2>>;
pub type List3<T1: Expr, T2: Expr, T3: Expr> = ConsCell<T1, List2<T2, T3>>;
pub type List4<T1: Expr, T2: Expr, T3: Expr, T4: Expr> = ConsCell<T1, List3<T2, T3, T4>>;

trait Cons<T1: Expr, T2: Expr> {
    type Out: Expr;
}

impl <T1: Expr, T2: Expr>Cons<T1, T2> for Nothing {
    type Out = ConsCell<T1, T2>;
}


impl <T1: Expr, T2: Expr> Fun1<ConsCell<T1, T2>> for Car {
    type Out = T1;
}

impl <T1: Expr, T2: Expr> Fun1<ConsCell<T1, T2>> for Cdr {
    type Out = T2;
}
