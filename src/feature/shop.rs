use crossterm::{event::KeyCode, style::Stylize};

use super::{inventory::Rarity, Feature};
use crate::{
    message::Message,
    util::{conv::to_number, style::rarity_stylize},
    *,
};

pub struct Upgrade {
    pub name: String,
    pub description: String,
    pub cost: u64,
    pub max_count: u32,
    pub rarity: Rarity,
    pub apply: Box<dyn Fn(&mut State)>,
}

pub fn get_all_upgrades() -> Vec<Upgrade> {
    vec![
        Upgrade {
            name: "Damage".to_string(),
            description: "Increases your damage by 1".to_string(),
            cost: 5,
            max_count: 5,
            rarity: Rarity::Common,
            apply: Box::new(|state| {
                state.fight.player.attack += 1.0;
            }),
        },
        Upgrade {
            name: "Health".to_string(),
            description: "Increases your max health by 1".to_string(),
            cost: 5,
            max_count: 5,
            rarity: Rarity::Common,
            apply: Box::new(|state| {
                state.fight.player.max_health += 1.0;
            }),
        },
        Upgrade {
            name: "Regeneration".to_string(),
            description: "Doubles your health regeneration at floor 0".to_string(),
            cost: 10,
            max_count: 1,
            rarity: Rarity::Uncommon,
            apply: Box::new(|state| {
                state.fight.regen += 0.5;
            }),
        },
    ]
}

pub struct UpgradeInfo {
    pub name: String,
    pub count: u32,
}

pub struct Upgrades {
    upgrades: Vec<UpgradeInfo>,
}

impl Upgrades {
    pub fn contains(&self, name: &str) -> Option<u32> {
        self.upgrades
            .iter()
            .find(|u| u.name == name)
            .map(|u| u.count)
    }

    pub fn buy(&mut self, name: &str) {
        if let Some(u) = self.upgrades.iter_mut().find(|u| u.name == name) {
            u.count += 1;
        } else {
            self.upgrades.push(UpgradeInfo {
                name: name.to_string(),
                count: 1,
            });
        }
    }
}

impl Default for Upgrades {
    fn default() -> Self {
        Self {
            upgrades: Vec::new(),
        }
    }
}

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
                .get_amount("Gold")
                .to_string()
                .dark_yellow()
                .bold(),
        ]
    }

    fn update(
        &mut self,
        _delta: f32,
        state: &mut state::State,
        message: &mut message::MessageManager,
    ) {
        if let Some(i) = to_number(state.key) {
            if let Some(upgrade) = get_all_upgrades()
                .iter()
                .filter(|u| {
                    state
                        .upgrades
                        .contains(u.name.as_str())
                        .map_or_else(|| true, |c| c < u.max_count)
                })
                .nth(i - 1)
            {
                if state.inventory.get_amount("Gold") >= upgrade.cost {
                    state.inventory.remove("Gold", upgrade.cost);
                    state.upgrades.buy(upgrade.name.as_str());
                    (upgrade.apply)(state);
                    message.add_message(Message {
                        text: format!(
                            "You bought {} for {} gold. {}/{}",
                            upgrade.name,
                            upgrade.cost,
                            state.upgrades.contains(upgrade.name.as_str()).unwrap_or(0),
                            upgrade.max_count
                        )
                        .green(),
                        location: message::TextLocation::Center,
                        duration: 5.0,
                    });
                } else {
                    message.add_message(Message {
                        text: "You don't have enough gold".to_string().red(),
                        location: message::TextLocation::Center,
                        duration: 3.0,
                    });
                }
            }
        }
    }

    fn render(
        &self,
        state: &state::State,
        _features: &Vec<Box<dyn Feature>>,
    ) -> Vec<crossterm::style::StyledContent<String>> {
        let mut lines = Vec::new();

        get_all_upgrades()
            .iter()
            .filter(|u| {
                state
                    .upgrades
                    .contains(u.name.as_str())
                    .map_or_else(|| true, |c| c < u.max_count)
            })
            .take(10)
            .enumerate()
            .for_each(|(i, u)| {
                lines.push(
                    format!(
                        "({}) |{}/{}| [{}] <{}> {}",
                        i + 1,
                        state.upgrades.contains(u.name.as_str()).unwrap_or(0),
                        u.max_count,
                        u.cost,
                        rarity_stylize(u.rarity, u.name.as_str()),
                        u.description
                    )
                    .stylize(),
                );
            });
        lines
    }
}
