use crate::Complex;
use crate::Lateral;
use num::Num;
use std::ops;
use std::ops::{Add, Neg};

impl<Q> ops::Add<Complex<Q>> for Complex<Q>
where
    for<'a> Q: Num + Add<Output = Q> + Neg<Output = Q>,
{
    type Output = Complex<Q>;

    fn add(self, z: Complex<Q>) -> Complex<Q> {
        let a_bar = self.a + z.a;
        let w_bar = self.w + z.w;
        return Complex::new(a_bar, w_bar);
    }
}

impl<'a, 'b, Q> ops::Add<&'b Complex<Q>> for &'a Complex<Q>
where
    Q: Num + Add<Output = Q> + Neg<Output = Q> + Copy,
{
    type Output = Complex<Q>;

    fn add(self, z: &'b Complex<Q>) -> Complex<Q> {
        let a_bar = self.a + z.a;
        let w_bar = self.w + z.w;
        return Complex::new(a_bar, w_bar);
    }
}

impl<Q> ops::Add<Q> for Complex<Q>
where
    for<'a> Q: Num + Add<Output = Q> + Neg<Output = Q>,
{
    type Output = Complex<Q>;

    fn add(self, x: Q) -> Complex<Q> {
        let a_bar = self.a + x;
        return Complex::new(a_bar, self.w);
    }
}

impl<Q> ops::Add<Lateral<Q>> for Complex<Q>
where
    for<'a> Q: Num + Add<Output = Q> + Neg<Output = Q>,
{
    type Output = Complex<Q>;

    fn add(self, w: Lateral<Q>) -> Complex<Q> {
        let w_bar = self.w + w;
        return Complex::new(self.a, w_bar);
    }
}
