use crate::{error, interpreter::value::{Convert, Value}};

pub fn fn_str_concat(args: Vec<Value>) -> Value {
    let mut concat_str = String::from("");

    args.iter().for_each(|arg| {
        let str = String::convert(arg.to_string()).unwrap();
        concat_str.push_str(&str);
    });

    return Value::String(concat_str);
}

pub fn fn_to_number(args: Vec<Value>) -> Value {
    let value = args.get(0).unwrap();

    match value {
        Value::String(_) => {
            // TODO: Check edge cases, what do to with an empty string?
            // What if it cannot convert to number?
            let s = match value {
                Value::String(s) => s,
                _ => "",
            };
            if let Ok(i) = s.parse::<i64>() {
                Value::Integer(i)
            } else if let Ok(f) = s.parse::<f64>() {
                Value::Float(f)
            } else {
                // TODO: Proper runtine error handling
                error(format!("Cannot convert string '{}' to number", s).as_str())
            }
        }
        _ => value.to_number(), // other types use existing coercion
    }
}

