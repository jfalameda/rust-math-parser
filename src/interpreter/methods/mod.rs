mod println;
mod print;
mod readln;
mod string;
mod math;

use crate::error;

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