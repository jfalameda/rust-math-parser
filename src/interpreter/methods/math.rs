use crate::interpreter::value::{Value, Convert};

pub fn fn_sin(args: Vec<Value>) -> Value {
    let number = args.get(0).unwrap();
    let number = f64::convert(number.to_number()).unwrap();
    return Value::Float(f64::sin(number));
}

pub fn fn_cos(args: Vec<Value>) -> Value {
    let value = args.get(0).unwrap();

    // Convert anything to f64 using your existing logic
    // TODO: Implement proper runtime error handing
    let number = value.to_f64();

    Value::Float(number.cos())
}