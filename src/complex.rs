use num::Num;

use crate::Lateral;
use std::ops::{Add, Mul, Neg, Sub};

pub struct Complex<Q> {
    pub a: Q,
    pub w: Lateral<Q>,
}

impl<Q> Complex<Q>
where
    for<'a> Q: Num + Add<Output = Q> + Mul<Output = Q> + Sub<Output = Q> + Neg<Output = Q>,
{
    pub fn new(a: Q, w: Lateral<Q>) -> Self {
        return Self { a, w };
    }
}
