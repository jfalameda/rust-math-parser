mod println;
mod print;
mod readln;
mod string;
mod math;

use super::{runtime_errors::RuntimeError, value::Value};

pub type NativeFn = fn(Vec<Value>) -> Result<Value, RuntimeError>;

pub struct Method {
    pub name: &'static str,
    pub func: NativeFn,
}

inventory::collect!(Method);

pub fn get_method(name: String, args: Vec<Value>) -> Result<Value, RuntimeError> {
    for method in inventory::iter::<Method> {
        if method.name == name {
            return (method.func)(args);
        }
    }

    Err(RuntimeError::new(format!("Method not found: {}", name)))
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