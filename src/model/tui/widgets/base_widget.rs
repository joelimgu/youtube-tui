use crate::model::tui::widgets::screens::Widgets;
use async_trait::async_trait;
use crossterm::event::Event;

#[async_trait]
pub trait EventHandler {
    async fn handle_events<'a>(&mut self, event: Event) -> Option<Widgets<'a>>;
}
