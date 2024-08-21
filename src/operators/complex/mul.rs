use crate::Complex;
use crate::Lateral;
use std::ops;

impl ops::Mul<f64> for Complex {
    type Output = Complex;

    fn mul(self, x: f64) -> Complex {
        let a = self.a * x;
        let w = self.w * x;

        return Complex::new(a, w);
    }
}

impl ops::Mul<i32> for Complex {
    type Output = Complex;

    fn mul(self, x: i32) -> Complex {
        let xf64 = x as f64;
        let a = self.a * xf64;
        let w = self.w * xf64;

        return Complex::new(a, w);
    }
}

impl ops::Mul<Lateral> for Complex {
    type Output = Complex;
    fn mul(self, w: Lateral) -> Complex {
        let a_bar = &self.w * &w;
        let w_bar = &self.a * &w;

        return Complex::new(a_bar, w_bar);
    }
}

impl ops::Mul<Complex> for Complex {
    type Output = f64;

    fn mul(self, z: Complex) -> f64 {
        let x = self.a * z.a;
        let y = self.w * z.w;

        return x + y;
    }
}

impl<'a, 'b> ops::Mul<&'a f64> for &'b Complex {
    type Output = Complex;

    fn mul(self, x: &'a f64) -> Complex {
        let a_bar = &self.a * *x;
        let w_bar = Lateral::new(&self.w.b * *x);

        return Complex::new(a_bar, w_bar);
    }
}

impl<'a, 'b> ops::Mul<&'a i32> for &'b Complex {
    type Output = Complex;

    fn mul(self, x: &'a i32) -> Complex {
        let xf64 = *x as f64;
        let a_bar = self.a * xf64;
        let w_bar = Lateral::new(&self.w.b * *x as f64);

        return Complex::new(a_bar, w_bar);
    }
}

impl<'a, 'b> ops::Mul<&'a Lateral> for &'b Complex {
    type Output = Complex;

    fn mul(self, w: &'a Lateral) -> Complex {
        let a_bar = &self.w * w;
        let w_bar = &self.a * w;

        return Complex::new(a_bar, w_bar);
    }
}

impl<'a, 'b> ops::Mul<&'a Complex> for &'b Complex {
    type Output = f64;

    fn mul(self, z: &'a Complex) -> f64 {
        let x = &self.a * z.a;
        let y = &self.w * &z.w;

        return x + y;
    }
}
