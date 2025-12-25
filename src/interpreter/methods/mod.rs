mod math;
mod print;
mod println;
mod readln;
mod string;

use std::rc::Rc;

use super::{runtime_errors::RuntimeError, value::Value};

pub type NativeFn = fn(Vec<Rc<Value>>) -> Result<Rc<Value>, RuntimeError>;

pub struct Method {
    pub name: &'static str,
    pub func: NativeFn,
}

inventory::collect!(Method);

type NativeFnArgs = Vec<Rc<Value>>;
type NativeFnReturn = Rc<Value>;

pub fn get_method(name: String, args: NativeFnArgs) -> Result<NativeFnReturn, RuntimeError> {
    for method in inventory::iter::<Method> {
        if method.name == name {
            return (method.func)(args);
        }
    }

    Err(RuntimeError::new(format!("Method not found: {}", name)))
}

#[macro_export]
macro_rules! takes_arguments {
    ($args:expr, 0) => {{
        if $args.len() != 0 {
            return Err(RuntimeError::new(format!(
                "Expected 0 parameters, found {}",
                $args.len()
            )));
        }
        Ok(())
    }};
    ($args:expr, 1) => {{
        if $args.len() != 1 {
            return Err(RuntimeError::new(format!(
                "Expected 1 parameter, found {}",
                $args.len()
            )));
        }
        Ok(($args[0].clone(),))
    }};
    ($args:expr, 2) => {{
        if $args.len() != 2 {
            return Err(RuntimeError::new(format!(
                "Expected 2 parameters, found {}",
                $args.len()
            )));
        }
        Ok(($args[0].clone(), $args[1].clone()))
    }};
    ($args:expr, 3) => {{
        if $args.len() != 3 {
            return Err(RuntimeError::new(format!(
                "Expected 3 parameters, found {}",
                $args.len()
            )));
        }
        Ok(($args[0].clone(), $args[1].clone(), $args[2].clone()))
    }};
    ($args:expr, 4) => {{
        if $args.len() != 4 {
            return Err(RuntimeError::new(format!(
                "Expected 4 parameters, found {}",
                $args.len()
            )));
        }
        Ok(($args[0].clone(), $args[1].clone(), $args[2].clone(), $args[3].clone()))
    }};
    ($args:expr, 5) => {{
        if $args.len() != 5 {
            return Err(RuntimeError::new(format!(
                "Expected 5 parameters, found {}",
                $args.len()
            )));
        }
        Ok((
            $args[0].clone(),
            $args[1].clone(),
            $args[2].clone(),
            $args[3].clone(),
            $args[4].clone(),
        ))
    }};
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
