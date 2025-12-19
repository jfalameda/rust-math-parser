use crate::{interpreter::{runtime_errors::RuntimeError, value::{Convert, Value}}, register_method};

pub fn fn_sin(args: Vec<Value>) -> Result<Value, RuntimeError> {
    let number = args.get(0).unwrap();
    let number = f64::convert(number.to_number()).unwrap();
    Ok(Value::Float(f64::sin(number)))
}

pub fn fn_cos(args: Vec<Value>) -> Result<Value, RuntimeError> {
    let value = args.get(0).unwrap();

    // Convert anything to f64 using your existing logic
    // TODO: Implement proper runtime error handing
    let number = value.to_f64();

    Ok(Value::Float(number.cos()))
}

register_method!("sin", fn_sin);
register_method!("cos", fn_cos);
