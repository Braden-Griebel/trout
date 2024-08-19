use std::fs::{File, read_to_string};
use std::io::{Error, Write};
use std::path::PathBuf;
use regex::{Match, Regex};
use crate::textbuffer::lines::Line;
use crate::textbuffer::text_location::TextPosition;

/// A text buffer, representing a collection of lines of text
pub struct Buffer {
    /// A vector of lines representing the text
    pub text: Vec<Line>,
    /// The file extension (used for syntax highlighting)
    pub extension: Option<String>,
    /// Path to where to output the buffer
    pub path: PathBuf,
    /// Number of lines within the buffer
    pub num_lines: usize,
    /// Current line for iterator
    cur_line: usize
}

impl Iterator for Buffer {
    type Item = Line;

    fn next(&mut self) -> Option<Self::Item> {
        let cur = self.cur_line;
        self.cur_line+=1;
        if cur >= self.num_lines{
            return None
        }
        Some(self.text[cur].clone())
    }
}

impl Buffer {
    /// Create an empty buffer
    pub fn empty()->Buffer{
        Self{
            text: Vec::new(),
            extension: None,
            path: PathBuf::new(),
            num_lines: 0,
            cur_line:0,
        }
    }

    /// Create a buffer from a file
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
        let num_lines = text.len();
        Self {
            text,
            extension,
            path: file_path,
            num_lines,
            cur_line:0,
        }
    }

    /// Write the current buffer to the file it is targeting
    pub fn write_file(&self)->Result<(), Error>{
        let mut file = File::create(&self.path)?;
        file.write_all(self.lines_to_str().as_bytes())?;
        Ok(())
    }

    /// Insert a (utf8) character into a line of the text, at grapheme_index
    pub fn insert_char(&mut self, line:usize, grapheme_index: usize, character:char){
        self.text[line].insert_char(grapheme_index, character);
    }

    /// Delete a (utf-8) character at the grapheme_index, if the line is already empty,
    /// then this will instead delete that line
    pub fn delete_char(&mut self, line:usize, grapheme_index: usize){
        if self.text[line].text == ""{
            _=self.text.remove(line);
            self.num_lines-=1;
        } else {
            self.text[line].delete_grapheme(grapheme_index)
        }
    }

    /// Create a default line, potentially splitting a line into two parts
    pub fn new_line(&mut self, line:usize, grapheme_index: usize){
        if line >= self.num_lines{
            self.text.push(Line::from_string(""));
            self.num_lines+=1;
        } else {
            let new_line = self.text[line].split_line_grapheme(grapheme_index);
            self.text.insert(line, new_line);
            self.num_lines+=1;
        }
    }

    /// Copy text form the start position to the end position
    pub fn copy_text(&self, start_position: TextPosition, end_position: TextPosition)->String{
        if start_position.row == end_position.row {
            // Only on one line, simplest case
            let start_byte = self.text[start_position.row].grapheme_start(start_position.grapheme);
            let end_byte = self.text[end_position.row].grapheme_end(end_position.grapheme);
            let mut copied_string = String::new();
            self.text[start_position.row].text[start_byte..=end_byte].clone_into(&mut copied_string);
            return copied_string;
        }
        let mut copied_lines:Vec<&str> = Vec::new();
        let start_byte = self.text[start_position.row].grapheme_start(start_position.grapheme);
        let end_byte = self.text[end_position.row].grapheme_end(end_position.grapheme);
        copied_lines.push(&self.text[start_position.row].text[start_byte..]);
        for idx in (start_position.row+1)..end_position.row{
            copied_lines.push(&self.text[idx].text[..]);
        }
        copied_lines.push(&self.text[start_position.row].text[..=end_byte]);
        copied_lines.join("\n").to_string()
    }

    /// Paste text at start position
    pub fn paste_text(&mut self, start_position:TextPosition, insert_str: &str){
        // This is a really inefficient way of doing this, but its a lot simpler than
        // alternatives
        self.text[start_position.row].insert_str(start_position.grapheme, insert_str);
        self.fix_newlines();
    }

    /// Return a &str for printing (optionally highlighted, not yet implemented)
    pub fn print_line(&mut self, line: usize,
                      start_grapheme: usize,
                      end_grapheme: usize,
                      highlighted: bool)->&str{
        let mut end_g = end_grapheme;
        // If the start grapheme is beyond the text, just return an empty string
        if start_grapheme >= self.text[line].grapheme_count-1{
            return ""
        }
        // If the end grapheme is beyond the text, set the end grapheme to be the last grapheme
        // in the text
        if end_grapheme >= self.text[line].grapheme_count - 1{
            end_g = self.text[line].grapheme_count -1;
        }
        let start_byte = self.text[line].grapheme_start(start_grapheme);
        let end_byte = self.text[line].grapheme_start(end_g);
        &self.text[line].text[start_byte..=end_byte]
    }

    fn fix_newlines(&mut self){
        if self.text.len() == 0usize {
            return;
        }
        let new_line_regex = Regex::new("\n").unwrap();
        let mut idx = 0usize;
        loop{
            let start = match new_line_regex.find(&self.text[idx].text){
                None=>None,
                Some(m)=>Some(m.start())
            };
            match start{
                None => {}
                Some(s) => {
                    // Find the newline character, and split the line there
                    let mut newline = self.text[idx].split_line(s);
                    // Delete the newline character at the start
                    newline.delete_grapheme(0);
                    // Insert this new line next in the buffer
                    self.text.insert(idx+1, newline);
                    self.num_lines+=1;
                }
            }
            idx+=1;
            if idx >= self.text.len(){
                break;
            }
        }
    }

    fn lines_to_str(&self)-> String{
        let mut out_str = String::new();
        for idx in 0..self.num_lines {
            out_str.push_str(&self.text[idx].text);
            out_str.push('\n');
        }
        out_str
    }
}
