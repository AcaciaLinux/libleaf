//! The error leaf works with

mod ext;
pub use ext::*;

use std::{
    error,
    fmt::{self, Display},
    io,
};

/// The different types of errors that can occur
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LErrorClass {
    None,
    Unknown,
    Abort,
    CURL,
    CURLHttpNot2xx,
    JSON,
    MirrorNotLoaded,

    IO(io::ErrorKind),
}

/// A leaf error has a class and an optional message
#[derive(Debug, Clone)]
#[repr(C)]
pub struct LError {
    pub class: LErrorClass,
    pub messages: Vec<String>,
}

impl LError {
    /// Creates a new LError with only the class set
    /// # Arguments
    /// * `class` - The type of error
    pub fn new_class(class: LErrorClass) -> LError {
        LError {
            class,
            messages: Vec::new(),
        }
    }

    /// Creates a new LError with an attached message
    /// # Arguments
    /// * `class` - The type of error
    /// * `message` - Additional information about the error at hand
    pub fn new(class: LErrorClass, message: &str) -> LError {
        LError {
            class,
            messages: vec![message.to_owned()],
        }
    }

    /// Creates a LError that represents no error
    pub fn none() -> LError {
        LError::new_class(LErrorClass::None)
    }

    /// Appends the supplied message to the message of the error
    /// in the following form: `<error message>: <message>`
    /// # Arguments
    /// * `message` - The message to prepend
    pub fn append(&mut self, message: &str) {
        self.messages.push(message.to_owned());
    }

    /// Prepends the supplied message to the message of the error
    /// in the following form: `<message>: <error message>`
    /// # Arguments
    /// * `message` - The message to prepend
    pub fn prepend(&mut self, message: &str) {
        self.messages.insert(0, message.to_owned());
    }

    /// Converts the error class to a human-readable string
    pub fn ec_str(&self) -> String {
        use LErrorClass::*;
        match self.class {
            None => "None",
            Unknown => "Unknown",
            Abort => "Action was aborted",
            CURL => "Error in CURL occured",
            CURLHttpNot2xx => "HTTP response code was not 2xx",
            JSON => "Failed to parse json",
            MirrorNotLoaded => "Mirror not loaded",
            IO(_) => "An IO error occured",
        }
        .to_owned()
    }
}

impl error::Error for LError {}

impl Display for LError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}: ", &self.ec_str())?;

        for (i, msg) in self.messages.iter().enumerate() {
            writeln!(f, "{}- {}", "  ".repeat(i), msg)?;
        }

        Ok(())
    }
}

impl Display for LErrorClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl LErrorClass {
    pub fn from_io_err(e: io::ErrorKind) -> LErrorClass {
        LErrorClass::IO(e)
    }
}

impl From<std::io::Error> for LError {
    fn from(value: std::io::Error) -> Self {
        LError {
            class: LErrorClass::IO(value.kind()),
            messages: vec![value.to_string()],
        }
    }
}
