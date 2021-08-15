use crossterm::{terminal, QueueableCommand};
use std::io::stdout;
use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::widgets::Widget;

pub struct Clear;

impl Widget for Clear {
    fn render(self, _area: Rect, _buf: &mut Buffer) {
        clear();
    }
}
pub fn clear() {
    let mut stdout = stdout();
    stdout
        .queue(terminal::Clear(terminal::ClearType::All))
        .expect("can't clear the terminal");
}
