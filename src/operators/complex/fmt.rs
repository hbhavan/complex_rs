use crate::Complex;
use crate::Lateral;
use core::fmt;
use core::fmt::Display;
use num::{zero, Num, One, Zero};

impl<Q> fmt::Display for Complex<Q>
where
    for<'a> Q: Num + Display + One + Zero,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let result = match (self.a == zero(), self.w.b == zero()) {
            (true, true) => String::from("0"),
            (true, false) => self.w.to_string(),
            (false, true) => self.a.to_string(),
            (_, _) => format!("{} + {}", self.a.to_string(), self.w.to_string()),
        };

        return write!(f, "{}", result);
    }
}
