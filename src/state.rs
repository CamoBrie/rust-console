use crate::feature::{fight::FightData, inventory::Inventory};
use crossterm::event::KeyCode;

/// State struct:
/// A struct that holds the state of the application. This is the data that is passed around to all features.
pub struct State {
    pub key: KeyCode,
    pub selected_feature: Option<usize>,
    pub quit: bool,

    pub count: i32,
    pub fight: FightData,
    pub inventory: Inventory,
}
