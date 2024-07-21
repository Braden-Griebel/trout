use std::collections::HashMap;
use crate::commands::actions::ActionType;

pub struct KeyMap {
    pub normal: HashMap<String, ActionType>,
    pub insert: HashMap<String, ActionType>,
    pub jump: HashMap<String, ActionType>,
    pub command: HashMap<String, ActionType>,
    pub find: HashMap<String, ActionType>,
    pub open: HashMap<String, ActionType>,
    pub select: HashMap<String, ActionType>,
}

impl KeyMap {
    fn default()-> KeyMap {
        let mut normal:HashMap<String, ActionType> = HashMap::new();
        let mut insert:HashMap<String, ActionType> = HashMap::new();
        let mut jump:HashMap<String, ActionType> = HashMap::new();
        let mut command :HashMap<String, ActionType> = HashMap::new();
        let mut find :HashMap<String, ActionType> = HashMap::new();
        let mut open:HashMap<String, ActionType> = HashMap::new();
        let mut select: HashMap<String, ActionType> = HashMap::new();
        // Normal Mode Keymaps
        normal.insert("w".to_string(), ActionType::MoveUp);
        normal.insert("a".to_string(), ActionType::MoveLeft);
        normal.insert("s".to_string(), ActionType::MoveDown);
        normal.insert("d".to_string(), ActionType::MoveRight);
        normal.insert("Space".to_string(), ActionType::EnterJump);
        // Insert Mode Keymaps
        for c in ' '..='~'{
            insert.insert(format!("{c}"), ActionType::InsertChar);
        }
        insert.insert("Escape".to_string(), ActionType::EnterNormal);
        KeyMap {
            normal, insert, jump, command, find, open, select
        }
    }
}