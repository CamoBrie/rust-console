use crossterm::{event::KeyCode, style::{StyledContent, Stylize}};
use crate::{feature::Feature, state::State};


/// Exit feature
/// A simple feature that exits the application whenever it is selected.
pub struct ExitFeature;

impl Feature for ExitFeature {
  fn get_key(&self) -> KeyCode {
    KeyCode::Esc
  }

  fn get_name(&self) -> StyledContent<&str> {
    "Quit".dark_grey()
  }

  fn get_inputs(&self) -> Vec<(KeyCode, StyledContent<String>)> {
    vec![]
  }

  fn update(&mut self, _: f32, _: &mut State) {
    std::process::exit(0);
  }

  fn render(&self, _: &State) -> Vec<StyledContent<String>> {
    vec!["See you later!".to_string().stylize()]
  }
}

