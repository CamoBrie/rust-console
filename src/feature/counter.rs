use crate::{feature::Feature, state::State};

pub struct CounterFeature;

impl Feature for CounterFeature
{
  fn update(&mut self, state: &mut State)
  {
    if state.key == 'c' {
      state.count += 1;
    }
  }

  fn render(&self, state: &State) -> String
  {
    format!("Count: {}", state.count)
  }
}