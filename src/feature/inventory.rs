use std::default;

use crossterm::{
    event::KeyCode,
    style::{StyledContent, Stylize},
};
use enum_iterator::Sequence;

use crate::{
    feature::Feature,
    state::State,
    util::flag::{Flag, Flags},
};

#[derive(Eq, PartialEq, Copy, Clone, Debug, Sequence)]
pub enum Rarity {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
    Mythic,
}

pub struct Item {
    pub name: String,
    pub description: String,
    pub amount: u64,
    pub rarity: Rarity,
}

pub struct Inventory {
    items: Vec<Item>,
    cur_size: u32,
    max_size: u32,
}

impl Default for Inventory {
    fn default() -> Self {
        Self {
            items: vec![],
            cur_size: 0,
            max_size: 10,
        }
    }
}

impl Inventory {
    pub fn add(&mut self, name: &str, amount: u64) {
        if let Some(item) = get_item(name) {
            if let Some(existing_item) = self.items.iter_mut().find(|i| i.name == name) {
                existing_item.amount += amount;
            } else {
                if self.cur_size + 1 <= self.max_size {
                    self.cur_size += 1;
                    self.items.push(item);
                    self.add(name, amount);
                }
            }
        }
    }

    pub fn remove(&mut self, name: &str, amount: u64) {
        if let Some(existing_item) = self.items.iter_mut().find(|i| i.name == name) {
            existing_item.amount -= amount;
            if existing_item.amount <= 0 {
                self.items.retain(|i| i.name != name);
                self.cur_size -= 1;
            }
        }
    }

    pub fn get(&self, name: &str) -> Option<&Item> {
        self.items.iter().find(|i| i.name == name)
    }

    pub fn get_mut(&mut self, name: &str) -> Option<&mut Item> {
        self.items.iter_mut().find(|i| i.name == name)
    }

    pub fn contains(&self, name: &str) -> bool {
        self.items.iter().any(|i| i.name == name)
    }

    /// Get the amount of an item in the inventory. or default 0
    pub fn get_amount(&self, name: &str) -> u64 {
        if let Some(item) = self.items.iter().find(|i| i.name == name) {
            item.amount
        } else {
            0
        }
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Debug, Sequence)]
enum InventoryFlag {
    ShowDetailed,
}

impl Flag<State> for InventoryFlag {
    fn handle(&self, _flags: &mut Flags<Self, State>, _state: &mut State) {}
}

pub struct InventoryFeature {
    flags: Flags<InventoryFlag, State>,
}

impl default::Default for InventoryFeature {
    fn default() -> Self {
        Self {
            flags: Flags::new(),
        }
    }
}

impl Feature for InventoryFeature {
    fn get_key(&self) -> KeyCode {
        KeyCode::Char('i')
    }

    fn get_name(&self) -> StyledContent<&str> {
        "Inventory".bold().white().on_black()
    }

    fn get_top_bar(&self, state: &State) -> Vec<StyledContent<String>> {
        vec![format!(
            " | Items: {}/{}",
            state.inventory.items.len(),
            state.inventory.max_size
        )
        .bold()]
    }

    fn get_description(&self) -> StyledContent<&str> {
        "View your inventory".italic().white()
    }

    fn counter_data(&self) -> (i32, i32, StyledContent<&str>) {
        (100, 0, " and more than 1 gold".stylize())
    }

    fn is_unlocked(&self, state: &State) -> bool {
        state.inventory.items.len() > 0
    }

    fn update(&mut self, _ms_step: f32, state: &mut State) {
        process_input(self, state.key, state);
    }

    fn render(&self, state: &State, _: &Vec<Box<dyn Feature>>) -> Vec<StyledContent<String>> {
        state
            .inventory
            .items
            .iter()
            .map(|item| {
                format!(
                    "[{}] {} {}",
                    item.amount,
                    rarity_stylize(item.rarity, item.name.as_str()),
                    if self.flags.is_marked(&InventoryFlag::ShowDetailed) {
                        item.description.as_str()
                    } else {
                        ""
                    }
                )
                .stylize()
            })
            .collect()
    }
}

fn process_input(inv: &mut InventoryFeature, key: KeyCode, _state: &mut State) {
    match key {
        KeyCode::Char('d') => {
            if inv.flags.is_marked(&InventoryFlag::ShowDetailed) {
                inv.flags.unmark(InventoryFlag::ShowDetailed)
            } else {
                inv.flags.mark(InventoryFlag::ShowDetailed)
            }
        }
        _ => {}
    }
}

fn rarity_stylize(rarity: Rarity, string: &str) -> String {
    match rarity {
        Rarity::Common => string.stylize().white(),
        Rarity::Uncommon => string.stylize().green(),
        Rarity::Rare => string.stylize().blue(),
        Rarity::Epic => string.stylize().magenta(),
        Rarity::Legendary => string.stylize().yellow(),
        Rarity::Mythic => string.stylize().red().bold(),
    }
    .to_string()
}

fn get_item(name: &str) -> Option<Item> {
    match name {
        "Gold" => Some(Item {
            name: "Gold".to_string(),
            description: "A shiny coin".to_string(),
            amount: 0,
            rarity: Rarity::Common,
        }),
        "XP" => Some(Item {
            name: "XP".to_string(),
            description: "Experience Points".to_string(),
            amount: 0,
            rarity: Rarity::Uncommon,
        }),
        _ => None,
    }
}
