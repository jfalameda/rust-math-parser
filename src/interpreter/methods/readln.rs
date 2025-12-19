use std::{
    io::{self, stdout, BufRead, Write},
    rc::Rc,
};

use crate::{
    interpreter::{runtime_errors::RuntimeError, value::Value},
    register_method,
};

pub fn fn_readln(args: Vec<Value>) -> Result<Value, RuntimeError> {
    // Print all arguments without converting to String::convert
    for arg in args.iter() {
        let val_str = arg.to_string(); // returns Value::String(Rc<str>)
        if let Value::String(rc) = val_str {
            print!("{}", rc); // Rc<str> derefs to str
        } else {
            unreachable!(); // to_string() always returns Value::String
        }
    }

    // Flush stdout
    stdout().flush().expect("Unable to flush");

    // Read line from stdin
    let mut line = String::new();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut line).unwrap();

    // Remove trailing newline
    if line.ends_with('\n') {
        line.pop();
        if line.ends_with('\r') {
            // handle Windows CRLF
            line.pop();
        }
    }

    // Return as Value::String(Rc<str>)
    Ok(Value::String(Rc::from(line)))
}

register_method!("readln", fn_readln);
