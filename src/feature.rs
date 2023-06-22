use console::StyledObject;

use crate::state::State;

pub trait Feature {
  fn get_key(&self) -> char;
  fn get_name(&self) -> StyledObject<&str>;
  fn update(&mut self, state: &mut State);
  fn render(&self, state: &State) -> String;
}

pub mod counter;
pub mod fight;
