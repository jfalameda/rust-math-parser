use std::rc::Rc;
use crate::{error, interpreter::value::{Value}, register_method};


/// Concatenate multiple Values into a single string
pub fn fn_str_concat(args: Vec<Value>) -> Value {
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

    Value::String(Rc::from(concat_str))
}

/// Convert a Value to a numeric Value (Integer or Float)
pub fn fn_to_number(args: Vec<Value>) -> Value {
    let value = args.get(0).expect("fn_to_number requires at least one argument");

    match value {
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
    }
}


register_method!("str_concat", fn_str_concat);
register_method!("to_number", fn_to_number);
