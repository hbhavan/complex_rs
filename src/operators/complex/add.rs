use crate::Complex;
use crate::Lateral;
use std::ops;

impl ops::Add<f64> for Complex {
    type Output = Complex;

    fn add(self, x: f64) -> Complex {
        let a_bar = self.a + x;
        return Complex::new(a_bar, self.w);
    }
}

impl ops::Add<i32> for Complex {
    type Output = Complex;

    fn add(self, x: i32) -> Complex {
        let a = self.a + x as f64;
        return Complex::new(a, self.w);
    }
}

impl ops::Add<Lateral> for Complex {
    type Output = Complex;

    fn add(self, w: Lateral) -> Complex {
        let w_bar = self.w + w;
        return Complex::new(self.a, w_bar);
    }
}

impl ops::Add<Complex> for Complex {
    type Output = Complex;

    fn add(self, z: Complex) -> Complex {
        let a_bar = self.a + z.a;
        let w_bar = self.w + z.w;
        return Complex::new(a_bar, w_bar);
    }
}

impl<'a, 'b> ops::Add<&'a f64> for &'b Complex {
    type Output = Complex;

    fn add(self, x: &'a f64) -> Complex {
        let a_bar = self.a + x;
        let w_bar = Lateral::new(self.w.b);
        return Complex::new(a_bar, w_bar);
    }
}

impl<'a, 'b> ops::Add<&'a i32> for &'b Complex {
    type Output = Complex;

    fn add(self, x: &'a i32) -> Complex {
        let a_bar = self.a + *x as f64;
        let w_bar = Lateral::new(self.w.b);
        return Complex::new(a_bar, w_bar);
    }
}

impl<'a, 'b> ops::Add<&'a Lateral> for &'b Complex {
    type Output = Complex;

    fn add(self, w: &'a Lateral) -> Complex {
        let w_bar = &self.w + w;
        return Complex::new(self.a, w_bar);
    }
}

impl<'a, 'b> ops::Add<&'a Complex> for &'b Complex {
    type Output = Complex;

    fn add(self, z: &'a Complex) -> Complex {
        let a_bar = self.a + z.a;
        let w_bar = Lateral::new(self.w.b * z.w.b);
        return Complex::new(a_bar, w_bar);
    }
}
