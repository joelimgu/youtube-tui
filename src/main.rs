mod model;
extern crate jpeg_decoder as jpeg;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let screen = model::tui::screen::Screen::new_input();
    let _ = screen.render().await;
}
