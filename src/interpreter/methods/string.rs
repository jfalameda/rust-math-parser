use crate::{
    error::error,
    interpreter::{methods::{NativeFnArgs, NativeFnReturn}, runtime_errors::RuntimeError, value::Value},
    register_method, takes_arguments,
};

use std::rc::Rc;

/// Concatenate multiple Values into a single string
pub fn fn_str_concat(args: NativeFnArgs) -> Result<NativeFnReturn, RuntimeError> {
    let mut concat_str = String::new();

    for arg in args.iter() {
        // Force JS-style string coercion
        let val_str = arg.to_string(); // Value::String(Rc<str>)
        if let Value::String(rc) = val_str {
            concat_str.push_str(&rc); // Rc<str> derefs to &str
        } else {
            unreachable!(); // to_string() always returns Value::String
        }
    }

    Ok(Value::String(Rc::from(concat_str)).into_rc())
}

/// Convert a Value to a numeric Value (Integer or Float)
pub fn fn_to_number(args: NativeFnArgs) -> Result<NativeFnReturn, RuntimeError> {
    let (value,) = takes_arguments!(args, 1)?;

    let result = match value.as_ref() {
        Value::String(rc) => {
            let s = rc.as_ref(); // &str from Rc<str>

            if s.is_empty() {
                Value::Integer(0) // or decide on other behavior for empty string
            } else if let Ok(i) = s.parse::<i64>() {
                Value::Integer(i)
            } else if let Ok(f) = s.parse::<f64>() {
                Value::Float(f)
            } else {
                error(&format!("Cannot convert string '{}' to number", s))
            }
        }
        _ => value.to_number(), // other types use existing coercion
    };

    Ok(result.into_rc())
}

register_method!("str_concat", fn_str_concat);
register_method!("to_number", fn_to_number);
