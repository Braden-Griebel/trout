use std::io::{stdout, Write};
use crate::view::splash_art::SplashArt;
use crossterm::event::{Event, read, KeyCode, KeyEvent, KeyModifiers, ModifierKeyCode, KeyEventKind};

mod editor;
use editor::Editor;
mod view;
mod textbuffer;
mod commands;
mod terminal;
mod input;

fn main() {
   //let editor = Editor::new();
}
