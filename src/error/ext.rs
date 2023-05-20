//! This module provides functionality for enriching error types that
//! work with LError
use super::LError;

/// This trait allows any Result that returns a LError to use some common functions
pub trait LErrorExt<T> {
    /// This function can take a Result and if it is an `Err`, it appends the
    /// supplied message using the LError::append() function
    /// # Arguments
    /// * `message` - The message to append to the error
    fn err_append(self, message: &str) -> Result<T, LError>;

    /// This function can take a Result and if it is an `Err`, it prepends the
    /// supplied message using the LError::prepend() function
    /// # Arguments
    /// * `message` - The message to prepend to the error
    fn err_prepend(self, message: &str) -> Result<T, LError>;
}

/// Implement the trait for the Result returning the LError
impl<T> LErrorExt<T> for Result<T, LError> {
    fn err_append(self, message: &str) -> Result<T, LError> {
        match self {
            Ok(v) => Ok(v),
            Err(mut e) => {
                e.append(message);
                Err(e)
            }
        }
    }

    fn err_prepend(self, message: &str) -> Result<T, LError> {
        match self {
            Ok(v) => Ok(v),
            Err(mut e) => {
                e.prepend(message);
                Err(e)
            }
        }
    }
}

impl<T> LErrorExt<T> for Result<T, rusqlite::Error> {
    fn err_append(self, message: &str) -> Result<T, LError> {
        match self {
            Ok(v) => Ok(v),
            Err(e) => {
                let mut le = LError::from(e);
                le.append(message);
                Err(le)
            }
        }
    }

    fn err_prepend(self, message: &str) -> Result<T, LError> {
        match self {
            Ok(v) => Ok(v),
            Err(e) => {
                let mut le = LError::from(e);
                le.prepend(message);
                Err(le)
            }
        }
    }
}

impl<T> LErrorExt<T> for Result<T, std::io::Error> {
    fn err_append(self, message: &str) -> Result<T, LError> {
        match self {
            Ok(v) => Ok(v),
            Err(e) => {
                let mut le = LError::from(e);
                le.append(message);
                Err(le)
            }
        }
    }

    fn err_prepend(self, message: &str) -> Result<T, LError> {
        match self {
            Ok(v) => Ok(v),
            Err(e) => {
                let mut le = LError::from(e);
                le.prepend(message);
                Err(le)
            }
        }
    }
}
