use crate::textbuffer::buffer::Buffer;
use crate::view::modes::command::CommandViewer;
use crate::view::modes::find::FindViewer;
use crate::view::modes::insert::InsertViewer;
use crate::view::modes::jump::JumpViewer;
use crate::view::modes::normal::NormalViewer;
use crate::view::modes::open::OpenViewer;
use crate::view::modes::search::SearchViewer;
use crate::view::modes::select::SelectViewer;
use crate::view::modes::welcome_screen::WelcomeViewer;
use crate::view::screen::Boundary;
pub mod normal;
mod jump;
mod search;
mod insert;
mod command;
mod find;
mod open;
mod select;
mod welcome_screen;