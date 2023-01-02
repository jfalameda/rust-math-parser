mod println;
mod print;
mod readln;
mod string;
mod math;

use self::math::{fn_sin, fn_cos};
use self::println::fn_println;
use self::print::fn_print;
use self::readln::fn_readln;
use self::string::{fn_str_concat, fn_to_number};

use super::value::Value;


pub fn get_method(method_name: String, args: Vec<Value>) -> Value {
    match method_name.as_str() {
        "println" => fn_println(args),
        "print" => fn_print(args),
        "readln" => fn_readln(args),
        "str_concat" => fn_str_concat(args),
        "to_number" => fn_to_number(args),
        "sin" => fn_sin(args),
        "cos" => fn_cos(args),
        _ => panic!("Method not found: {}", method_name)
    }
}