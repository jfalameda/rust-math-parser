use crate::interpreter::value::{Value, Convert};

pub fn fn_sin(args: Vec<Value>) -> Value {
    let number = args.get(0).unwrap();
    let number = f32::convert(number.to_number()).unwrap();
    return Value::Float(f32::sin(number));
}

pub fn fn_cos(args: Vec<Value>) -> Value {
    let number = args.get(0).unwrap();
    let number = f32::convert(number.to_number()).unwrap();
    return Value::Float(f32::cos(number));
}