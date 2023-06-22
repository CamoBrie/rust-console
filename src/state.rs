use console::Key;

use crate::feature::fight::FightData;

/// State struct:
/// A struct that holds the state of the application. This is the data that is passed around to all features.
pub struct State {
  pub count: i32,
  pub key: Key,
  pub selected_feature: Option<usize>,

  pub fight: FightData,
}
