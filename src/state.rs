use console::Key;

use crate::feature::fight::FightData;

pub struct State {
  pub count: i32,
  pub key: Key,
  pub selected_feature: Option<usize>,

  pub fight: FightData,
}
