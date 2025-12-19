use crate::{interpreter::value::{Convert, Value}, register_method};

pub fn fn_println(args: Vec<Value>) -> Value {
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

    Value::Empty
}
register_method!("println", fn_println);