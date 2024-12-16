use crate::Complex;
use crate::Lateral;
use num::Num;
use std::ops;
use std::ops::{Add, Neg};

impl<Q> ops::Add<Lateral<Q>> for Lateral<Q>
where
    for<'a> Q: Num + Add<Output = Q> + Neg<Output = Q>,
{
    type Output = Lateral<Q>;

    fn add(self, w: Lateral<Q>) -> Lateral<Q> {
        let b = self.b + w.b;
        return Lateral::new(b);
    }
}

impl<'a, 'b, Q> ops::Add<&'b Lateral<Q>> for &'a Lateral<Q>
where
    Q: Num + Add<Output = Q> + Neg<Output = Q> + Copy,
{
    type Output = Lateral<Q>;

    fn add(self, w: &'b Lateral<Q>) -> Lateral<Q> {
        let b = self.b + w.b;
        return Lateral::new(b);
    }
}
