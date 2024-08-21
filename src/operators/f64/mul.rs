use crate::Complex;
use crate::Lateral;
use std::ops;

impl ops::Mul<Lateral> for f64 {
    type Output = Lateral;

    fn mul(self, w: Lateral) -> Lateral {
        return Lateral::new(self * w.b);
    }
}

impl ops::Mul<Complex> for f64 {
    type Output = Complex;

    fn mul(self, z: Complex) -> Complex {
        let a_bar = self * z.a;
        let w_bar = self * z.w;
        return Complex::new(a_bar, w_bar);
    }
}

impl<'a, 'b> ops::Mul<&'a Lateral> for &'b f64 {
    type Output = Lateral;

    fn mul(self, w: &'a Lateral) -> Lateral {
        return Lateral::new(*self * w.b);
    }
}

impl<'a, 'b> ops::Mul<&'a Complex> for &'b f64 {
    type Output = Complex;

    fn mul(self, z: &'a Complex) -> Complex {
        let a_bar = self * z.a;
        let w_bar = Lateral::new(*self * z.w.b);

        return Complex::new(a_bar, w_bar);
    }
}
