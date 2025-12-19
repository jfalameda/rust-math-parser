use std::fmt;

use crate::interpreter::call_stack::StackFrame;

#[derive(Debug, Clone)]
pub struct RuntimeError {
    pub message: String,
    pub stack: Vec<StackFrame>,
}

pub trait StackAttachable: Sized {
    fn with_frame(self, frame: StackFrame) -> Self;
}

impl StackAttachable for RuntimeError {
    fn with_frame(mut self, frame: StackFrame) -> Self {
        self.stack.push(frame);
        self
    }
}

impl RuntimeError {
    pub fn new<S: Into<String>>(msg: S) -> Self {
        RuntimeError {
            message: msg.into(),
            stack: vec![],
        }
    }
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Runtime Error: {}", self.message)?;
        if !self.stack.is_empty() {
            writeln!(f, "Call stack:")?;
            for frame in self.stack.iter().rev() {
                let location_str = frame
                    .location
                    .map(|loc| loc.to_string())
                    .unwrap_or_else(|| "?".to_string());
                writeln!(f, "  at {} ({})", frame.function, location_str)?;
            }
        }
        Ok(())
    }
}

impl std::error::Error for RuntimeError {}
