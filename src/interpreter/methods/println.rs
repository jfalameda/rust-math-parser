use crate::{interpreter::{runtime_errors::RuntimeError, value::Value}, register_method};

pub fn fn_println(args: Vec<Value>) -> Result<Value, RuntimeError> {
    for arg in args.iter() {
        // Force the Value into a Value::String
        let val_str = arg.to_string(); // returns Value::String(Rc<str>)
        if let Value::String(rc) = val_str {
            // Rc<str> derefs to str, so print works directly
            print!("{}", rc);
        } else {
            unreachable!(); // to_string() always returns Value::String
        }
    }
    println!();

    Ok(Value::Empty)
}
register_method!("println", fn_println);
