use crate::{
    interpreter::{
        methods::{NativeFnArgs, NativeFnReturn}, runtime_errors::RuntimeError, value::{Value}
    },
    register_method, takes_arguments,
};

pub fn fn_sin(args: NativeFnArgs) -> Result<NativeFnReturn, RuntimeError> {
    let (angle,) = takes_arguments!(args, 1)?;

    let number = angle.to_f64()?;
    
    Ok(Value::Float(f64::sin(number)).into_rc())
}

pub fn fn_cos(args: NativeFnArgs) -> Result<NativeFnReturn, RuntimeError> {
    let (angle,) = takes_arguments!(args, 1)?;

    // Convert anything to f64 using your existing logic
    // TODO: Implement proper runtime error handing
    let number = angle.to_f64()?;

    Ok(Value::Float(number.cos()).into_rc())
}

register_method!("sin", fn_sin);
register_method!("cos", fn_cos);
