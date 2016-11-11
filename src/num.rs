#![allow(dead_code)]
use std::marker::PhantomData;
use expr::*;
use fun::*;
use symbol::*;

pub struct Number<N: Num> {
    number: PhantomData<N>,
}

impl <N: Num>Expr for Number<N> {
    fn to_string() -> String {
        N::to_string()
    }
}

pub trait Num {
    fn to_string() -> String {
        Self::to_int().to_string()
    }
    fn to_int() -> isize;
}
pub struct Zero;
pub struct Succ<T: Num> {t: PhantomData<T>}
impl Num for Zero {
    fn to_int() -> isize {
        0
    }
}
impl <T: Num>Num for Succ<T> {
    fn to_int() -> isize {
        1 + T::to_int()
    }
}

macro_rules! numty {
    ($num: ident, $n: ident, $p: ty) => {
        pub type $n= Succ<$p>;
        pub type $num = Number<$n>;
    }
}

pub type __0 = Zero;
pub type _0 = Number<__0>;
numty!(_1, __1, __0);
numty!(_2, __2, __1);
numty!(_3, __3, __2);
numty!(_4, __4, __3);
numty!(_5, __5, __4);
numty!(_6, __6, __5);
numty!(_7, __7, __6);
numty!(_8, __8, __7);
numty!(_9, __9, __8);
numty!(_10, __10, __9);


pub type Add = symbol!(A D D);
pub type Mul = symbol!(M U L);

impl <T1: Num> Fun2<Number<T1>, _0> for Add {
    type Out = Number<T1>;
}

impl <T1: Num, T2: Num, ON: Num> Fun2<Number<T1>, Number<Succ<T2>>> for Add
    where Add: Fun2<Number<Succ<T1>>, Number<T2>, Out = Number<ON>> {
    type Out = Number<ON>;
}


