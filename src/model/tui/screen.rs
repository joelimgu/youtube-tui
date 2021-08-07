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
use tui::widgets::Paragraph;
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    Terminal,
};

use crate::model::{
    tui::youtube_results_list::ResList, youtube::api::search_response::SearchResponse,
};

pub struct Screen<'a> {
    renders_done: u32,
    tx: Sender<Result<Event, crossterm::ErrorKind>>,
    rx: Receiver<Result<Event, crossterm::ErrorKind>>,
    main_screen: ResList<'a>,
}

impl Screen<'_> {
    pub fn new<'a>(res: SearchResponse) -> Screen<'a> {
        let (tx, rx) = mpsc::channel();
        Screen {
            renders_done: 0,
            tx,
            rx,
            main_screen: ResList::new(res),
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

    pub fn handle_events(&mut self) {
        match self
            .rx
            .recv()
            .expect("Err while recievent the events in the reciever")
            .unwrap()
        {
            Event::Key(event) => match event.code {
                KeyCode::Esc => Screen::exit_app(),
                KeyCode::Up => {
                    self.main_screen.select_prev();
                }
                KeyCode::Down => {
                    self.main_screen.select_next();
                }
                KeyCode::Char('a') => self.main_screen.state.select(Some(2)),
                _ => {}
            },
            _ => {}
        }
    }

    pub fn render(&mut self) -> Result<(), Error> {
        enable_raw_mode().unwrap();

        let print = "------";
        let stdout = stdout();
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;
        Screen::enter_terminal();
        let tx = self.tx.clone();
        thread::spawn(move || loop {
            if poll(Duration::from_millis(100)).unwrap() {
                tx.send(read()).expect("Can't send the events from tx");
            }
        });

        self.main_screen.state.select(Some(0));

        loop {
            terminal
                .draw(|frame| {
                    let size = frame.size();
                    let chunks = Layout::default()
                        .direction(Direction::Vertical)
                        .margin(2)
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

                    let page_bottom =
                        Paragraph::new("youtube-tui".to_owned() + &self.renders_done.to_string())
                            .style(Style::default().fg(Color::Blue))
                            .alignment(Alignment::Center);

                    frame.render_widget(text, chunks[0]);
                    frame.render_stateful_widget(
                        self.main_screen.list.clone(),
                        chunks[1],
                        &mut self.main_screen.state,
                    );

                    frame.render_widget(page_bottom, chunks[2]);
                })
                .expect("can't render the image");
            self.handle_events();
            self.renders_done += 1;
        }
    }
}
