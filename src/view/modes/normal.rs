use std::cmp::min;
use std::iter::Enumerate;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers, read};
use crate::terminal::controls::Terminal;
use crate::terminal::screen_location::ScreenLocation;
use crate::view::screen::{Mode, Screen, ScreenAction};

pub struct NormalViewer<'a> {
    quit_view: bool,
    screen: &'a mut Screen,
    screen_action: ScreenAction,
    needs_redraw: bool,
}

impl<'a> NormalViewer<'a> {
    pub fn enter(screen: &'a mut Screen) ->ScreenAction{
        let mut s = Self{
            quit_view:false,
            screen,
            screen_action: ScreenAction::QuitScreen,
            needs_redraw:false
        };
        s.run()
    }
    pub fn run(&mut self) -> ScreenAction{
        // Set Cursor to blinking block
        Terminal::blinking_block_cursor().unwrap();
        loop {
            if self.quit_view{
                break;
            }
            match read().unwrap() {
                Event::FocusGained => {} // Nothing for now
                Event::FocusLost => {} // Nothing for now
                Event::Key(KeyEvent{code, modifiers, kind,.. }) => {
                    if kind == KeyEventKind::Press{
                        match modifiers{
                            KeyModifiers::CONTROL => {
                                match code {
                                    KeyCode::Char(c)=>{
                                        match c {
                                            'w'=>{}
                                            'a'=>{}
                                            's'=>{}
                                            'd'=>{}
                                            _=>{}
                                        }
                                    }
                                    _=>{}
                                }
                            } // Nothing yet
                            KeyModifiers::SHIFT => {
                                match code {
                                    KeyCode::Left => {self.screen.move_prev_word().unwrap()}
                                    KeyCode::Right => {self.screen.move_next_word().unwrap()}
                                    KeyCode::Up => {self.screen.move_first_line().unwrap()}
                                    KeyCode::Down => {self.screen.move_last_line().unwrap()}
                                    KeyCode::Home => {self.screen.move_start_line().unwrap()}
                                    KeyCode::End => {self.screen.move_end_line().unwrap()}
                                    KeyCode::Char(c) => {
                                        match c{
                                            'w'=>{self.screen.move_first_line().unwrap()}
                                            'a'=>{self.screen.move_prev_word().unwrap()}
                                            's' => {self.screen.move_last_line().unwrap()}
                                            'd'=>{self.screen.move_next_word().unwrap()}
                                            _ => {}
                                        }
                                    }
                                    _=>{}
                                }
                            }
                            KeyModifiers::ALT => {}
                            KeyModifiers::META => {}
                            KeyModifiers::NONE => {
                                match code {
                                    KeyCode::Delete =>{self.screen.delete_grapheme(
                                        self.screen.text_position.clone()
                                    )}
                                    KeyCode::Left => {self.screen.move_left().unwrap()}
                                    KeyCode::Right => {self.screen.move_right().unwrap()}
                                    KeyCode::Up => {self.screen.move_up().unwrap()}
                                    KeyCode::Down => {self.screen.move_down().unwrap()}
                                    KeyCode::Home => {self.screen.move_start_line().unwrap()}
                                    KeyCode::End => {self.screen.move_end_line().unwrap()}
                                    KeyCode::Char(c) => {
                                        match c{
                                            'q'=>{return ScreenAction::QuitScreen}
                                            'w'=>{self.screen.move_up().unwrap()}
                                            'a'=>{self.screen.move_left().unwrap()}
                                            's'=>{self.screen.move_down().unwrap()}
                                            'd'=>{self.screen.move_right().unwrap()}
                                            'i'=>{return ScreenAction::EnterMode(Mode::Insert)}
                                            ' '=>{return ScreenAction::EnterMode(Mode::Jump)}
                                            'e'=>{return ScreenAction::EnterMode(Mode::Open)}
                                            'f'=>{return ScreenAction::EnterMode(Mode::Find)}
                                            'c'=>{return ScreenAction::EnterMode(Mode::Command)}
                                            'h'=>{return ScreenAction::EnterMode(Mode::Select)}
                                            'x'=>{self.screen.delete_grapheme(self.screen.text_position.clone())}
                                            _=>{}
                                        }
                                    }
                                    _=>{}
                                }

                            }
                            _=>{}
                        }
                    }
                }
                Event::Mouse(_) => {}
                Event::Paste(_) => {}
                Event::Resize(row, col) => {} // Nothing yet, but should resize the screen bounds
            }
            self.draw();
        }
        return self.screen_action.clone();
    }

    pub fn draw(&mut self) {
        let _ = Terminal::hide_caret(); // Hide the caret so it doesn't flicker across the screen
        self.draw_text(); // Draw the text to the screen
        let _ = Terminal::execute(); // Execute the queued commands, drawing the current view
    }

    /// Draw the text portion of the screen
    fn draw_text(&mut self){
        for (idx,line) in (self.screen.scroll_offset.row..(
            self.screen.view_height()+self.screen.scroll_offset.row)).enumerate(){
            if line < self.screen.buffer.num_lines {
                self.draw_line(idx, line);
            } else {
                self.draw_empty_line(idx);
            }
        }
    }

    /// draw a line of text to the screen
    fn draw_line(&mut self, screen_row: usize, text_line: usize){
        // Move caret to start of view on current line
        let _=Terminal::move_caret_to(ScreenLocation{
            row:screen_row, col: self.screen.inner_boundary.left});
        // Clear to the end of the line
        let _ = Terminal::clear_to_line_end();
        // Print the row of text
        let _ = Terminal::print(&self.screen.buffer.print_line(
            text_line,
            self.screen.scroll_offset.col,
            self.screen.scroll_offset.col+self.screen.view_width(),
            false
        ));
    }

    /// draw an empty line to the screen
    fn draw_empty_line(&self, screen_row: usize){
        let _ = Terminal::move_caret_to(ScreenLocation{
            row: screen_row, col: self.screen.inner_boundary.left
        });
        let _ = Terminal::clear_to_line_end();
        let _ = Terminal::print("~");
    }
}