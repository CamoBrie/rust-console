use crossterm::{event::KeyCode, style::StyledContent};

/// Get a string representation of a key
pub fn get_string(key: KeyCode) -> String {
    match key {
        KeyCode::Char(c) => format!("{}", c),
        KeyCode::Esc => "Esc".to_string(),
        KeyCode::Left => "<-".to_string(),
        KeyCode::Right => "->".to_string(),
        KeyCode::Null => "".to_string(),
        _ => "?".to_string(),
    }
}

/// Wraps a string to a certain width, returning a vector of lines and their lengths
pub fn wrap(text: StyledContent<&str>, width: u16) -> Vec<(u16, String)> {
    let mut lines = Vec::new();
    let mut line: String = String::new();
    let mut len: u16 = 0;

    for word in text.to_string().split_whitespace() {
        if len + word.len() as u16 > width {
            lines.push((len, line));
            line = String::new();
            len = 0;
        }

        line.push_str(word);
        line.push(' ');
        len += word.len() as u16 + 1;
    }

    lines.push((len, line));

    lines
}
