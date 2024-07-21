use std::ops::Deref;
use crate::view::screen::Mode;
use crate::input::keymap::KeyMap;
use crate::commands::actions::{ActionType, ActionParam, Action};
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers, ModifierKeyCode, read};

/// Handles keypress events
struct KeyReader {
    key_map: KeyMap,
    input_buffer: String,
}

impl KeyReader {
    pub fn read_input(&mut self, key_event: KeyEvent, mode: Mode) -> Option<Action> {
        match mode {
            Mode::Normal => {self.normal_mode(key_event)}
            Mode::Insert => {self.insert_mode(key_event)}
            Mode::Jump => {self.jump_mode(key_event)}
            Mode::Command => {self.command_mode(key_event)}
            Mode::Find => {self.find_mode(key_event)}
            Mode::Open => {self.open_mode(key_event)}
            Mode::Select => {self.select_mode(key_event)}
        }
    }

    fn normal_mode(&mut self, key_event: KeyEvent) -> Option<Action> {
        let code = key_event.code;
        let modifiers = key_event.modifiers;
        match modifiers {
            KeyModifiers::CONTROL => { self.input_buffer.push_str("Ctrl-") }
            KeyModifiers::ALT => { self.input_buffer.push_str("Alt-") }
            KeyModifiers::META => { self.input_buffer.push_str("Meta-") }
            _ => {}
        }
        match code {
            KeyCode::Backspace => { self.input_buffer.push_str("Backspace") }
            KeyCode::Enter => { self.input_buffer.push_str("Enter") }
            KeyCode::Left => { self.input_buffer.push_str("Left") }
            KeyCode::Right => { self.input_buffer.push_str("Right") }
            KeyCode::Up => { self.input_buffer.push_str("Up") }
            KeyCode::Down => { self.input_buffer.push_str("Down") }
            KeyCode::Home => { self.input_buffer.push_str("Home") }
            KeyCode::End => { self.input_buffer.push_str("End") }
            KeyCode::PageUp => { self.input_buffer.push_str("PageUp") }
            KeyCode::PageDown => { self.input_buffer.push_str("PageDown") }
            KeyCode::Tab => { self.input_buffer.push_str("Tab") }
            KeyCode::BackTab => { self.input_buffer.push_str("BackTab") }
            KeyCode::Delete => { self.input_buffer.push_str("Delete") }
            KeyCode::Insert => { self.input_buffer.push_str("Insert") }
            KeyCode::F(key) => { self.input_buffer.push_str(&format!("Fn{key}")) }
            KeyCode::Char(c) => {
                self.input_buffer.push(c)
            }
            KeyCode::Null => {}
            KeyCode::Esc => {
                // Special Handling since this key needs to be able to cancel any currently entered
                // input
                self.clear_input_buffer();
                return Some(Action{action_type:ActionType::EnterNormal, action_param:ActionParam::None});
            }
            KeyCode::CapsLock => { self.input_buffer.push_str("CapsLock") }
            KeyCode::ScrollLock => { self.input_buffer.push_str("ScrollLock") }
            KeyCode::NumLock => { self.input_buffer.push_str("NumLock") }
            KeyCode::PrintScreen => { self.input_buffer.push_str("PrintScreen") }
            KeyCode::Pause => { self.input_buffer.push_str("Pause") }
            KeyCode::Menu => { self.input_buffer.push_str("Menu") }
            KeyCode::KeypadBegin => { self.input_buffer.push_str("KeyboardBegin") }
            KeyCode::Media(_) => {}
            KeyCode::Modifier(modifier) => {
                match modifier {
                    ModifierKeyCode::LeftShift => { self.input_buffer.push_str("LeftShift") }
                    ModifierKeyCode::LeftControl => { self.input_buffer.push_str("LeftControl") }
                    ModifierKeyCode::LeftAlt => { self.input_buffer.push_str("LeftAlt") }
                    ModifierKeyCode::LeftSuper => { self.input_buffer.push_str("LeftSuper") }
                    ModifierKeyCode::LeftHyper => { self.input_buffer.push_str("LeftHyper") }
                    ModifierKeyCode::LeftMeta => { self.input_buffer.push_str("LeftMeta") }
                    ModifierKeyCode::RightShift => { self.input_buffer.push_str("RightShift") }
                    ModifierKeyCode::RightControl => { self.input_buffer.push_str("RightControl") }
                    ModifierKeyCode::RightAlt => { self.input_buffer.push_str("RightAlt") }
                    ModifierKeyCode::RightSuper => { self.input_buffer.push_str("RightSuper") }
                    ModifierKeyCode::RightHyper => { self.input_buffer.push_str("RightHyper") }
                    ModifierKeyCode::RightMeta => { self.input_buffer.push_str("RightMeta") }
                    _ => {}
                }
            }
        }

        let (num, command_str) = Self::strip_digits(&self.input_buffer);

        match self.key_map.normal.get(command_str) {
            None => None,
            Some(&action_type)=> Some(Action{action_type, action_param:ActionParam::Repeat(num)})
        }
    }

    fn insert_mode(&mut self, key_event: KeyEvent)-> Option<Action>{None}

    fn jump_mode(&mut self, key_event: KeyEvent)-> Option<Action>{None}

    fn command_mode(&mut self, key_event: KeyEvent)-> Option<Action>{None}

    fn find_mode(&mut self, key_event: KeyEvent)-> Option<Action>{None}

    fn open_mode(&mut self, key_event: KeyEvent)-> Option<Action>{None}

    fn select_mode(&mut self, key_event: KeyEvent)-> Option<Action>{None}

    fn clear_input_buffer(&mut self){
        self.input_buffer = "".to_string();
    }

    fn strip_digits(in_string: &str) -> (u16, &str) {
        let mut index = 0;
        for (i, c) in in_string.char_indices() {
            if !c.is_digit(10) {
                break;
            }
            index = i + c.len_utf8();
        }

        let mut digits = String::new();
        in_string[..index].clone_into(&mut digits);
        let parsed_digits: u16 = digits.parse().expect("Failed to parse into unsigned integer");


        return match in_string.strip_prefix(&digits) {
            None => { (0u16, in_string) }
            Some(stripped_str) => { (parsed_digits, stripped_str) }
        };
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_strip_digits() {
        let s = "123jlk";
        let (res_digit, res_str) = KeyReader::strip_digits(s);
        assert_eq!(res_digit, 123u16);
        assert_eq!(res_str, "jlk");
    }
}