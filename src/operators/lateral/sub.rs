use crate::Complex;
use crate::Lateral;
use num::Num;
use std::ops;
use std::ops::{Neg, Sub};

impl<Q> ops::Sub<Lateral<Q>> for Lateral<Q>
where
    for<'a> Q: Num + Sub<Output = Q> + Neg<Output = Q>,
{
    type Output = Lateral<Q>;

    fn sub(self, w: Lateral<Q>) -> Lateral<Q> {
        let b = self.b - w.b;
        return Lateral::new(b);
    }
}

impl<'a, 'b, Q> ops::Sub<&'b Lateral<Q>> for &'a Lateral<Q>
where
    Q: Num + Sub<Output = Q> + Neg<Output = Q> + Copy,
{
    type Output = Lateral<Q>;

    fn sub(self, w: &'b Lateral<Q>) -> Lateral<Q> {
        let b = self.b - w.b;
        return Lateral::new(b);
    }
}
