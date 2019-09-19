use crate::error::SCError;

/// A shorthand notation for `Result<T, SCError>`.
pub type SCResult<T> = Result<T, SCError>;

/// Indicates that every variant of
/// this type has an "opponent".
pub trait HasOpponent {
	/// Fetches the opponent of this variant.
	fn opponent(self) -> Self;
}
