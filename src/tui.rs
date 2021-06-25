use crossterm::{cursor, event::KeyCode, execute, terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode}};

use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Widget},
    Terminal,
};

use std::io::{stdout, Error};


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

fn print_events() -> crossterm::Result<()> {
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
                },
                Event::Mouse(event) => println!("{:?}", event),
                Event::Resize(width, height) => println!("New size {}x{}", width, height),
            }
        } else {
            // Timeout expired and no `Event` is available
        }
    }
    Ok(())
}

pub fn render() -> Result<(), Error>{
    enable_raw_mode().unwrap();

    let stdout = stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    enter_terminal();
    terminal.draw(|f| {
        let size = f.size();
        let block = Block::default()
            .title("Block")
            .borders(Borders::ALL);
        f.render_widget(block, size);
    })?;
    let _x = print_events();

    //never executed
    close_terminal();
    Ok(())
}
