pub struct Lateral {
    pub b: f64,
}

impl Lateral {
    pub fn new(b: f64) -> Self {
        return Self { b };
    }

    pub fn to_string(&self) -> String {
        if self.b == 1.0 {
            return String::from("i");
        } else {
            return format!("{}i", self.b);
        }
    }
}
