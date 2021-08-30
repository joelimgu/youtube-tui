use crate::model::tui::screen::Widgets;
use crate::model::tui::widgets::base_widget::EventHandler;
use crate::model::tui::widgets::video_player::VideoPlayer;
use async_trait::async_trait;
use crossterm::event::Event;
use tui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Rect},
    style::Style,
    widgets::{Block, Borders, Paragraph, Widget, Wrap},
};

#[derive(Clone)]
pub struct Download {
    video_id: String,
}

impl Download {
    pub fn new(id: String) -> Download {
        Download { video_id: id }
    }
}

impl Widget for Download {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let chunks = Layout::default()
            .margin(2)
            .constraints(
                [
                    Constraint::Percentage(30),
                    Constraint::Length(3),
                    Constraint::Min(2),
                ]
                .as_ref(),
            )
            .split(area);

        let paragraph = Paragraph::new(self.video_id)
            .block(
                Block::default()
                    .title("Downloading: ")
                    .borders(Borders::ALL),
            )
            .style(Style::default())
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: true });

        Widget::render(paragraph, chunks[1], buf);
    }
}

#[async_trait]
impl EventHandler for Download {
    async fn handle_events<'a>(&mut self, _event: Event) -> Option<Widgets<'a>> {
        let id = "https://www.youtube.com/watch?v=".to_owned() + &self.video_id;
        let path_to_video = rustube::download_worst_quality(&id)
            .await
            .expect("error while saving downloading the video");
        Some(Widgets::VideoPlayer(VideoPlayer::new(path_to_video)))
    }
}
