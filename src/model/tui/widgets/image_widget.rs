// todo use crossterm insted to be consistent
use crate::model::tui::widgets::base_widget::EventHandler;
use async_trait::async_trait;
use crossterm::event::Event;
use image::RgbImage;
use termion::{color, cursor};
use tui::widgets::Widget;
use tui::{buffer::Buffer, layout::Rect};

#[derive(Clone)]
pub struct Image {
    pub image: RgbImage,
}

impl Image {
    pub fn new(img: RgbImage) -> Image {
        Image { image: img }
    }
}

#[async_trait]
impl EventHandler for Image {
    async fn handle_events(&mut self, _event: Event) {}
}

impl Widget for Image {
    fn render(self, area: Rect, _buf: &mut Buffer) {
        print_img(self.image, area)
    }
}

fn print_img(img: RgbImage, area: Rect) {
    let size = (area.height - 1, area.width - 1);
    let initial_position = (area.x, area.y);
    let scale = get_scale(&img, size);

    for line in 0..size.1 {
        // thread::spawn(|| { /* code to execute in the thread */});
        for col in 0..(size.0) {
            //test if the coord is in the img adn dont' display anything if it gets out
            let is_in_img = {
                (col as u32 * scale.0 as u32) < (img.dimensions().0 as u32)
                    && (line as u32 * scale.1 as u32) < (img.dimensions().1 as u32)
            };

            //go to the top left, the x2 is bc the unicode char is 2 times as tall than wide
            //so we need the print 2 char to create a pixel
            print!(
                "{}{}",
                cursor::Hide,
                cursor::Goto(
                    2 * col + initial_position.0 + 1,
                    line + initial_position.1 + 1
                )
            );

            if is_in_img {
                let r = img
                    .get_pixel(col as u32 * scale.0 as u32, line as u32 * scale.1 as u32)
                    .0[0];
                let g = img
                    .get_pixel(col as u32 * scale.0 as u32, line as u32 * scale.1 as u32)
                    .0[1];
                let b = img
                    .get_pixel(col as u32 * scale.0 as u32, line as u32 * scale.1 as u32)
                    .0[2];

                print!("{}\u{2588}", color::Fg(color::Rgb(r, g, b)));
                print!("{}\u{2588}", color::Fg(color::Rgb(r, g, b)));
            } else {
                print!(" ");
            }
        }
    }
}

fn get_scale(img: &RgbImage, size: (u16, u16)) -> (i32, i32) {
    //let's find how much the img is scaled % to the terminal in each direction
    let scale = (
        (img.dimensions().0 / size.0 as u32) as i32,
        (img.dimensions().1 / size.1 as u32) as i32,
    );

    //now we take the smallest scale to avoid distorting the img
    (
        if 2 * scale.0 > scale.1 {
            scale.0
        } else {
            scale.1
        },
        if 2 * scale.0 > scale.1 {
            scale.0
        } else {
            scale.1
        },
    )
}
