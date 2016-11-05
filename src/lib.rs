use std::marker::PhantomData;

pub mod expr;
pub mod symbol;
pub mod fun;
pub mod num;
pub mod cons;
pub mod env;
pub mod eval;
pub mod user;


#[macro_export]
macro_rules! eval {
    ($t: ty) => {
        <<$t as Eval>::Out as Expr>::to_string()
    }
}
