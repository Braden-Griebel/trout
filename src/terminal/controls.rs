use crossterm::cursor::{Hide, MoveTo, Show, SetCursorStyle};
use crossterm::style::Print;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType,
                          EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{queue, Command};
use std::io::{stdout, Error, Write};


/// Struct representing the current size of the visible screen
pub struct Size {
    pub height: usize,
    pub width: usize,
}

/// Struct representing a location on the screen
pub struct ScreenLocation {
    pub row: usize,
    pub col: usize,
}

impl ScreenLocation {
    pub fn default()->Self {
        Self {row:0, col:0}
    }
}

/// Represents the Terminal, and implements methods for interacting
/// with the terminal more easily
pub struct Terminal;

impl Terminal {
    /// End the current terminal session, leaving alternate screen, and ensuring caret isn't hidden
    pub fn terminate() -> Result<(), Error> {
        Self::leave_alternate_screen()?;
        Self::show_caret()?;
        Self::execute()?;
        disable_raw_mode()?;
        Ok(())
    }

    /// Begin terminal session, entering alternate screen and clearing it
    pub fn initialize() -> Result<(), Error> {
        enable_raw_mode()?;
        Self::enter_alternate_screen()?;
        Self::clear_screen()?;
        Self::execute()?;
        Ok(())
    }

    /// Clear the current screen
    pub fn clear_screen() -> Result<(), Error> {
        Self::queue_command(Clear(ClearType::All))?;
        Ok(())
    }

    /// Clear the current line
    pub fn clear_line() -> Result<(), Error> {
        Self::queue_command(Clear(ClearType::CurrentLine))?;
        Ok(())
    }

    /// Move the Caret/Cursor to specified screen location
    pub fn move_caret_to(position: ScreenLocation)->Result<(), Error>{
        Self::queue_command(MoveTo(position.col as u16, position.row as u16))?;
        Ok(())
    }

    /// Enter an alternate screen
    pub fn enter_alternate_screen() -> Result<(), Error> {
        Self::queue_command(EnterAlternateScreen)?;
        Ok(())
    }

    /// Leave the alternate screen
    pub fn leave_alternate_screen() -> Result<(), Error> {
        Self::queue_command(LeaveAlternateScreen)?;
        Ok(())
    }

    /// Hide the caret/cursor
    pub fn hide_caret() ->Result<(), Error> {
        Self::queue_command(Hide)?;
        Ok(())
    }

    /// Show the caret/cursor
    pub fn show_caret() -> Result<(), Error> {
        Self::queue_command(Show)?;
        Ok(())
    }

    /// Print a string at the current location
    pub fn print(string: &str) -> Result<(), Error> {
        Self::queue_command(Print(string))?;
        Ok(())
    }

    /// Print a string to a particular row
    pub fn print_row(row:usize, line_text: &str)->Result<(), Error> {
        Self::move_caret_to(ScreenLocation{row, col:0})?;
        Self::clear_line()?;
        Self::print(line_text)?;
        Ok(())
    }

    /// Get the current size of the terminal
    pub fn size() -> Result<Size, Error> {
        let (width, height) = size()?;
        let width = width as usize;
        let height = height as usize;
        Ok(Size {height, width})
    }

    /// Set the Cursor to be a steady bar
    pub fn bar_cursor()->Result<(), Error>{
        Self::queue_command(SetCursorStyle::SteadyBar)?;
        Ok(())
    }

    pub fn blinking_block_cursor()-> Result<(), Error>{
        Self::queue_command(SetCursorStyle::BlinkingBlock)?;
        Ok(())
    }

    /// Execute the queued commands
    pub fn execute() -> Result<(), Error> {
        stdout().flush()?;
        Ok(())
    }

    /// Add a command to the Command Queue
    fn queue_command<T:Command>(command:T) -> Result<(), Error> {
        queue!(stdout(), command)?;
        Ok(())
    }
}
