#![allow(dead_code)]
use std::marker::PhantomData;

pub struct Stuck;
pub struct Nothing;
pub trait Expr {
        fn to_string() -> String;
}
impl Expr for Stuck {
    fn to_string() -> String {
        "stuck".to_string()
    }
}
