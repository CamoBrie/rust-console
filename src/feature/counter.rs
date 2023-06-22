use crossterm::{event::KeyCode, style::{StyledContent, Stylize}};
use crate::{feature::Feature, state::State};


pub struct CounterFeature;

impl Feature for CounterFeature
{
  fn get_key(&self) -> KeyCode {
    KeyCode::Char('c')
  }

  fn get_name(&self) -> StyledContent<&str> {
    "Counter".cyan()
  }

  fn update(&mut self, _: f32, state: &mut State)
  {
    if state.key == KeyCode::Char('c') {
      state.count += 1;
    }
  }

  fn render(&self, state: &State) -> String
  {
    format!("Count: {}", state.count)
  }
}