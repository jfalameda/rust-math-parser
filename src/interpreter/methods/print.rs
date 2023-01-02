use crate::interpreter::value::{Value, Convert};

pub fn fn_print(args: Vec<Value>) -> Value {
    args.iter().for_each(|arg| {
        let str = String::convert(arg.to_string()).unwrap();
        print!("{}", str);
    });

    Value::Empty
}