use crate::{feature::Feature, state::State, util::flag::Flag, util::flag::Flags};
use crossterm::{
    event::KeyCode,
    style::{StyledContent, Stylize},
};
use enum_iterator::Sequence;

/// Fight feature
/// A feature that allows the player to fight enemies.
/// The player can move up and down floors, attack enemies and collect gold.
/// The player can also level up and gain more health.
/// The player can die and respawn.
pub struct FightFeature {
    flags: Flags<FightFlag, FightData>,
}
impl Default for FightFeature {
    fn default() -> FightFeature {
        FightFeature {
            flags: Flags::new(),
        }
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Debug, Sequence)]
enum FightFlag {
    Attack,
    EnemyAttack,
    Respawn,
    EnemyDead,
    PlayerDead,
}

impl Flag<FightData> for FightFlag {
    fn handle(&self, flags: &mut Flags<Self, FightData>, state: &mut State) {
        let data = &mut state.fight;
        match self {
            FightFlag::Attack => {
                if let Some(enemy) = &mut data.enemy {
                    enemy.health -= (data.player.attack - enemy.defense).max(0.0);
                    data.attack_timer = data.attack_max;

                    if enemy.health <= 0.0 {
                        flags.mark(FightFlag::EnemyDead)
                    }
                }
            }
            FightFlag::EnemyAttack => {
                if let Some(enemy) = &mut data.enemy {
                    data.player.health -= (enemy.attack - data.player.defense).max(0.0);
                    data.enemy_timer = data.enemy_max;

                    if data.player.health <= 0.0 {
                        flags.mark(FightFlag::PlayerDead);
                    }
                }
            }
            FightFlag::Respawn => {
                if data.floor > 0 {
                    // only spawn enemy if not on floor 0
                    data.enemy = Some(get_enemy(data.floor));
                    data.enemy_timer = data.enemy_max;
                }
            }
            FightFlag::EnemyDead => {
                data.enemy = None;
                data.enemy_timer = data.enemy_max;

                data.enemy_count += 1;
                if data.enemy_count >= data.enemy_required {
                    data.enemy_count = 0;
                    data.enemy_required += 1;
                    data.max_floor += 1;
                }

                state.inventory.add("Gold", data.floor as u64);
                state.inventory.add("XP", data.floor as u64);

                if state.inventory.get_amount("XP") >= data.xp_to_next_level {
                    state.inventory.remove("XP", data.xp_to_next_level);

                    let xp_increase = 10.0 * 1.15f32.powi(data.level as i32);

                    data.xp_to_next_level += xp_increase as u64;
                    data.level += 1;
                }
            }
            FightFlag::PlayerDead => {
                data.player.health = data.player.max_health;
                data.enemy = None;
                data.floor = 0;

                state
                    .inventory
                    .remove("Gold", state.inventory.get_amount("Gold") >> 1);
            }
        };
    }
}

impl Feature for FightFeature {
    fn get_info(&self) -> super::FeatureInfo {
        super::FeatureInfo {
            key: KeyCode::Char('f'),
            name: "Fight".red(),
            description: "Fight enemies, collect gold and XP.".dark_grey(),
            visible_count: 0,
            unlock_count: 10,
            counter_string: None,
        }
    }

    fn get_top_bar(&self, _state: &State) -> Vec<StyledContent<String>> {
        vec![
            " [<]Go down a floor ".to_string().stylize(),
            "[>]Go up a floor ".to_string().stylize(),
            "[a]Attack".to_string().stylize(),
        ]
    }

    fn update(&mut self, delta: f32, state: &mut State, _: &mut crate::message::MessageManager) {
        process_input(self, state.key, &mut state.fight);
        self.flags.handle(state);
        update_timers(self, delta, &mut state.fight);

        // simple healing at floor 0.
        if state.fight.floor == 0 && state.fight.player.health <= state.fight.player.max_health {
            state.fight.player.health += delta as f64 * state.fight.regen;
            if state.fight.player.health > state.fight.player.max_health {
                state.fight.player.health = state.fight.player.max_health;
            }
        }
    }

    fn render(&self, state: &State, _: &Vec<Box<dyn Feature>>) -> Vec<StyledContent<String>> {
        let data = &state.fight;
        vec![
            format!(
                "Floor: {} | Gold: {} | Level: {} | XP: {}/{}",
                data.floor,
                state.inventory.get_amount("Gold"),
                data.level,
                state.inventory.get_amount("XP"),
                data.xp_to_next_level
            )
            .stylize(),
            if data.respawn_timer != data.respawn_max {
                format!("Respawn in {:.2}", data.respawn_timer).stylize()
            } else {
                if let Some(enemy) = &data.enemy {
                    format!(
                        "Enemy goal: {}/{} |{} Enemy HP: {:.2} | Enemy attack: {:.2}",
                        data.enemy_count,
                        data.enemy_required,
                        if enemy.defense > 0.0 {
                            format!(" Enemy defense: {:.2} |", enemy.defense).stylize()
                        } else {
                            "".to_string().stylize()
                        },
                        enemy.health,
                        data.enemy_timer,
                    )
                    .stylize()
                } else {
                    "No enemy".to_string().stylize()
                }
            },
            format!(
                "Player HP: {:.2}/{:.2} | Damage: {:.2} | Attack: {:.2}",
                data.player.health, data.player.max_health, data.player.attack, data.attack_timer
            )
            .stylize(),
            //format!("{:?}", self.flags).stylize(),
        ]
    }
}

/// Update the fight feature timers, and set flags if the timers are up.
fn update_timers(flags: &mut FightFeature, delta: f32, data: &mut FightData) {
    if data.respawn_timer > 0.0 && data.enemy.is_none() && data.floor > 0 {
        data.respawn_timer -= delta;
        if data.respawn_timer <= 0.0 {
            data.respawn_timer = data.respawn_max;
            flags.flags.mark(FightFlag::Respawn);
        }
    }

    if data.attack_timer > 0.0 && data.enemy.is_some() {
        data.attack_timer -= delta;
    }

    if data.enemy_timer > 0.0 && data.enemy.is_some() {
        data.enemy_timer -= delta;
        if data.enemy_timer <= 0.0 {
            flags.flags.mark(FightFlag::EnemyAttack)
        }
    }
}

/// Process user input
fn process_input(flags: &mut FightFeature, key: KeyCode, data: &mut FightData) {
    match key {
        KeyCode::Left => {
            // go down a floor
            data.floor = if data.floor > 0 { data.floor - 1 } else { 0 };
            data.respawn_timer = data.respawn_max;
            data.attack_timer = data.attack_max;
            data.enemy_timer = data.enemy_max;
            data.enemy = None;
        }
        KeyCode::Right => {
            // go up a floor
            data.floor = if data.floor < data.max_floor {
                data.floor + 1
            } else {
                data.max_floor
            };
            data.respawn_timer = data.respawn_max;
            data.attack_timer = data.attack_max;
            data.enemy_timer = data.enemy_max;
            data.enemy = None;
        }
        KeyCode::Char('a') => {
            if data.attack_timer <= 0.0 {
                data.attack_timer = data.attack_max;
                flags.flags.mark(FightFlag::Attack);
            }
        }
        _ => {}
    }
}

/// Get a new enemy based on the floor
fn get_enemy(floor: u32) -> Living {
    Living {
        attack: 0.9 + floor as f64 * 0.1,
        defense: if floor >= 5 {
            (floor - 5) as f64 * 0.1
        } else {
            0.0
        },
        health: 3.0 + floor as f64 * 2.0,
        max_health: 3.0 + floor as f64 * 2.0,
    }
}

/// Starting state for the fight feature
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

            attack_timer: 1.5,
            attack_max: 1.5,

            enemy_timer: 2.0,
            enemy_max: 2.0,

            xp_to_next_level: 10,
            level: 1,
            regen: 0.5,
        }
    }
}

/// Data for the fight feature.
/// It contains all data related to the feature
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
    pub respawn_max: f32,
    attack_timer: f32,
    pub attack_max: f32,
    enemy_timer: f32,
    enemy_max: f32,

    // player data
    xp_to_next_level: u64,
    pub level: u32,
    pub regen: f64,
}

/// Struct for the living entities in the fight feature
pub struct Living {
    pub attack: f64,
    pub defense: f64,
    pub health: f64,
    pub max_health: f64,
}
