use crossterm::event::{Event, read};
use crate::terminal::controls::Terminal;
use crate::view::screen::{Screen, ScreenAction};

pub struct NormalViewer<'a> {
    quit_view: bool,
    screen: &'a Screen,
    screen_action: ScreenAction,
}

impl NormalViewer<'_> {
    pub fn run(&mut self, screen: &mut Screen) -> ScreenAction{
        // Set Cursor to blinking block
        Terminal::blinking_block_cursor().unwrap();
        loop {
            if self.quit_view{
                break;
            }
            let event = read().unwrap();
            self.evaluate_event(event);
            self.draw();
        }
        return self.screen_action.clone();
    }

    pub fn draw(&mut self) {

        Terminal::execute();
    }

    pub fn exit(&mut self){}

    fn evaluate_event(&mut self, event: Event){

    }
}