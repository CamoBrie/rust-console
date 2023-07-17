use crossterm::{event::KeyCode, style::Stylize};

use super::Feature;
use crate::*;

pub struct ShopFeature;

impl Feature for ShopFeature {
    fn get_info(&self) -> super::FeatureInfo {
        super::FeatureInfo {
            key: KeyCode::Char('s'),
            name: "Shop".yellow(),
            description: "A shop where you can buy items to help you in your adventure".dark_grey(),
            visible_count: 50,
            unlock_count: 200,
            counter_string: None,
        }
    }

    fn get_top_bar(
        &self,
        state: &crate::state::State,
    ) -> Vec<crossterm::style::StyledContent<String>> {
        vec![
            " | Gold: ".to_string().stylize(),
            state
                .inventory
                .get_amount("gold")
                .to_string()
                .dark_yellow()
                .bold(),
        ]
    }

    fn update(
        &mut self,
        delta: f32,
        state: &mut state::State,
        message: &mut message::MessageManager,
    ) {
    }

    fn render(
        &self,
        state: &state::State,
        features: &Vec<Box<dyn Feature>>,
    ) -> Vec<crossterm::style::StyledContent<String>> {
        vec!["list of items".to_string().stylize()]
    }
}
