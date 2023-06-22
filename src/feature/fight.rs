use crossterm::{event::KeyCode, style::{StyledContent, Stylize}};
use crate::{feature::Feature, state::State};

#[derive(Default)]
pub struct FightFeature {
  attack: bool, // player attack flag
  enemy_attack: bool, // enemy attack flag
  respawn: bool, // enemy respawn flag
  enemy_dead: bool, // reward flag
  player_dead: bool, // death flag
}

impl Feature for FightFeature {

  fn get_key(&self) -> KeyCode {
    KeyCode::Char('f')
  }

  fn get_name(&self) -> StyledContent<&str> {
    "Fight".red()
  }

  fn update(&mut self, ms_step: f32, state: &mut State) {
    process_input(self, state.key, &mut state.fight);
    perform_flags(self, &mut state.fight);
    update_timers(self, ms_step, &mut state.fight);

    // simple healing at floor 0.
    if state.fight.floor == 0 && state.fight.player.health <= state.fight.player.max_health {
      state.fight.player.health += ms_step as f64 * 0.5;
      if state.fight.player.health > state.fight.player.max_health {
        state.fight.player.health = state.fight.player.max_health;
      }
    }
  }

  fn render(&self, state: &State) -> String {
    let mut str = String::new();
    let data = &state.fight;

    str.push_str(&format!("Floor: {} | ", data.floor));
    str.push_str(&format!("Gold: {} | ", data.gold));
    str.push_str(&format!("Level: {} | ", data.level));
    str.push_str(&format!("XP: {}/{} \n", data.xp, data.xp_to_next_level));
    str.push_str(&format!("Enemy goal: {}/{} | ", data.enemy_count, data.enemy_required));
    str.push_str(&format!("Enemy HP: {} | ", data.enemy.as_ref().map(|e| e.health).unwrap_or(0.0)));
    str.push_str(&format!("Enemy attack: {:.2} | ", data.enemy_timer));
    str.push_str(&format!("Respawn: {:.2}\n", data.respawn_timer));
    str.push_str(&format!("Player HP: {:.2} | ", data.player.health));
    str.push_str(&format!("Attack: {:.2}\n", data.attack_timer));

    str.push_str(&format!("flags: {} {} {} {} {}\n", 
      if self.attack { "attack " } else { "" },
      if self.enemy_attack { "enemyAttack " } else { "" },
      if self.respawn { "respawn " } else { "" },
      if self.enemy_dead { "enemyDead " } else { "" },
      if self.player_dead { "playerDead " } else { "" },
    ));

    str

  }
}

fn update_timers(flags: &mut FightFeature, ms_step: f32, data: &mut FightData) {
  if data.respawn_timer > 0.0 && data.enemy.is_none() && data.floor > 0 {
    data.respawn_timer -= ms_step;
    if data.respawn_timer <= 0.0 {
      data.respawn_timer = data.respawn_max;
      flags.respawn = true;
    }
  }

  if data.attack_timer > 0.0 && data.enemy.is_some() {
    data.attack_timer -= ms_step;
  }

  if data.enemy_timer > 0.0 && data.enemy.is_some() {
    data.enemy_timer -= ms_step;
    if data.enemy_timer <= 0.0 {
      flags.enemy_attack = true;
    }
  }
}

fn process_input(flags: &mut FightFeature, key: KeyCode, data: &mut FightData) {
  match key {
    KeyCode::Left => { // go down a floor
      data.floor = if data.floor > 0 { data.floor - 1 } else { 0 };
      data.respawn_timer = data.respawn_max;
      data.attack_timer = data.attack_max;
      data.enemy_timer = data.enemy_max;
      data.enemy = None;
    },
    KeyCode::Right => { // go up a floor
      data.floor = if data.floor < data.max_floor { data.floor + 1 } else { data.max_floor };
      data.respawn_timer = data.respawn_max;
      data.attack_timer = data.attack_max;
      data.enemy_timer = data.enemy_max;
      data.enemy = None;
    },
    KeyCode::Char(' ') => { 
      if data.attack_timer <= 0.0 {
        data.attack_timer = data.attack_max;
        flags.attack = true;
      }
    }
    _ => {}
  }
}

fn perform_flags(flags: &mut FightFeature, data: &mut FightData) {
  if flags.attack {
    flags.attack = false;
    if let Some(enemy) = &mut data.enemy {
      enemy.health -= (data.player.attack - enemy.defense).max(0.0);
      data.attack_timer = data.attack_max;

      if enemy.health <= 0.0 {
        flags.enemy_dead = true;
      }
    }
  };

  if flags.enemy_attack {
    flags.enemy_attack = false;
    if let Some(enemy) = &mut data.enemy {
      data.player.health -= (enemy.attack - data.player.defense).max(0.0);
      data.enemy_timer = data.enemy_max;

      if data.player.health <= 0.0 {
        flags.player_dead = true;
      }
    }
  };

  if flags.respawn {
    flags.respawn = false;
    if data.floor > 0 { // only spawn enemy if not on floor 0
      data.enemy = Some(Living {
        attack: 1.0, // TODO: make formula based on floor
        defense: 0.0,
        health: 5.0,
        max_health: 5.0,
      });
      data.enemy_timer = data.enemy_max;
    }
  };

  if flags.enemy_dead {
    flags.enemy_dead = false;
    data.enemy = None;
    data.enemy_timer = data.enemy_max;

    data.gold += 1; // TODO: make formula based on floor
    data.xp += 1; // TODO: make formula based on floor
    if data.xp >= data.xp_to_next_level { // TODO: make formula based on floor
      data.xp -= data.xp_to_next_level;
      data.xp_to_next_level += 10;
      data.level += 1;
    };

    if data.floor == data.max_floor {
      data.enemy_count += 1;

      if data.enemy_count >= data.enemy_required {
        data.max_floor += 1;
        data.enemy_count = 0;
        data.enemy_required += 1;
      };
    };
  };

  if flags.player_dead {
    flags.player_dead = false;
    data.player.health = data.player.max_health;
    data.gold = (data.gold as f64 * 0.5) as u32; // TODO: make better
    data.floor = 0;
  };

}

impl Default for FightData {
  fn default() -> Self {
    Self {
      player: Living {
        attack: 1.0,
        defense: 0.0,
        health: 10.0,
        max_health: 10.0,
      },
      enemy: None,

      floor: 0,
      max_floor: 1,
      enemy_count: 0,
      enemy_required: 10,

      respawn_timer: 0.0,
      respawn_max: 3.0,

      attack_timer: 0.0,
      attack_max: 1.5,

      enemy_timer: 2.0,
      enemy_max: 2.0,

      gold: 0,
      xp: 0,
      xp_to_next_level: 10,
      level: 1,
    }
  }
}

pub struct FightData {
  pub player: Living,
  enemy: Option<Living>,

  // floor data
  pub floor: u32,
  pub max_floor: u32,
  enemy_count: u32,
  enemy_required: u32,

  // fight data
  respawn_timer: f32,
  respawn_max: f32,
  attack_timer: f32,
  attack_max: f32,
  enemy_timer: f32,
  enemy_max: f32,

  // player data
  pub gold: u32,
  pub xp: u64,
  xp_to_next_level: u64,
  pub level: u32,
}

pub struct Living {
  pub attack: f64,
  pub defense: f64,
  pub health: f64,
  pub max_health: f64,
}