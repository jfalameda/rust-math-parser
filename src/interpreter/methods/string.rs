use crate::interpreter::value::{Convert, Value};

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
    value.convert_to_number()
}

