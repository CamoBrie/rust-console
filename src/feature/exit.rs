use crossterm::{event::KeyCode, style::{StyledContent, Stylize}};
use crate::{feature::Feature, state::State};


pub struct ExitFeature;

impl Feature for ExitFeature {
  fn get_key(&self) -> KeyCode {
    KeyCode::Esc
  }

  fn get_name(&self) -> StyledContent<&str> {
    "Quit".dark_grey()
  }

  fn update(&mut self, _: &mut State)
  {
    std::process::exit(0);
  }

  fn render(&self, _: &State) -> String
  {
    "See you later!".to_string()
  }
}

