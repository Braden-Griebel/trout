use std::path::PathBuf;
use crate::terminal::controls::{ScreenLocation, Size, Terminal};
use crate::textbuffer::buffer::Buffer;

/// Struct representing the currently viewed screen
pub struct Screen {
    /// Buffer which holds the text to display
    buffer: Option<Buffer>, // Option because welcome screen, selection screen etc. are empty
    /// Location of caret on the screen
    screen_location: ScreenLocation,
    /// Location of the cursor within the text
    text_position: TextPosition,
    /// Offset of current view from 0,0
    scroll_offset: ScreenLocation,
    /// Edges of the buffer area
    inner_boundary: Boundary,
    /// Size of the terminal window
    size: Size,
    /// Current mode
    mode: Mode,
    /// Flag for whether the current screen should close
    quit_screen: bool,
}

impl Screen {
    /// Create a new instance of Screen
    pub fn new()-> Screen {
        let size = Terminal::size().unwrap_or_else(|_| Size { height: 0, width: 0 });
        Self {
            buffer:None,
            screen_location:ScreenLocation::default(),
            text_position: TextPosition::default(),
            scroll_offset: ScreenLocation::default(),
            inner_boundary: Boundary::default(),
            mode: Mode::Normal,
            size,
            quit_screen: false,
        }
    }

    /// Reads a file
    pub fn load_file(&mut self, file_path:PathBuf){
        self.buffer = Some(Buffer::from_file(file_path));
    }

    /// Runs the current screen
    pub fn run(&mut self){
        loop {
            if self.quit_screen{
                break;
            }
        }
    }

    pub fn move_down(&mut self){

    }
}

pub struct TextPosition {
    row: usize,
    byte: usize,
    grapheme: usize,
}

impl TextPosition {
    pub fn default()-> Self {
        Self{row:0, byte:0, grapheme:0}
    }
}


/// Represents rows/columns of padding on each of the edges
pub struct Boundary {
    top: usize,
    right: usize,
    left: usize,
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
    /// Enter a new view mode
    EnterMode(Mode),
    /// Quit this screen, and tell the editor to
    /// open a new screen with the PathBuf file
    OpenScreen(PathBuf),
    QuitScreen,

}