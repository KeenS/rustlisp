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

#[macro_export]
macro_rules! list {
    () => {
        $crate::cons::Nil
    };
    (,) => {
        $crate::cons::Nil
    };
    (, $t: ty $(, $tys: ty)*) => {
        $crate::cons::ConsCell<$t , list!($(, $tys)*)>
    };
    ($t: ty $(, $tys: ty)*) => {
        $crate::cons::ConsCell<$t , list!($(, $tys)*)>
    };
}

pub type Cons = symbol!(C O N S);
pub type Car = symbol!(C A R);
pub type Cdr = symbol!(C D R);


impl <T1: Expr, T2: Expr> Fun2<T1, T2> for Cons {
    type Out = ConsCell<T1, T2>;
}

impl <T1: Expr, T2: Expr> Fun1<ConsCell<T1, T2>> for Car {
    type Out = T1;
}

impl <T1: Expr, T2: Expr> Fun1<ConsCell<T1, T2>> for Cdr {
    type Out = T2;
}
