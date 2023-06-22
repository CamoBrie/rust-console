use crate::state::State;

pub trait Feature {
  fn update(&mut self, state: &mut State);
  fn render(&self, state: &State) -> String;
}

pub mod counter;
