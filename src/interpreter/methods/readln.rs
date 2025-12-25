use std::{
    io::{self, stdout, BufRead, Write},
    rc::Rc,
};

use crate::{
    interpreter::{methods::{NativeFnArgs, NativeFnReturn}, runtime_errors::RuntimeError, value::Value},
    register_method,
};

pub fn fn_readln(args: NativeFnArgs) -> Result<NativeFnReturn, RuntimeError> {
    // Print all arguments without converting to String::convert
    for arg in args.iter() {
        let val_str = arg.to_string();
        if let Value::String(rc) = val_str {
            print!("{}", rc);
        } else {
            unreachable!();
        }
    }

    // Flush stdout
    stdout().flush().expect("Unable to flush");

    // Read line from stdin
    let mut line = String::new();
    let stdin = io::stdin();

    stdin.lock().read_line(&mut line)
        .map_err(|err| { RuntimeError::new(format!("Unable to read line: {}", err.to_string())) })?;

    // Remove trailing newline
    if line.ends_with('\n') {
        line.pop();
        if line.ends_with('\r') {
            // handle Windows CRLF
            line.pop();
        }
    }

    Ok(Value::String(Rc::from(line)).into_rc())
}

register_method!("readln", fn_readln);
