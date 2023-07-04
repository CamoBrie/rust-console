use crossterm::event::KeyCode;

/// Get a string representation of a key
pub fn get_string(key: KeyCode) -> String {
    match key {
        KeyCode::Char(c) => format!("{}", c),
        KeyCode::Esc => "Esc".to_string(),
        KeyCode::Left => "<-".to_string(),
        KeyCode::Right => "->".to_string(),
        _ => "?".to_string(),
    }
}
