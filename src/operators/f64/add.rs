use crate::Complex;
use crate::Lateral;
use std::ops;

impl ops::Add<Complex> for f64 {
    type Output = Complex;

    fn add(self, z: Complex) -> Complex {
        let a_bar = self + z.a;

        return Complex::new(a_bar, z.w);
    }
}

impl<'a, 'b> ops::Add<&'a Complex> for &'b f64 {
    type Output = Complex;

    fn add(self, z: &'a Complex) -> Complex {
        let a_bar = self + z.a;
        let w_bar = Lateral::new(z.w.b);
        return Complex::new(a_bar, w_bar);
    }
}
