use crate::state::State;
use crossterm::{event::KeyCode, style::StyledContent};

/// Feature trait:
/// A trait that defines a feature. A feature is a part of the application that can be selected and updated.
/// Each feature has a key, a name, a list of inputs, an update function and a render function.
pub trait Feature {
    /// Get the key that selects this feature
    fn get_key(&self) -> KeyCode;

    /// Get the name of this feature
    fn get_name(&self) -> StyledContent<&str>;

    /// Get the list of inputs for this feature
    fn get_inputs(&self) -> Vec<(KeyCode, StyledContent<String>)>;

    /// Update the feature
    fn update(&mut self, ms_step: f32, state: &mut State);

    /// Render the feature
    fn render(&self, state: &State) -> Vec<StyledContent<String>>;
}

pub mod counter;
pub mod exit;
pub mod fight;
