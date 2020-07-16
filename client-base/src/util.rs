use crate::error::SCError;

/// Creates a new HashMap using a literal-like syntax. It automatically
/// performs `Into` conversions for convenience.
/// 
/// Source: https://stackoverflow.com/questions/27582739/how-do-i-create-a-hashmap-literal
#[macro_export]
macro_rules! hashmap {
    [ $($key:expr => $value:expr),* ] => {{
        let mut m = ::std::collections::HashMap::new();
        $(
            m.insert($key.into(), $value.into());
        )+
        m
    }}
}

/// A shorthand notation for `Result<T, SCError>`.
pub type SCResult<T> = Result<T, SCError>;

/// Indicates that every variant of
/// this type has an "opponent".
pub trait HasOpponent {
    /// Fetches the opponent of this variant.
    fn opponent(self) -> Self;
}
