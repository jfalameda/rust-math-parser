use std::rc::Rc;

use crate::{
    interpreter::{
        runtime_errors::RuntimeError,
        value::{Convert, Value},
    },
    register_method,
};

pub fn fn_sin(args: Vec<Rc<Value>>) -> Result<Rc<Value>, RuntimeError> {
    let number = args.first().unwrap();
    let number = f64::convert(number.to_number()).unwrap();
    Ok(Rc::new(Value::Float(f64::sin(number))))
}

pub fn fn_cos(args: Vec<Rc<Value>>) -> Result<Rc<Value>, RuntimeError> {
    let value = args.first().unwrap();

    // Convert anything to f64 using your existing logic
    // TODO: Implement proper runtime error handing
    let number = value.to_f64();

    Ok(Rc::new(Value::Float(number.cos())))
}

register_method!("sin", fn_sin);
register_method!("cos", fn_cos);
