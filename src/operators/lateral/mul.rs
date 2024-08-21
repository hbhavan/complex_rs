use crate::Complex;
use crate::Lateral;
use std::ops;

impl ops::Mul<f64> for Lateral {
    type Output = Lateral;

    fn mul(self, x: f64) -> Lateral {
        return Lateral::new(self.b * x);
    }
}

impl ops::Mul<i32> for Lateral {
    type Output = Lateral;

    fn mul(self, x: i32) -> Lateral {
        return Lateral::new(self.b * x as f64);
    }
}

impl ops::Mul<Lateral> for Lateral {
    type Output = f64;

    fn mul(self, w: Lateral) -> f64 {
        return self.b * w.b * -1.0;
    }
}

impl ops::Mul<Complex> for Lateral {
    type Output = Complex;

    fn mul(self, z: Complex) -> Complex {
        let a_bar = &self * &z.w;
        let w_bar = &self * &z.a;

        return Complex::new(a_bar, w_bar);
    }
}

impl<'a, 'b> ops::Mul<&'a f64> for &'b Lateral {
    type Output = Lateral;

    fn mul(self, x: &'a f64) -> Lateral {
        return Lateral::new(self.b * *x);
    }
}

impl<'a, 'b> ops::Mul<&'a i32> for &'b Lateral {
    type Output = Lateral;

    fn mul(self, x: &'a i32) -> Lateral {
        return Lateral::new(self.b * *x as f64);
    }
}

impl<'a, 'b> ops::Mul<&'a Lateral> for &'b Lateral {
    type Output = f64;

    fn mul(self, w: &'a Lateral) -> f64 {
        return self.b * w.b * -1.0;
    }
}

impl<'a, 'b> ops::Mul<&'a Complex> for &'b Lateral {
    type Output = Complex;

    fn mul(self, z: &'a Complex) -> Complex {
        let a_bar = self * &z.w;
        let w_bar = self * &z.a;

        return Complex::new(a_bar, w_bar);
    }
}
