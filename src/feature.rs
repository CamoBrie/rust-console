use crossterm::{event::KeyCode, style::StyledContent};
use crate::state::State;


/// Feature trait:
/// A trait that defines a feature. A feature is a part of the application that can be selected and updated.
pub trait Feature {
  fn get_key(&self) -> KeyCode;
  fn get_name(&self) -> StyledContent<&str>;
  fn update(&mut self, state: &mut State);
  fn render(&self, state: &State) -> String;
}

pub mod counter;
pub mod fight;
pub mod exit;
