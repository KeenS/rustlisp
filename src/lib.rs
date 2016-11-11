use std::marker::PhantomData;

pub mod expr;
#[macro_use]
pub mod symbol;
pub mod fun;
pub mod num;
#[macro_use]
pub mod cons;
pub mod env;
pub mod eval;
pub mod user;


#[macro_export]
macro_rules! eval {
    ($t: ty) => {
        <<$t as $crate::eval::Eval>::Out as $crate::expr::Expr>::to_string()
    };
}
