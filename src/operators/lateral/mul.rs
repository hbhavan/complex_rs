use crate::Complex;
use crate::Lateral;
use num::Num;
use std::ops;
use std::ops::{Mul, Neg};

impl<Q> ops::Mul<Lateral<Q>> for Lateral<Q>
where
    for<'a> Q: Num + Mul<Output = Q> + Neg<Output = Q>,
{
    type Output = Q;

    fn mul(self, w: Lateral<Q>) -> Q {
        let b = self.b * w.b;
        return Neg::neg(b);
    }
}

impl<'a, 'b, Q> ops::Mul<&'b Lateral<Q>> for &'a Lateral<Q>
where
    Q: Num + Mul<Output = Q> + Neg<Output = Q> + Copy,
{
    type Output = Q;

    fn mul(self, w: &'b Lateral<Q>) -> Q {
        let b = self.b * w.b;
        return Neg::neg(b);
    }
}

impl<'a, Q> ops::Mul<Q> for Lateral<Q>
where
    Q: Num + Mul<Output = Q> + Neg<Output = Q>,
{
    type Output = Lateral<Q>;

    fn mul(self, x: Q) -> Lateral<Q> {
        let b = self.b * x;
        return Lateral::new(b);
    }
}

impl<'a, 'b, Q> ops::Mul<&'b Q> for &'a Lateral<Q>
where
    Q: Num + Mul<Output = Q> + Neg<Output = Q> + Copy,
{
    type Output = Lateral<Q>;

    fn mul(self, x: &'b Q) -> Lateral<Q> {
        let b = self.b * *x;
        return Lateral::new(b);
    }
}
