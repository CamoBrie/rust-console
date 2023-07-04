use crate::{feature::Feature, state::State};
use crossterm::{
    event::KeyCode,
    style::{StyledContent, Stylize},
};

/// Counter feature
/// A simple feature that increments a counter when the 'c' key is pressed.
pub struct CounterFeature;

impl Feature for CounterFeature {
    fn get_key(&self) -> KeyCode {
        KeyCode::Char('c')
    }

    fn get_name(&self) -> StyledContent<&str> {
        "Counter".cyan()
    }

    fn get_top_bar(&self, _state: &State) -> Vec<StyledContent<String>> {
        vec![" [c]Increment counter".to_string().stylize()]
    }

    fn update(&mut self, _: f32, state: &mut State) {
        if state.key == KeyCode::Char('c') {
            state.count += 1;
        }
    }

    fn render(&self, state: &State) -> Vec<StyledContent<String>> {
        vec![format!("Count: {}", state.count).stylize()]
    }
}
