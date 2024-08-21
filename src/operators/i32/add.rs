use crate::Complex;
use crate::Lateral;
use std::ops;

impl ops::Add<Complex> for i32 {
    type Output = Complex;

    fn add(self, z: Complex) -> Complex {
        return z + self;
    }
}

impl<'a, 'b> ops::Add<&'a Complex> for &'b i32 {
    type Output = Complex;

    fn add(self, z: &'a Complex) -> Complex {
        let a_bar = *self as f64 + z.a;
        let w_bar = Lateral::new(z.w.b);
        return Complex::new(a_bar, w_bar);
    }
}
