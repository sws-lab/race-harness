use std::{error::Error, fmt::Display};


#[derive(Debug)]
pub struct HarnessError(String);

impl Display for HarnessError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Harness error: {}", self.get_message())
    }
}

impl Error for HarnessError {}

impl HarnessError {
    pub fn new<T>(message: T) -> HarnessError
        where T: Into<String> {
        HarnessError(message.into())
    }

    pub fn get_message(&self) -> &str {
        self.0.as_str()
    }
}
