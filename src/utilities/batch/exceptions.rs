use std::{backtrace::Backtrace, error::Error, fmt};

#[derive(Debug)]
pub struct ExceptionInfo(Backtrace);

#[derive(Debug)]
pub struct BatchProcessingError<'a> {
    msg: String,
    child_exceptions: &'a Vec<ExceptionInfo>,
}

impl<'a> BatchProcessingError<'a> {
    pub fn new(msg: String, child_exceptions: &'a Vec<ExceptionInfo>) -> Self {
        BatchProcessingError {
            msg,
            child_exceptions,
        }
    }
}

impl<'a> Error for BatchProcessingError<'a> {}

impl<'a> fmt::Display for BatchProcessingError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut exception_list = vec![];
        for exception in self.child_exceptions {
            let ExceptionInfo(tb) = exception;
            exception_list.push(tb.to_string());
        }
        write!(f, "{:?}: {:?}", self.msg, exception_list.join("\n"))
    }
}
