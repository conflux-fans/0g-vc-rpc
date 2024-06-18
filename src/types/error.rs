use std::fmt::{self, Display};

use serde::{de, ser};

#[derive(Debug)]
pub enum VcError {
    Message(String),
}

impl ser::Error for VcError {
    fn custom<T: Display>(msg: T) -> Self {
        VcError::Message(msg.to_string())
    }
}

impl de::Error for VcError {
    fn custom<T: Display>(msg: T) -> Self {
        VcError::Message(msg.to_string())
    }
}

impl Display for VcError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            VcError::Message(msg) => formatter.write_str(msg),
        }
    }
}

impl std::error::Error for VcError {}
