use crossterm::{
    cursor,
    event::KeyCode,
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use tui::{
    backend::{
        CrosstermBackend,
        Backend
    },
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Widget},
    Terminal,
};

use std::{
    io::{stdout, Error},
    result,
};

fn enter_terminal() {
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen).unwrap();
    execute!(stdout, cursor::Hide).unwrap();
}

fn close_terminal() {
    let mut stdout = stdout();
    execute!(stdout, LeaveAlternateScreen).unwrap();
    execute!(stdout, cursor::Show).unwrap();
}

use std::time::Duration;

use crossterm::event::{poll, read, Event};

fn start_application() -> crossterm::Result<()> {
    let stdout = stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    draw_layouts(&mut terminal);
    loop {
        // `poll()` waits for an `Event` for a given time period
        if poll(Duration::from_millis(5000))? {
            // It's guaranteed that the `read()` won't block when the `poll()`
            // function returns `true`
            match read()? {
                Event::Key(event) => {
                    println!("Key: {:?}", event);
                    if event.code == crossterm::event::KeyCode::Esc {
                        close_terminal();
                        disable_raw_mode().unwrap();
                        std::process::exit(0);
                    }
                }
                Event::Mouse(event) => println!("{:?}", event),
                Event::Resize(width, height) => {
                    draw_layouts(&mut terminal);
                }
            }
        }
    }
}
fn draw_layouts<B: Backend> (terminal :&mut Terminal<B>){
    terminal.draw(|f| {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(
                [
                    Constraint::Percentage(10),
                    Constraint::Percentage(80),
                    Constraint::Percentage(10)
                ].as_ref()
            )
            .split(f.size());
        let block = Block::default()
             .title("Block")
             .borders(Borders::ALL);
        f.render_widget(block, chunks[0]);
        let block = Block::default()
             .title("Block 2")
             .borders(Borders::ALL);
        f.render_widget(block, chunks[1]);
    });
}
pub fn render() -> Result<(), Error> {
    enable_raw_mode().unwrap();
    enter_terminal();
    start_application().unwrap();
    //never executed
    close_terminal();
    Ok(())
}
