use crate::interpreter::runtime_errors::StackAttachable;

#[derive(Debug, Clone)]
pub struct StackFrame {
    pub function: String,
    pub location: Option<usize>, // optional line:col info
}

#[derive(Debug, Default)]
pub struct CallStack {
    pub frames: Vec<StackFrame>,
}

impl CallStack {
    pub fn new() -> Self {
        CallStack { frames: vec![] }
    }

    pub fn push(&mut self, frame: StackFrame) {
        self.frames.push(frame);
    }

    pub fn pop(&mut self) {
        self.frames.pop();
    }

    pub fn attach_to_error<T>(&self, mut err: T) -> T
    where
        T: StackAttachable,
    {
        for frame in &self.frames {
            err = err.with_frame(frame.clone());
        }
        err
    }
}
