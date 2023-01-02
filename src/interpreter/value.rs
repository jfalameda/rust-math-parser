#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Float(f32),
    String(String),
    Empty
}

impl Value {
    pub fn to_string(&self) -> String {
        match self {
            Value::Float(f) => f.to_string(),
            Value::String(s) => s.to_string(),
            Value::Empty => "".to_string()
        }
    }

    pub fn to_number(&self) -> f32 {
        match self {
            Value::Float(f) => *f,
            _ => panic!("Expected number value, found incompatible type.")
        }
    }
}
