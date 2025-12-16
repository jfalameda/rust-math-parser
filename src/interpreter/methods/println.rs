use crate::{interpreter::value::{Convert, Value}, register_method};

pub fn fn_println(args: Vec<Value>) -> Value {
    args.iter().for_each(|arg| {
        let str = String::convert(arg.to_string()).unwrap();
        print!("{}", str);
    });
    println!("");

    Value::Empty
}

register_method!("println", fn_println);