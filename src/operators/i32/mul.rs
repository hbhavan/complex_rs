use crate::Complex;
use crate::Lateral;
use std::ops;

impl ops::Mul<Lateral> for i32 {
    type Output = Lateral;

    fn mul(self, w: Lateral) -> Lateral {
        return Lateral::new(w.b * self as f64);
    }
}

impl ops::Mul<Complex> for i32 {
    type Output = Complex;

    fn mul(self, z: Complex) -> Complex {
        return z * self;
    }
}

impl<'a, 'b> ops::Mul<&'a Lateral> for &'b i32 {
    type Output = Lateral;

    fn mul(self, w: &Lateral) -> Lateral {
        return Lateral::new(w.b * *self as f64);
    }
}

impl<'a, 'b> ops::Mul<&'a Complex> for &'b i32 {
    type Output = Complex;

    fn mul(self, z: &'a Complex) -> Complex {
        let a_bar = *self as f64 * z.a;
        let w_bar = Lateral::new(*self as f64 * z.w.b);

        return Complex::new(a_bar, w_bar);
    }
}
