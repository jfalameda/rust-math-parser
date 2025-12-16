mod println;
mod print;
mod readln;
mod string;
mod math;

use crate::error;
use crate::interpreter::methods::string::fn_to_number;

use self::math::{fn_sin, fn_cos};
use self::println::fn_println;
use self::print::fn_print;
use self::readln::fn_readln;
use self::string::{fn_str_concat};

use super::value::Value;

pub type NativeFn = fn(Vec<Value>) -> Value;

pub struct Method {
    pub name: &'static str,
    pub func: NativeFn,
}

inventory::collect!(Method);

pub fn get_method(name: String, args: Vec<Value>) -> Value {
    for method in inventory::iter::<Method> {
        if method.name == name {
            return (method.func)(args);
        }
    }

    error(format!("Method not found: {}", name).as_str())
}

#[macro_export]
macro_rules! register_method {
    ($name:expr, $func:path) => {
        inventory::submit! {
            $crate::interpreter::methods::Method {
                name: $name,
                func: $func,
            }
        }
    };
}