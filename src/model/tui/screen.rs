use std::io::{stdout, Error};
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::time::Duration;

use crossterm::event::{poll, read, Event};
use crossterm::{
    cursor,
    event::KeyCode,
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::layout::Alignment;
use tui::style::{Color, Style};
use tui::widgets::{Paragraph, Widget};
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    Terminal,
};

use crate::model::tui::widgets::base_widget::EventHandler;
pub use crate::model::{
    tui::{
        widgets::{
            clear_area, image_widget::Image, screens::Widgets, yb_search_results::YBSearchResults,
        },
        youtube_results_list::ResList,
    },
    youtube::api::search_response::SearchResponse,
};

pub struct Screen<'a> {
    renders_done: u32,
    tx: Sender<Result<Event, crossterm::ErrorKind>>,
    rx: Receiver<Result<Event, crossterm::ErrorKind>>,
    main_screen: Widgets<'a>, //Screens
}

impl Screen<'_> {
    pub async fn new<'a>(res: SearchResponse) -> Screen<'a> {
        let (tx, rx) = mpsc::channel();
        Screen {
            renders_done: 0,
            tx,
            rx,
            main_screen: Widgets::YBSearchResults(YBSearchResults::new_from_res(res).await),
        }
    }

    pub fn enter_terminal() {
        let mut stdout = stdout();
        execute!(stdout, EnterAlternateScreen).unwrap();
        execute!(stdout, cursor::Hide).unwrap();
    }

    fn close_terminal() {
        let mut stdout = stdout();
        execute!(stdout, LeaveAlternateScreen).unwrap();
        execute!(stdout, cursor::Show).unwrap();
    }

    pub fn exit_app() {
        Screen::close_terminal();
        disable_raw_mode().unwrap();
        std::process::exit(0);
    }

    pub async fn handle_events(&mut self) {
        let event = self
            .rx
            .recv()
            .expect("Err while recievent the events in the reciever")
            .unwrap();

        match &mut self.main_screen {
            Widgets::ResList(res_list) => {
                match self
                    .rx
                    .recv()
                    .expect("Err while recievent the events in the reciever")
                    .unwrap()
                {
                    Event::Key(event) => match event.code {
                        KeyCode::Esc => Screen::exit_app(),
                        KeyCode::Up => {
                            res_list.select_prev();
                        }
                        KeyCode::Down => {
                            res_list.select_next();
                        }
                        _ => {}
                    },
                    Event::Resize(_, _) => clear_area::clear(),
                    _ => {}
                }
            }
            Widgets::YBSearchResults(search_res) => search_res.handle_events(event).await,
            _ => {}
        }
    }

    pub fn render_results<T: Widget>(widget: T) {
        let print = "------";

        let stdout = stdout();
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend).expect("can't access the terminal");
        // let res_list: &mut ResList = &mut self.main_screen;
        terminal
            .draw(|frame| {
                let size = frame.size();
                // let thumbnail = Image::new(img.clone());

                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(1)
                    .constraints(
                        [
                            Constraint::Length(3),
                            Constraint::Min(2),
                            Constraint::Length(3),
                        ]
                        .as_ref(),
                    )
                    .split(size);

                let text = Paragraph::new(print).style(Style::default());

                let page_bottom = Paragraph::new("youtube-tui".to_owned())
                    .style(Style::default().fg(Color::Blue))
                    .alignment(Alignment::Center);

                frame.render_widget(text, chunks[0]);
                frame.render_widget(widget, chunks[1]);

                frame.render_widget(page_bottom, chunks[2]);
            })
            .expect("can't render the image");
    }
    pub async fn render(&mut self) -> Result<(), Error> {
        enable_raw_mode().unwrap();

        Screen::enter_terminal();
        let tx = self.tx.clone();
        thread::spawn(move || loop {
            if poll(Duration::from_millis(100)).unwrap() {
                tx.send(read()).expect("Can't send the events from tx");
            }
        });

        loop {
            match &mut self.main_screen {
                Widgets::ResList(_res_list) => {
                    // Screen::render_results(&mut res_list.clone(), img.clone());
                }
                Widgets::YBSearchResults(yb_search_results) => {
                    Screen::render_results(yb_search_results.to_owned());
                }
                _ => {}
            }
            self.handle_events().await;
            self.renders_done += 1;
        }
    }
}
