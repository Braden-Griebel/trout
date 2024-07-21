

/// An enum representing possible actions
///
/// This includes basic movement, opening a new file, entering different modes, etc.
#[derive(Copy, Clone)]
pub enum ActionType {
    // Basic Movement Controls
    MoveRight,
    MoveLeft,
    MoveUp,
    MoveDown,
    // Change Mode Controls
    EnterNormal,
    EnterInsert,
    EnterJump,
    EnterCommand,
    EnterFind,
    EnterOpen,
    // Insert Character
    InsertChar,
    // Cancel current action
    Cancel,
}

pub struct Action {
    pub action_type: ActionType,
    pub action_param: ActionParam,
}

pub enum ActionParam {
    Repeat(u16),
    Character(char),
    JumpSequence(String),
    None
}