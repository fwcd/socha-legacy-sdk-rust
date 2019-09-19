use crate::error::SCError;

/// A shorthand notation for `Result<T, SCError>`.
pub type SCResult<T> = Result<T, SCError>;
