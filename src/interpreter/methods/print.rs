use crate::{interpreter::value::{Convert, Value}, register_method};

pub fn fn_print(args: Vec<Value>) -> Value {
    args.iter().for_each(|arg| {
        let str = String::convert(arg.to_string()).unwrap();
        print!("{}", str);
    });

    Value::Empty
}

register_method!("print", fn_print);