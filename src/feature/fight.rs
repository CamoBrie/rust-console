use console::{StyledObject, style, Key};

use crate::{feature::Feature, state::State};

pub struct FightFeature;

impl Feature for FightFeature {

  fn get_key(&self) -> Key {
    console::Key::Char('f')
  }

  fn get_name(&self) -> StyledObject<&str> {
    style("Fight").red()
  }

  fn update(&mut self, state: &mut State) {
    if state.key == console::Key::Char('c') {
      state.fight.player.health -= state.fight.enemy.attack - state.fight.player.defense;
      state.fight.enemy.health -= state.fight.player.attack - state.fight.enemy.defense;

      if state.fight.player.health <= 0 {
        state.fight.player.health = 0;
        println!("You died!");
      }

      if state.fight.enemy.health <= 0 {
        state.fight.enemy.health = 0;
        println!("You won!");
      }
    }
  }

  fn render(&self, state: &State) -> String {
    format!("Player: {} Enemy: {}", state.fight.player.health, state.fight.enemy.health)

  }
}

pub struct FightData {
  pub player: Player,
  pub enemy: Enemy,
}

pub struct Player {
  pub attack: i32,
  pub defense: i32,
  pub health: i32,
  pub max_health: i32,
}

pub struct Enemy {
  pub attack: i32,
  pub defense: i32,
  pub health: i32,
  pub max_health: i32,
}