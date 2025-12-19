use crate::{interpreter::value::{Value}, register_method};

pub fn fn_print(args: Vec<Value>) -> Value {
    for arg in args.iter() {
        // Keep Value alive
        let val_str = arg.to_string(); // Value::String(Rc<str>)
        if let Value::String(rc) = val_str {
            // rc now lives as long as this iteration
            print!("{}", rc); // &Rc<str> implements Display
        } else {
            unreachable!()
        }
    }

    Value::Empty
}

register_method!("print", fn_print);