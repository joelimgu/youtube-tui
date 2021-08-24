use crate::model::tui::widgets::base_widget::EventHandler;
use crate::model::tui::widgets::screens::Widgets;
use crate::model::youtube::api::search_response::SearchResponse;
use async_trait::async_trait;
use crossterm::event::{Event, KeyCode};
use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::widgets::{Block, Borders, StatefulWidget, Widget};
use tui::{
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{List, ListItem, ListState},
};

#[derive(Clone, Debug)]
pub struct ResList<'a> {
    pub(crate) list: List<'a>,
    pub state: ListState,
    pub length: u32,
    pub current: u32,
    pub(crate) res: SearchResponse,
}

impl ResList<'_> {
    pub fn new<'a>(search_result: SearchResponse) -> ResList<'a> {
        let length = search_result.items.len() as u32;
        let state = ListState::default();
        let list: Vec<_> = search_result
            .items
            .iter()
            .map(|item| {
                ListItem::new(Spans::from(vec![Span::styled(
                    item.snippet.title.clone(),
                    Style::default(),
                )]))
            })
            .collect();
        let list = List::new(list)
            .block(Block::default().title("List").borders(Borders::ALL))
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
            .highlight_symbol(">>");

        ResList {
            list,
            state,
            length,
            current: 0,
            res: search_result,
        }
    }

    pub fn select_next(&mut self) {
        if self.current < self.length - 1 {
            self.current += 1;
            self.state.select(Some(self.current as usize))
        }
    }

    pub fn select_prev(&mut self) {
        if self.current > 0 {
            self.current -= 1;
            self.state.select(Some(self.current as usize))
        }
    }
}

#[async_trait]
impl EventHandler for ResList<'_> {
    async fn handle_events<'a>(&mut self, event: Event) -> Option<Widgets<'a>> {
        match event {
            Event::Key(key) => match key.code {
                KeyCode::Up => self.select_prev(),
                KeyCode::Down => self.select_next(),
                _ => {}
            },
            _ => {}
        }
        None
    }
}

impl Widget for ResList<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        StatefulWidget::render(self.list, area, buf, &mut self.state.clone());
    }
}
