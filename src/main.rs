use std::io::Write;
use regex::Regex;

mod editor;
mod view;
mod textbuffer;
mod commands;
mod terminal;
mod input;

fn main() {
   //let editor = Editor::default();
    let word_regex = Regex::new(r"\w|[(){}\-+&=]").unwrap();
    let test = "fn test_function{println!(\"Hello World\")}";
    match word_regex.find_iter(test).last(){
        None => {}
        Some(m) => {println!("{}", m.start())}
    }
}
