use crate::Lateral;
use core::fmt;
use core::fmt::Display;
use num::{one, Num, One};

impl<Q> fmt::Display for Lateral<Q>
where
    for<'a> Q: Num + Display + One,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.b == one() {
            return write!(f, "i");
        } else {
            return write!(f, "{}i", self.b.to_string());
        }
    }
}
