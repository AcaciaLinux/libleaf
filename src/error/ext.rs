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
