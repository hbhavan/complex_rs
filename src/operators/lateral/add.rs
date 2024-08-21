use crate::Complex;
use crate::Lateral;
use std::ops;

impl ops::Add<Lateral> for Lateral {
    type Output = Lateral;

    fn add(self, w: Lateral) -> Lateral {
        return Lateral::new(self.b + w.b);
    }
}

impl ops::Add<Complex> for Lateral {
    type Output = Complex;

    fn add(self, z: Complex) -> Complex {
        let w_bar = Lateral::new(&self.b + &z.w.b);
        return Complex::new(z.a, w_bar);
    }
}

impl<'a, 'b> ops::Add<&'a Lateral> for &'b Lateral {
    type Output = Lateral;

    fn add(self, w: &'a Lateral) -> Lateral {
        return Lateral::new(self.b + w.b);
    }
}

impl<'a, 'b> ops::Add<&'a Complex> for &'b Lateral {
    type Output = Complex;

    fn add(self, z: &'a Complex) -> Complex {
        let w_bar = Lateral::new(self.b + z.w.b);
        return Complex::new(z.a, w_bar);
    }
}
