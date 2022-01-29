/// Indicates that the value has an "associated" team.
/// The plugin-specific `GameState` should return the current
/// team when implementing this trait.
pub trait HasTeam {
    type Team;

    /// Fetches the associated team.
    fn team(&self) -> Self::Team;
}
