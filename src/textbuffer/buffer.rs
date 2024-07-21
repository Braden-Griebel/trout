use std::fs::read_to_string;
use std::path::PathBuf;

use crate::textbuffer::lines::Line;

/// A text buffer, representing a collection of lines of text
pub struct Buffer {
    /// A vector of lines representing the text
    text: Vec<Line>,
    /// The file extension (used for syntax highlighting)
    extension: Option<String>,
    /// Path to where to output the buffer
    path: PathBuf
}

impl Buffer {
    pub fn from_file(file_path: PathBuf)->Buffer{
        let mut file_str = "".to_string();
        if file_path.exists() {
            file_str = match read_to_string(&file_path) {
                Ok(result) => { result }
                Err(_)=> "".to_string() // If it doesn't exist, just set this to an empty string
            };
        }
        let mut text: Vec<Line> = Vec::new();
        for line in file_str.lines(){
            text.push(Line::from_string(line))
        }
        let extension = match file_path.extension(){
            None => {None}
            Some(ext) => {Some(ext.to_str().unwrap_or("").to_string())}
        };
        Self {
            text,
            extension,
            path: file_path
        }
    }

    /// Write the current buffer to the file it is targeting
    pub fn write_file(&self){}

    pub fn insert_char(&mut self, line:usize, grapheme_index: usize, character:char){
        self.text[line].insert_char(grapheme_index, character);
    }
}
