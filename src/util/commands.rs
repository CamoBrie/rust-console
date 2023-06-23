use crossterm::Command;

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