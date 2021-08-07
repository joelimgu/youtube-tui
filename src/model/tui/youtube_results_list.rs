use crate::model::youtube::api::search_response::SearchResponse;
use tui::widgets::{Block, Borders};
use tui::{
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{List, ListItem, ListState},
};

pub struct ResList<'a> {
    pub(crate) list: List<'a>,
    pub state: ListState,
    pub length: u32,
    current: u32,
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
        }
    }

    pub fn select_next(&mut self) {
        if self.current < self.length {
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
