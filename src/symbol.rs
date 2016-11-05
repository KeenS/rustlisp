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

pub type Sym1<T1: SymChar> = SymCons<T1, Eos>;
pub type Sym2<T1: SymChar, T2: SymChar> = SymCons<T1, Sym1<T2>>;
pub type Sym3<T1: SymChar, T2: SymChar, T3: SymChar> = SymCons<T1, Sym2<T2, T3>>;
pub type Sym4<T1: SymChar, T2: SymChar, T3: SymChar, T4: SymChar> = SymCons<T1, Sym3<T2, T3, T4>>;


pub type Add = Symbol<Sym3<A, D, D>>;
pub type Mul = Symbol<Sym3<M, U, L>>;
pub type Car = Symbol<Sym3<C, A, R>>;
pub type Cdr = Symbol<Sym3<C, D, R>>;
