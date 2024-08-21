use crate::Lateral;
use core::fmt;

impl fmt::Display for Lateral {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let result = if self.b == 1.0 {
            String::from("i")
        } else {
            format!("{}i", self.b)
        };

        return write!(f, "{}", result);
    }
}
