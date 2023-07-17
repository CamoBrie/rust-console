use core::panic;
use std::iter;

use crossterm::{
    csi,
    cursor::MoveToNextLine,
    terminal::{Clear, ClearType},
    Command,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrintAll<T: std::fmt::Display>(pub Vec<T>);

impl<T: std::fmt::Display> Command for PrintAll<T> {
    fn write_ansi(&self, f: &mut impl std::fmt::Write) -> std::fmt::Result {
        self.0.iter().try_for_each(|t| write!(f, "{}", t))
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> Result<(), std::io::Error> {
        panic!("tried to execute Print command using WinAPI, use ANSI instead");
    }

    #[cfg(windows)]
    fn is_ansi_code_supported(&self) -> bool {
        true
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrintAllLines<T: std::fmt::Display>(pub Vec<T>);

impl<T: std::fmt::Display> Command for PrintAllLines<T> {
    fn write_ansi(&self, f: &mut impl std::fmt::Write) -> std::fmt::Result {
        self.0.iter().try_for_each(|t| {
            write!(f, "{}", t)?;
            Clear(ClearType::UntilNewLine).write_ansi(f)?;
            MoveToNextLine(1).write_ansi(f)
        })
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> Result<(), std::io::Error> {
        panic!("tried to execute Print command using WinAPI, use ANSI instead");
    }

    #[cfg(windows)]
    fn is_ansi_code_supported(&self) -> bool {
        true
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Divider(pub char);

impl Command for Divider {
    fn write_ansi(&self, f: &mut impl std::fmt::Write) -> std::fmt::Result {
        write!(f, csi!("0G"))?;
        if let Ok((size, _)) = crossterm::terminal::size() {
            write!(
                f,
                "{}",
                iter::repeat(self.0).take(size as usize).collect::<String>()
            )
        } else {
            panic!("Failed to get terminal size");
        }
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> Result<(), std::io::Error> {
        panic!("tried to execute Print command using WinAPI, use ANSI instead");
    }

    #[cfg(windows)]
    fn is_ansi_code_supported(&self) -> bool {
        true
    }
}
