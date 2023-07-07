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

    fn get_description(&self) -> StyledContent<&str> {
        "A simple counter that increments when the 'c' key is pressed. It is the way to unlock new content".dark_grey()
    }

    fn update(&mut self, _: f32, state: &mut State) {
        if state.key == KeyCode::Char('c') {
            state.count += 1;
        }
    }

    fn render(
        &self,
        state: &State,
        features: &Vec<Box<dyn Feature>>,
    ) -> Vec<StyledContent<String>> {
        let mut lines = vec![
            format!("Count: {}", state.count).stylize(),
            "".to_string().stylize(),
        ];
        lines.append(&mut get_unlocks(state, features));
        lines
    }
}

fn get_unlocks(state: &State, features: &Vec<Box<dyn Feature>>) -> Vec<StyledContent<String>> {
    let mut unlocks = vec![];

    for feature in features {
        let (necessary_count, visible_count, text) = feature.counter_data();

        if feature.is_unlocked(state) && state.count >= necessary_count {
            continue;
        }

        if state.count >= visible_count {
            unlocks.push(
                format!("{}{} unlocks {}", necessary_count, text, feature.get_name()).stylize(),
            );
        }
    }

    unlocks
}
