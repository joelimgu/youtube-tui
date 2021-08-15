use async_trait::async_trait;
use crossterm::event::Event;

#[async_trait]
pub trait EventHandler {
    async fn handle_events(&mut self, event: Event);
}
