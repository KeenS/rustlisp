#![allow(dead_code)]
use std::marker::PhantomData;
use expr::*;

pub trait Env{ }
pub struct Bottom;
pub struct Stack<Elm: Expr, T: Env> {pelm: PhantomData<Elm>, pt: PhantomData<T>}
impl Env for Bottom {}
impl <Elm: Expr, T: Env>Env for Stack<Elm, T> {}

pub type Stack1<T1: Expr> = Stack<T1, Bottom>;
pub type Stack2<T1: Expr, T2: Expr> = Stack<T1, Stack1<T2>>;
pub type Stack3<T1: Expr, T2: Expr, T3: Expr> = Stack<T1, Stack2<T2, T3>>;
pub type Stack4<T1: Expr, T2: Expr, T3: Expr, T4: Expr> = Stack<T1, Stack3<T2, T3, T4>>;
