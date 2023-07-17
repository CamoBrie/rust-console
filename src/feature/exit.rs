use crate::{feature::Feature, state::State};
use crossterm::{
    event::KeyCode,
    style::{StyledContent, Stylize},
};

/// Exit feature
/// A simple feature that exits the application whenever it is selected.
pub struct ExitFeature;

impl Feature for ExitFeature {
    fn get_info(&self) -> super::FeatureInfo {
        super::FeatureInfo {
            key: KeyCode::Esc,
            name: "Quit".dark_grey(),
            description: "Exit the application.".dark_grey(),
            visible_count: 0,
            unlock_count: 0,
            counter_string: None,
        }
    }

    fn get_top_bar(&self, _state: &State) -> Vec<StyledContent<String>> {
        vec![]
    }

    fn update(&mut self, _: f32, state: &mut State, _: &mut crate::message::MessageManager) {
        state.quit = true;
    }

    fn render(&self, _: &State, _: &Vec<Box<dyn Feature>>) -> Vec<StyledContent<String>> {
        vec!["See you later!".to_string().stylize()]
    }
}
