use crate::model;
use crate::model::tui::screen::Screen;
use crate::model::tui::widgets::base_widget::EventHandler;
use crate::model::tui::widgets::screens::Widgets;
use crate::model::tui::widgets::yb_search_results::YBSearchResults;
use async_trait::async_trait;
use crossterm::event::{Event, KeyCode};
use std::fs;
use tui::buffer::Buffer;
use tui::layout::{Alignment, Constraint, Layout, Rect};
use tui::style::Style;
use tui::widgets::{Block, Borders, Paragraph, Widget, Wrap};

#[derive(Clone)]
pub struct Input {
    text: String,
    message: Option<String>,
}

impl Input {
    pub fn new() -> Input {
        Input {
            text: String::from(""),
            message: None,
        }
    }

    pub fn get_text(&self) -> &str {
        self.text.as_str()
    }

    fn render_paragraph(&self, area: Rect, buf: &mut Buffer) {
        let chunks = Layout::default()
            .margin(2)
            .constraints(
                [
                    Constraint::Percentage(30),
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Min(2),
                ]
                .as_ref(),
            )
            .split(area);

        let paragraph = Paragraph::new(self.text.clone())
            .block(Block::default().title("Search: ").borders(Borders::ALL))
            .style(Style::default())
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: true });

        match &self.message {
            Some(msg) => {
                let message = Paragraph::new("Error in search: ".to_owned() + msg)
                    .block(Block::default())
                    .style(Style::default())
                    .alignment(Alignment::Left)
                    .wrap(Wrap { trim: true });
                Widget::render(message, chunks[2], buf);
            }
            None => {}
        };
        Widget::render(paragraph, chunks[1], buf);
    }

    fn edit_text(&mut self, key_code: KeyCode) {
        match key_code {
            KeyCode::Backspace => match self.text.pop() {
                _ => {}
            },
            KeyCode::Char('a') => self.text.push('a'),
            KeyCode::Char('b') => self.text.push('b'),
            KeyCode::Char('c') => self.text.push('c'),
            KeyCode::Char('d') => self.text.push('d'),
            KeyCode::Char('e') => self.text.push('e'),
            KeyCode::Char('f') => self.text.push('f'),
            KeyCode::Char('g') => self.text.push('g'),
            KeyCode::Char('h') => self.text.push('h'),
            KeyCode::Char('i') => self.text.push('i'),
            KeyCode::Char('j') => self.text.push('j'),
            KeyCode::Char('k') => self.text.push('k'),
            KeyCode::Char('l') => self.text.push('l'),
            KeyCode::Char('m') => self.text.push('m'),
            KeyCode::Char('n') => self.text.push('n'),
            KeyCode::Char('o') => self.text.push('o'),
            KeyCode::Char('p') => self.text.push('p'),
            KeyCode::Char('q') => self.text.push('q'),
            KeyCode::Char('r') => self.text.push('r'),
            KeyCode::Char('s') => self.text.push('s'),
            KeyCode::Char('t') => self.text.push('t'),
            KeyCode::Char('u') => self.text.push('u'),
            KeyCode::Char('v') => self.text.push('v'),
            KeyCode::Char('w') => self.text.push('w'),
            KeyCode::Char('x') => self.text.push('x'),
            KeyCode::Char('y') => self.text.push('y'),
            KeyCode::Char('z') => self.text.push('z'),
            KeyCode::Char(' ') => self.text.push(' '),
            KeyCode::Char(..) => self.text.push('\u{25AF}'),
            _ => {}
        }
        // todo use the | as a marker on text 😬
    }
}

#[async_trait]
impl EventHandler for Input {
    async fn handle_events<'a>(&mut self, event: Event) -> Option<Widgets<'a>> {
        match event {
            Event::Key(key) => match key.code {
                KeyCode::Esc => {
                    Screen::exit_app();
                    None
                }
                KeyCode::Enter => {
                    let contents =
                        fs::read_to_string("/home/joel/Documents/Code/youtube-tui/src/config.json")
                            .expect("Something went wrong reading the file");

                    let config: serde_json::Value =
                        serde_json::from_str(&contents).expect("JSON was not well-formatted");

                    let result = model::youtube::api::requests::search_videos::search(
                        self.text.as_str(),
                        &config["key"].to_string(),
                    )
                    .await;

                    let search_result = serde_json::from_str(&result.expect("http request failed"));
                    match search_result {
                        Ok(res) => Some(Widgets::YBSearchResults(
                            YBSearchResults::new_from_res(res).await,
                        )),
                        Err(err) => {
                            self.message = Some(err.to_string());
                            None
                        }
                    }
                }
                _ => {
                    self.edit_text(key.code);
                    None
                }
            },
            _ => None,
        }
    }
}

impl Widget for Input {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.render_paragraph(area, buf);
    }
}

impl Widget for &mut Input {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.render_paragraph(area, buf);
    }
}
