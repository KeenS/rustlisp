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

// pub type _0 = Zero;
// pub type _1 = Succ<_0>;
// pub type _2 = Succ<_1>;
// pub type _3 = Succ<_2>;
// pub type _4 = Succ<_3>;
// pub type _5 = Succ<_4>;
// pub type _6 = Succ<_5>;
// pub type _7 = Succ<_6>;
// pub type _8 = Succ<_7>;
// pub type _9 = Succ<_8>;
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

// pub trait _Add<T1: Num, T2: Num> {
//     type Out: Num;
// }

// impl <T1: Num>_Add<T1, Zero> for Nothing {
//     type Out = T1;
// }

// impl <T1: Num, T2: Num>_Add<T1, Succ<T2>> for Nothing
//     where Nothing: _Add<T1, T2> {
//     type Out = Succ<<Nothing as _Add<T1, T2>>::Out>;
// }

impl <T1: Num> Fun2<Number<T1>, _0> for Add {
    type Out = Number<T1>;
}

impl <T1: Num, T2: Num, AddO: Num> Fun2<Number<T1>, Number<Succ<T2>>> for Add
    where Add: Fun2<Number<T1>, Number<T2>, Out = Number<AddO>> {
    type Out = Number<Succ<AddO>>;
}

// impl <T1: Expr, T2: Expr> Fun2<T1, T2> for Add<T1, T2> {
//     type Out = Stuck;
// }

// pub trait Add<T1: Num, T2: Num>{
//     type Out: Num;
// }

// impl <T1: Num> Add<T1, Zero> for Nothing {
//     type Out = T1;
// }

// impl <T1: Num, T2: Num> Add<T1, Succ<T2>> for Nothing
//     where Nothing: Add<T1, T2>, {
//     type Out = Succ<<Nothing as Add<T1, T2>>::Out>;
// }

