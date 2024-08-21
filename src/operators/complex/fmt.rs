use crate::Complex;
use core::fmt;

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let result = if self.w.b == 0.0 {
            self.a.to_string()
        } else {
            format!("{} + {}", self.a, self.w.to_string())
        };

        return write!(f, "{}", result);
    }
}
