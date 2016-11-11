#[macro_use]
extern crate rustlisp;
use rustlisp::expr::*;
use rustlisp::fun::*;
use rustlisp::num::*;
use rustlisp::symbol::*;
use rustlisp::cons::*;
use rustlisp::eval::*;
use rustlisp::user::*;

fn main() {
    println!("{}", Car::to_string());
    println!("{}", _3::to_string());
    println!("{}", <<Add as Fun2<_1, _3>>::Out as Expr>::to_string());
    println!("{}", eval!(list!(Fib, _3)));
}
