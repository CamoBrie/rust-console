use std::{collections::VecDeque, io::stdout};

use crossterm::{
    cursor::MoveTo,
    event::KeyCode,
    queue,
    style::{Print, StyledContent},
};

use crate::util::conv::wrap;

/// Different locations for text to be rendered
pub enum TextLocation {
    Center,
    Bottom,
}

/// A message to be displayed
pub struct Message {
    pub text: StyledContent<&'static str>,
    pub location: TextLocation,
    pub duration: f32,
}

/// A manager for messages
/// It keeps track of messages and their timers, and renders them
pub struct MessageManager {
    messages: VecDeque<Message>,
}

impl Default for MessageManager {
    fn default() -> Self {
        Self {
            messages: VecDeque::new(),
        }
    }
}

impl MessageManager {
    /// Add a message to the queue
    pub fn add_message(&mut self, message: Message) {
        self.messages.push_back(message);
    }

    /// Update the timers on all messages, removing any that have expired
    pub fn update(&mut self, key: KeyCode, delta: f32) {
        if let Some(message) = self.messages.front_mut() {
            message.duration -= delta;
            if message.duration <= 0.0 || key == KeyCode::Enter {
                self.messages.pop_front();
            }
        }
    }

    /// Render the first message in the queue
    pub fn render_one(&mut self) {
        if let Some(message) = self.messages.front() {
            let (t_c, t_r) = crossterm::terminal::size().unwrap_or((80, 24));
            let lines = wrap(message.text, t_c / 2);

            lines.iter().enumerate().for_each(|(i, (len, line))| {
                queue!(
                    stdout(),
                    match message.location {
                        TextLocation::Center => {
                            let col = t_c / 2 - *len / 2;
                            let row = t_r / 2 + i as u16;

                            MoveTo(col, row)
                        }
                        TextLocation::Bottom => {
                            let col = t_c / 2 - *len / 2;
                            let row = t_r - (lines.len() - i) as u16;
                            MoveTo(col, row)
                        }
                    },
                    Print(line)
                )
                .expect("Failed to queue message render")
            });
        }
    }
}
