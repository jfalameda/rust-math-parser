use std::rc::Rc;

use crate::{
    interpreter::{
        methods::{NativeFnArgs, NativeFnReturn}, runtime_errors::RuntimeError, value::{Convert, Value}
    },
    register_method, takes_arguments,
};

pub fn fn_sin(args: NativeFnArgs) -> Result<NativeFnReturn, RuntimeError> {
    let (angle,) = takes_arguments!(args, 1)?;

    let number = f64::convert(angle.to_number()).unwrap();
    
    Ok(Rc::new(Value::Float(f64::sin(number))))
}

pub fn fn_cos(args: NativeFnArgs) -> Result<NativeFnReturn, RuntimeError> {
    let (angle,) = takes_arguments!(args, 1)?;

    // Convert anything to f64 using your existing logic
    // TODO: Implement proper runtime error handing
    let number = angle.to_f64();

    Ok(Rc::new(Value::Float(number.cos())))
}

register_method!("sin", fn_sin);
register_method!("cos", fn_cos);
