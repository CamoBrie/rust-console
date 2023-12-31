use crate::{feature::Feature, message, state::State};
use crossterm::{
    event::KeyCode,
    style::{StyledContent, Stylize},
};

/// Counter feature
/// A simple feature that increments a counter when the 'c' key is pressed.
pub struct CounterFeature;

impl Feature for CounterFeature {
    fn get_info(&self) -> super::FeatureInfo {
        super::FeatureInfo {
            key: KeyCode::Char('c'),
            name: "Counter".cyan(),
            description: "A simple counter that increments when the 'c' key is pressed. It is the way to unlock new content".dark_grey(),
            visible_count: 0,
            unlock_count: 0,
            counter_string: None,
        }
    }

    fn get_top_bar(&self, _state: &State) -> Vec<StyledContent<String>> {
        vec![" [c]Increment".to_string().stylize()]
    }

    fn update(&mut self, _: f32, state: &mut State, message: &mut message::MessageManager) {
        if state.key == KeyCode::Char('c') {
            state.count += 1;
        }

        if state.count == 0 {
            message.add_message(message::Message {
                    text: "Keep going until you reach 10 count, you will unlock the fight feature! You can hide messages by pressing [Enter]."
                        .to_string().bold(),
                    location: message::TextLocation::Center,
                    duration: 10.0,
                });
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
        let info = feature.get_info();

        if feature.is_unlocked(state) && state.count >= info.unlock_count {
            continue;
        }

        if state.count >= info.visible_count {
            unlocks.push(
                format!(
                    "{}{} unlocks {}",
                    info.unlock_count,
                    info.counter_string.unwrap_or("".stylize()),
                    info.name
                )
                .stylize(),
            );
        }
    }

    unlocks
}
