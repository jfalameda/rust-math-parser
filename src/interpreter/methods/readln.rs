use std::io::{stdout, Write, self, BufRead};

use crate::interpreter::value::{Value, Convert};

pub fn fn_readln(args: Vec<Value>) -> Value {
    args.iter().for_each(|arg| {
        let str = String::convert(arg.to_string()).unwrap();
        print!("{}", str);
    });

    // TODO: Proper runtime error handling
    stdout().flush()
        .expect("Unable to flush");

    let mut line = String::new();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut line).unwrap();

    // Remove last character
    line.pop();
    
    Value::String(line)
}