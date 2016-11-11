#![allow(dead_code)]
use std::marker::PhantomData;
use expr::*;

pub struct Symbol<S: Sym>{
    s: PhantomData<S>
}

impl <S: Sym>Expr for Symbol<S> {
    fn to_string() -> String {
        S::to_string()
    }
}

pub trait Sym {
    fn to_string() -> String;
}
pub trait SymChar{
    fn to_string() -> String;
}
pub struct Eos;
pub struct SymCons<C: SymChar, S: Sym> {
    cp: PhantomData<C>,
    sp: PhantomData<S>,
}
impl Sym for Eos{
    fn to_string() -> String {
        "".to_string()
    }
}
impl <C: SymChar, S: Sym>Sym for SymCons<C, S>{
    fn to_string() -> String {
        C::to_string() + &S::to_string()
    }

}

macro_rules! symchar {
    ($c: ident) => {
        pub struct $c;
        impl SymChar for $c {
            fn to_string() -> String {
                stringify!($c).to_string()
            }
        }
    }
}
macro_rules! symchars {
    ($c1: ident) => {
        symchar!($c1);
    };
    ($c1: ident,) => {
        symchar!($c1);
    };
    ($c1: ident $(, $c: ident)*) => {
        symchar!($c1);
        symchars!($($c,)*);
    };
    ($c1: ident $(, $c: ident)*,) => {
        symchar!($c1);
        symchars!($($c,)*);
    };
}
symchars!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z);

#[macro_export]
macro_rules! sym {
    () => {
        $crate::symbol::Eos
    };
    (,) => {
        $crate::symbol::Eos
    };
    (, $t: ty $(, $tys: ty)*) => {
        $crate::symbol::SymCons<$t , sym!($(, $tys)*)>
    };
    ($t: ty $(, $tys: ty)*) => {
        $crate::symbol::SymCons<$t , sym!($(, $tys)*)>
    };
}

#[macro_export]
macro_rules! symbol {
    ($($sym:ty)*) => {
        Symbol<sym!($(, $sym)*)>
    };
}
