use std::cell::RefCell;

use parser::{
    interpreter::{runtime_errors::RuntimeError, value::Value},
    register_method,
};

#[derive(Clone, Debug)]
pub struct AssertionRecord {
    pub message: String,
    pub passed: bool,
}

thread_local! {
    static ASSERT_LOG: RefCell<Vec<AssertionRecord>> = RefCell::new(Vec::new());
}

pub fn reset_assertions() {
    ASSERT_LOG.with(|log| log.borrow_mut().clear());
}

pub fn take_assertions() -> Vec<AssertionRecord> {
    ASSERT_LOG.with(|log| log.borrow_mut().drain(..).collect())
}

fn fn_assert(args: Vec<Value>) -> Result<Value, RuntimeError> {
    if args.len() != 2 {
        return Err(RuntimeError::new(format!(
            "assert expects 2 arguments, got {}",
            args.len()
        )));
    }

    let message_value = args.get(0).unwrap().to_string();
    let message = match message_value {
        Value::String(rc) => rc.as_ref().to_owned(),
        _ => unreachable!(),
    };

    let passed = args.get(1).unwrap().to_bool();

    ASSERT_LOG.with(|log| log.borrow_mut().push(AssertionRecord {
        message: message.clone(),
        passed,
    }));

    if passed {
        Ok(Value::Empty)
    } else {
        Err(RuntimeError::new(message))
    }
}

register_method!("assert", fn_assert);
