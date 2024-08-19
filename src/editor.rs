use crate::view::screen::Screen;
use std::path::{Path, PathBuf};

use std::panic::{set_hook, take_hook};
use crate::view::screen::Mode;
use crate::terminal::controls::Terminal;

/// Main editor struct, which manages the user facing behavior
pub(crate) struct Editor {
    screens: Vec<Screen>,
    should_quit: bool,
    mode: Mode,
    current_screen: usize,
}

impl Editor {
    pub fn new(path: Option<&Path>)-> Editor{
        // Ensure that on panic, the terminal shuts down nicely
        let current_hook = take_hook();
        set_hook(Box::new(move |panic_info| {
            let _ = Terminal::terminate();
            current_hook(panic_info);
        }));
        // Create a default terminal session, entering raw mode, on an alternate screen, and clearing it
        _=Terminal::initialize();
        Editor {
            screens: Vec::new(),
            should_quit: false,
            mode: Mode::Normal,
            current_screen: 0,
        }
    }

    pub fn open_file(&mut self, file_path:PathBuf){
        self.screens.push(Screen::default());
        self.current_screen = self.screens.len()-1;
        self.screens[self.current_screen].load_file(file_path);
    }
}

/// Enum used for telling the editor what to do next, returned from a mode's run method
pub enum EditorAction {
    /// Change to the screen specified by the usize
    ChangeScreen(usize),
    /// Open a new screen with the provided path
    NewScreen(PathBuf),
    /// Open a new welcome screen
    NewWelcomeScreen,
    /// Quit the current screen (closing it without saving)
    QuitScreen,
}


