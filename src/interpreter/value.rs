use std::ops;

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Float(f32),
    String(String),
    Empty
}

impl Value {
    pub fn to_string(&self) -> Value {
        match self {
            Value::Float(f) => Value::String(f.to_string()),
            Value::String(s) => Value::String(s.to_string()),
            Value::Empty => Value::String("".to_string())
        }
    }

    pub fn to_number(&self) -> Value {
        match self {
            Value::Float(_) => self.clone(),
            _ => panic!("Expected number value, found incompatible type.")
        }
    }

    pub fn convert_to_number(&self) -> Value {
        let val = String::convert(self.clone().to_string()).unwrap();

        let float  = val.parse::<f32>();

        if float.is_err() {
            panic!("Unable to convert to number")
        }

        return Value::Float(float.unwrap());
    }

    pub fn power(&self, right: Value) -> Value {
        let left = f32::convert(self.to_number()).unwrap();
        let right = f32::convert(right.to_number()).unwrap();

        return Value::Float(left.powf(right));
    }

}

impl ops::Add<Value> for Value {
    type Output = Value;

    fn add(self, right: Value) -> Value {

        let left = f32::convert(self.to_number()).unwrap();
        let right = f32::convert(right.to_number()).unwrap();

        return Value::Float(left + right);
    }
}

impl ops::Sub<Value> for Value {
    type Output = Value;

    fn sub(self, right: Value) -> Value {

        let left = f32::convert(self.to_number()).unwrap();
        let right = f32::convert(right.to_number()).unwrap();

        return Value::Float(left - right);
    }
}

impl ops::Mul<Value> for Value {
    type Output = Value;

    fn mul(self, right: Value) -> Value {

        let left = f32::convert(self.to_number()).unwrap();
        let right = f32::convert(right.to_number()).unwrap();

        return Value::Float(left * right);
    }
}

impl ops::Div<Value> for Value {
    type Output = Value;

    fn div(self, right: Value) -> Value {

        let left = f32::convert(self.to_number()).unwrap();
        let right = f32::convert(right.to_number()).unwrap();

        return Value::Float(left / right);
    }
}

pub trait Convert : Sized {
    fn convert(v: Value) -> Option<Self>;
}


macro_rules! impl_convert {
    ($t:ty, $id:ident) => (
        impl Convert for $t {
            fn convert(v: Value) -> Option<$t> {
                match v {
                    Value::$id(x) => Some(x),
                    _ => None,
                }
            }
        }
    )
}

impl_convert!(f32, Float);
impl_convert!(String, String);

