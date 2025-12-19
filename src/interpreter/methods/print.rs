use crate::{interpreter::{runtime_errors::RuntimeError, value::Value}, register_method};

pub fn fn_print(args: Vec<Value>) -> Result<Value, RuntimeError> {
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

    Ok(Value::Empty)
}

register_method!("print", fn_print);
