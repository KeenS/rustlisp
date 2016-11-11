#![allow(dead_code)]
use std::marker::PhantomData;
use expr::*;
use env::*;
use fun::*;
use num::*;
use symbol::*;
use cons::*;

// trait Eval<E: Expr> {
//     type Out: Env;
// }

// impl <Ev: Env, N: Num>Eval<Number<N>> for Ev {
//     type Out = Stack<Number<N>, Ev>;
// }

// impl <Ev: Env, S: Sym>Eval<Symbol<S>> for Ev {
//     type Out = Stack<Symbol<S>, Ev>;
// }

// trait Call<E: Expr> {
//     type Out: Env;
// }

// impl <Ev: Env, S: Sym, T1: Expr, T1O: Expr, FO: Expr>Eval<List2<Symbol<S>, T1>> for Ev
//     where Ev: Eval<T1, Out = Stack<T1O, Ev>>,
//           Symbol<S>: Fun1<T1O, Out = FO>,
// {
//     type Out = Stack<FO, Ev>;
// }

pub trait Eval {
    type Out: Expr;
}

impl <N: Num>Eval for Number<N> {
    type Out = Number<N>;
}

impl <S: Sym>Eval for Symbol<S> {
    type Out = Symbol<S>;
}

impl <S: Sym, T1: Expr, T1O: Expr, O: Expr>Eval for list!(Symbol<S>, T1)
    where T1: Eval<Out = T1O>,
          Symbol<S>: Fun1<T1O, Out = O>,
{
    type Out = O;
}

impl <S: Sym, T1: Expr, T2: Expr, T1O: Expr, T2O: Expr, O: Expr>Eval for list!(Symbol<S>, T1, T2)
    where T1: Eval<Out = T1O>,
          T2: Eval<Out = T2O>,
          Symbol<S>: Fun2<T1O, T2O, Out = O>,
{
    type Out = O;
}

