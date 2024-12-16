use std::ops;
use std::ops::{Add, Mul, Sub};

pub struct Lateral<Q> {
    pub b: Q,
}

impl<Q> Lateral<Q>
where
    for<'a> &'a Q: Add<Output = Q> + Sub<Output = Q> + Mul<Output = Q> + TryInto<Q>,
{
    pub fn new(b: Q) -> Self {
        return Self { b };
    }
}

impl<Q> ops::Add<&Lateral<Q>> for &Lateral<Q>
where
    for<'a> &'a Q: Add<Output = Q> + Sub<Output = Q> + Mul<Output = Q>,
{
    type Output = Lateral<Q>;

    fn add(self, w: &Lateral<Q>) -> Lateral<Q> {
        return Lateral::new(self.b + w.b);
    }
}
