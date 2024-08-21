use crate::Lateral;

pub struct Complex {
    pub a: f64,
    pub w: Lateral,
}

impl Complex {
    pub fn new(a: f64, w: Lateral) -> Self {
        return Self { a, w };
    }

    pub fn to_string(&self) -> String {
        if self.w.b == 0.0 {
            return self.a.to_string();
        } else {
            return format!("{} + {}", self.a, self.w.to_string());
        }
    }
}
