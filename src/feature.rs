use crate::state::State;
use crossterm::{event::KeyCode, style::StyledContent};

/// Feature struct
/// A struct that contains all the information about a feature
pub struct FeatureInfo {
    pub key: KeyCode,
    pub name: StyledContent<&'static str>,
    pub description: StyledContent<&'static str>,
    pub visible_count: i32,
    pub unlock_count: i32,
    pub counter_string: Option<StyledContent<&'static str>>,
}

/// Feature trait:
/// A trait that defines a feature. A feature is a part of the application that can be selected and updated.
/// Each feature has a key, a name, a list of inputs, an update function and a render function.
pub trait Feature {
    /// Get the info of this feature
    fn get_info(&self) -> FeatureInfo;

    /// Get the top bar of this feature
    fn get_top_bar(&self, state: &State) -> Vec<StyledContent<String>>;

    /// Check if this feature is unlocked
    fn is_unlocked(&self, _state: &State) -> bool {
        true
    }

    /// Update the feature
    fn update(
        &mut self,
        delta: f32,
        state: &mut State,
        message: &mut crate::message::MessageManager,
    );

    /// Render the feature
    fn render(&self, state: &State, features: &Vec<Box<dyn Feature>>)
        -> Vec<StyledContent<String>>;
}

pub mod counter;
pub mod exit;
pub mod fight;
pub mod inventory;
pub mod shop;
