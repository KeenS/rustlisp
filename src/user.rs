use expr::*;
use symbol::*;
use fun::*;
use num::*;
use eval::*;

pub type Fib = Symbol<Sym3<F, I, B>>;

impl Fun1<_0> for Fib {
    type Out = _1;
}

impl Fun1<_1> for Fib {
    type Out = _1;
}

impl <T1N: Num, N1_O: Num, N2_O: Num, NO: Num>Fun1<Number<Succ<Succ<T1N>>>> for Fib
    where Fib: Fun1<Number<Succ<T1N>>, Out = Number<N1_O>>,
          Fib: Fun1<Number<     T1N >, Out = Number<N2_O>>,
          Add: Fun2<Number<N1_O>, Number<N2_O>, Out = Number<NO>>
{
    type Out = Number<NO>;
}
