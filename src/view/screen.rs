use std::io::Error;
use std::path::PathBuf;

use once_cell::sync::Lazy;
use regex::Regex;
use crate::editor::EditorAction;
use crate::terminal::controls::{Size, Terminal};
use crate::terminal::screen_location::ScreenLocation;
use crate::textbuffer::buffer::Buffer;
use crate::textbuffer::text_location::TextPosition;

/// Struct representing the currently viewed screen
pub struct Screen {
    /// Buffer which holds the text to display
    pub buffer: Buffer,
    /// Location of caret on the screen
    pub screen_location: ScreenLocation,
    /// Location of the cursor within the text
    pub text_position: TextPosition,
    /// Offset of current view from 0,0
    pub scroll_offset: ScreenLocation,
    /// Edges of the buffer area
    pub inner_boundary: Boundary,
    /// Size of the terminal window
    pub size: Size,
    /// Current mode
    pub mode: Mode,
    /// Welcome Screen toggle
    pub welcome_screen: bool,
    /// Flag for whether the current screen should close
    pub quit_screen: bool,
}

impl Screen {
    /// Create a default instance of Screen
    pub fn default() -> Screen {
        let size = Terminal::size().unwrap_or_else(|_| Size { height: 0, width: 0 });
        Self {
            buffer:Buffer::empty(),
            screen_location:ScreenLocation::default(),
            text_position: TextPosition::default(),
            scroll_offset: ScreenLocation::default(),
            inner_boundary: Boundary::default(),
            mode: Mode::Normal,
            size,
            welcome_screen: false,
            quit_screen: false,
        }
    }

    /// Create a instance of Screen showing the Welcome Screen view
    pub fn welcome()->Screen{
        let mut welcome_screen = Self::default();
        welcome_screen.welcome_screen = true;
        welcome_screen
    }

    /// Reads a file
    pub fn load_file(&mut self, file_path:PathBuf){
        self.buffer = Buffer::from_file(file_path);
    }

    /// Runs the current screen
    pub fn run(&mut self)->EditorAction{
        loop {
            if self.quit_screen{
                return EditorAction::QuitScreen;
            }
        }
    }

    /// Move the caret cursor one line up
    pub fn move_up(&mut self)-> Result<(), Error>{
        // Move the text position up a line, unless already at 0
        if self.text_position.row > 0{
            self.text_position.row -= 1;
        }
        self.sync_text_position_byte_to_grapheme();
        // Move the cursor location onto screen
        self.scroll_into_view()?;
        // Move the caret to the correct position
        Terminal::move_caret_to(self.screen_location.clone())?;
        Ok(())
    }

    /// Move the caret and cursor down one line
    pub fn move_down(&mut self)->Result<(), Error>{
        // Move the text position down a line, if there are more lines in the buffer
        if self.text_position.row < self.buffer.num_lines.saturating_sub(1){
            self.text_position.row +=1;
        }
        self.sync_text_position_byte_to_grapheme();
        // Move the cursor location onto screen
        self.scroll_into_view()?;
        // Move the caret to the correct position
        Terminal::move_caret_to(self.screen_location.clone())?;
        Ok(())
    }

    fn sync_text_position_byte_to_grapheme(&mut self){
        // Make sure the cursor isn't past the last character
        if self.text_position.grapheme >= self.buffer.text[self.text_position.row].grapheme_count{
            self.text_position.grapheme = self.buffer.text[self.text_position.row].grapheme_count-1
        }
        self.text_position.byte = self.buffer.text[self.text_position.row]
            .grapheme_start(self.text_position.grapheme);
    }

    /// Move the caret cursor one column left
    pub fn move_left(&mut self)-> Result<(), Error>{
        // Move the text position left a column, unless at the start of a line
        if self.text_position.grapheme > 0 {
            self.text_position.grapheme = self.text_position.grapheme.saturating_sub(1);
            self.text_position.byte =
                self.buffer.text[self.text_position.row]
                    .grapheme_start(self.text_position.grapheme);
        }
        // Move cursor location onto screen
        self.scroll_into_view()?;
        // Move the caret to the correct position
        Terminal::move_caret_to(self.screen_location.clone())?;
        Ok(())
    }

    /// Move the caret cursor one column right
    pub fn move_right(&mut self)-> Result<(), Error>{
        // Move the text position right a column, unless at the end of a line
        if self.text_position.grapheme < self.buffer
            .text[self.text_position.row]
            .grapheme_count
            .saturating_sub(1){
            self.text_position.grapheme = self.text_position.grapheme.saturating_add(1);
            self.text_position.byte =
                self.buffer.text[self.text_position.row]
                    .grapheme_start(self.text_position.grapheme);
        }
        // Move cursor location onto screen
        self.scroll_into_view()?;
        // Move the caret to the correct position
        Terminal::move_caret_to(self.screen_location.clone())?;
        Ok(())
    }

    /// Move the caret/cursor to the last grapheme of a line
    pub fn move_end_line(&mut self)->Result<(), Error>{
        // Move the text position to the end of the current line
        let line_length = self.buffer.text[self.text_position.row].grapheme_count;
        if line_length > 0 {
            self.text_position.grapheme = line_length-1;
        }
        self.scroll_into_view()?;
        Terminal::move_caret_to(self.screen_location.clone())?;
        Ok(())
    }

    /// Move the caret/cursor to the first grapheme of a line
    pub fn move_start_line(&mut self)->Result<(), Error>{
        self.text_position.grapheme=0;
        self.scroll_into_view()?;
        Terminal::move_caret_to(self.screen_location.clone())?;
        Ok(())
    }

    /// Move the cursor to the first line of a buffer
    pub fn move_first_line(&mut self)->Result<(), Error>{
        self.text_position.row=0;
        self.sync_text_position_byte_to_grapheme();
        self.scroll_into_view()?;
        Terminal::move_caret_to(self.screen_location.clone())?;
        Ok(())
    }

    /// Move the caret/cursor to the last line of a buffer
    pub fn move_last_line(&mut self)->Result<(), Error>{
        self.text_position.row = self.buffer.num_lines.saturating_sub(1);
        self.sync_text_position_byte_to_grapheme();
        self.scroll_into_view()?;
        Terminal::move_caret_to(self.screen_location.clone())?;
        Ok(())
    }

    /// Move the caret/cursor to the next word of a buffer
    pub fn move_next_word(&mut self)->Result<(), Error>{
        // Regex for recognizing a word
        static WORD_REGEX:Lazy<Regex> = Lazy::new(|| Regex::new(r"\w|[(){}\-+&=]").unwrap());
        match WORD_REGEX.find(&self.buffer
            .text[self.text_position.row]
            .text[self.text_position.byte..]){
            None => {
                // If no match found on this line, loop through any remaining line to see
                // if a match can be found
                for cur_line in self.text_position.row..self.buffer.num_lines{
                    match WORD_REGEX.find(&self.buffer.text[cur_line].text) {
                        None=>{},//do nothing
                        Some(m)=>{
                            let start = m.start();
                            self.text_position.byte = start;
                            self.text_position.grapheme = self.buffer.text[self.text_position.row].text_index_to_grapheme(start);
                            break;// Found needed match, stop loop
                        }
                    }
                }
            }
            Some(m) => {
                let start = m.start();
                self.text_position.byte = start;
                self.text_position.grapheme = self.buffer.text[self.text_position.row].text_index_to_grapheme(start);
            }
        }
        self.scroll_into_view()?;
        Terminal::move_caret_to(self.screen_location.clone())?;
        Ok(())
    }

    /// Move the caret/cursor to the previous word of a buffer
    pub fn move_prev_word(&mut self)->Result<(), Error>{
        static WORD_REGEX:Lazy<Regex> = Lazy::new(|| Regex::new(r"\w|[(){}\-+&=]").unwrap());
        match WORD_REGEX.find_iter(&self.buffer
            .text[self.text_position.row]
            .text[..self.text_position.byte]).last(){
            None=>{
                // If no match found, try looping through previous lines to find a match
                for cur_line in (0..=self.text_position.row).rev(){
                    match WORD_REGEX.find_iter(&self.buffer.text[cur_line].text).last(){
                        None=>{},
                        Some(m)=>{
                            let start = m.start();
                            self.text_position.byte = start;
                            self.text_position.grapheme = self.buffer.text[self.text_position.row].text_index_to_grapheme(start);
                            break;// Found needed match, stop loop
                        }
                    }
                }
            },
            Some(m)=>{
                let start = m.start();
                self.text_position.byte = start;
                self.text_position.grapheme = self.buffer
                    .text[self.text_position.row].text_index_to_grapheme(start);
            }
        }
        // It's okay if no match is found, just leave the cursors and positions alone
        self.scroll_into_view()?;
        Terminal::move_caret_to(self.screen_location.clone())?;
        Ok(())
    }


    /// Updates the scroll offset and caret_position so the text_position is on screen
    pub fn scroll_into_view(&mut self)->Result<(), Error>{
        self.scroll_vertical();
        self.scroll_horizontal();
        self.sync_screen_position();
        Ok(())
    }

    /// Delete the grapheme at the text position
    pub fn delete_grapheme(&mut self, location: TextPosition){
        self.buffer.delete_char(location.row, location.grapheme)
    }

    fn scroll_horizontal(&mut self){
        // If the text position is too far right, move the scroll offset right
        if self.text_position.grapheme.saturating_sub(self.scroll_offset.col) > self.view_width(){
            self.scroll_offset.col = self.text_position.grapheme-self.scroll_offset.col - self.view_width();
        } else if self.text_position.grapheme < self.scroll_offset.col{
            // The cursor is too far left, move the scroll offset to the left
            self.scroll_offset.col = self.text_position.grapheme;
        }
    }

    fn scroll_vertical(&mut self){
        // if the text position is too far down, move the scroll offset up
        if self.text_position.row.saturating_sub(self.scroll_offset.row) > self.view_height(){
            self.scroll_offset.row = self.text_position.row-self.scroll_offset.row - self.view_height();
        } else if self.text_position.row < self.scroll_offset.row {
            self.scroll_offset.row = self.text_position.row;
        }
    }

    /// Syncs the positions of the caret and the cursor
    fn sync_screen_position(&mut self) {
        self.screen_location.col = self.text_position.grapheme - self.scroll_offset.col + self.inner_boundary.left;
        self.screen_location.row = self.text_position.row - self.scroll_offset.row + self.inner_boundary.top;
    }

    pub fn view_width(&self)->usize{
        self.size.width - self.inner_boundary.left -self.inner_boundary.right
    }

    pub fn view_height(&self)->usize{
        self.size.height - self.inner_boundary.top - self.inner_boundary.bottom
    }

}


/// Represents rows/columns of padding on each of the edges
pub struct Boundary {
    top: usize,
    right: usize,
    pub(crate) left: usize,
    bottom: usize,
}

impl Boundary {
    pub fn default()-> Boundary{
        Self {
            top: 0,
            right:0,
            left:4, // To account for line numbers
            bottom:2, // For status line and command entry line
        }
    }
}

/// Enum Representing the current mode of the editor
#[derive(Clone, Debug)]
pub enum Mode {
    Normal,
    Insert,
    Jump,
    Command,
    Find,
    Open,
    Select,
}

/// Represents next action for screen to take
#[derive(Clone, Debug)]
pub enum ScreenAction {
    /// Enter a default view mode
    EnterMode(Mode),
    /// Quit this screen, and tell the editor to
    /// open a default screen with the PathBuf file
    OpenScreen(PathBuf),
    /// Close the current screen and open the next screen
    QuitScreen,
    /// Close the editor
    QuitEditor,
}