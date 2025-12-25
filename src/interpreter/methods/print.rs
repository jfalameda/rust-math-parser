use crate::{
    interpreter::{methods::{NativeFnArgs, NativeFnReturn}, runtime_errors::RuntimeError, value::Value},
    register_method,
};

pub fn fn_print(args: NativeFnArgs) -> Result<NativeFnReturn, RuntimeError> {
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

    Ok(Value::Empty.into_rc())
}

register_method!("print", fn_print);
