//! The error leaf works with

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
    JSON,
    MirrorNotLoaded,
    PackageNotFound,
    UnresolvedDependencies,
    UnexpectedPackageVariant,
    UnexpectedDependenciesVariant,

    IO(io::ErrorKind),
}

/// A leaf error has a class and an optional message
#[derive(Debug, Clone)]
#[repr(C)]
pub struct LError {
    pub class: LErrorClass,
    pub message: Option<String>,
}

impl LError {
    /// Creates a new LError with only the class set
    /// # Arguments
    /// * `class` - The type of error
    pub fn new_class(class: LErrorClass) -> LError {
        LError {
            class,
            message: None,
        }
    }

    /// Creates a new LError with an attached message
    /// # Arguments
    /// * `class` - The type of error
    /// * `message` - Additional information about the error at hand
    pub fn new(class: LErrorClass, message: &str) -> LError {
        LError {
            class,
            message: Some(message.to_owned()),
        }
    }

    /// Creates a LError that represents no error
    pub fn none() -> LError {
        LError::new_class(LErrorClass::None)
    }

    /// Converts the error class to a human-readable string
    pub fn ec_str(&self) -> String {
        use LErrorClass::*;
        match self.class {
            None => "None",
            Unknown => "Unknown",
            Abort => "Action was aborted",
            CURL => "Error in CURL occured",
            JSON => "Failed to parse json",
            MirrorNotLoaded => "Mirror was not loaded",
            PackageNotFound => "Package could not be found",
            UnresolvedDependencies => "Some dependencies are unresolved",
            UnexpectedPackageVariant => "Unexpected package variant",
            UnexpectedDependenciesVariant => "Unexpected dependencies variant",
            IO(_) => "An IO error occured",
        }
        .to_owned()
    }
}

impl error::Error for LError {}

impl Display for LError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.message {
            Some(m) => write!(f, "{}: {}", &self.ec_str(), m),
            None => write!(f, "{}", &self.ec_str()),
        }
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
            message: Some(value.to_string()),
        }
    }
}
