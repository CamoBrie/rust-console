use crossterm::style::Stylize;

use crate::feature::inventory::Rarity;

/// Stylize a string based on its rarity
pub fn rarity_stylize(rarity: Rarity, string: &str) -> String {
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
