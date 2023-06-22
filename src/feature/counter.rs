use crate::{feature::{Feature}, state::State};
use console::{style, StyledObject};

pub struct CounterFeature;

impl Feature for CounterFeature
{
  fn get_key(&self) -> char {
    'c'
  }

  fn get_name(&self) -> StyledObject<&str> {
    style("Counter").cyan()
  }

  fn update(&mut self, state: &mut State)
  {
    if state.key == Some('c') {
      state.count += 1;
    }
  }

  fn render(&self, state: &State) -> String
  {
    format!("Count: {}", state.count)
  }
}