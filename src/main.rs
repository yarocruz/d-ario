#![warn(clippy::all, clippy::pedantic)]
mod document;
mod editor;
mod row;
mod terminal;
pub use document::Document;
pub use row::Row;
pub use terminal::Terminal;
pub use editor::Position;
use editor::Editor;

fn main() {
   Editor::default().run();
}
