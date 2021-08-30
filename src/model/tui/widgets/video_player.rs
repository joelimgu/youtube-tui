use crate::model::tui::screen::{Screen, Widgets};
use crate::model::tui::widgets::base_widget::EventHandler;
use crate::model::tui::widgets::image_widget::Image;
use async_trait::async_trait;
use crossterm::event::{Event, KeyCode};
use ffmpeg_frame_grabber::{FFMpegVideo, FFMpegVideoOptions};
use std::path::PathBuf;
use tui::{buffer::Buffer, layout::Rect, widgets::Widget};

pub struct VideoPlayer {
    video_path: PathBuf,
    video: Option<FFMpegVideo>,
}

impl VideoPlayer {
    pub fn new(path: PathBuf) -> VideoPlayer {
        VideoPlayer {
            video_path: path,
            video: None,
        }
    }
}

impl Widget for &mut VideoPlayer {
    fn render(mut self, area: Rect, _buf: &mut Buffer) {
        match &mut self.video {
            None => {
                // we read the video
                // vid already implements iterator
                let vid: FFMpegVideo =
                    FFMpegVideo::open(&self.video_path, FFMpegVideoOptions::default()).unwrap();
                self.video = Some(vid);
            }
            Some(vid) => {
                let frame = vid.next().unwrap().unwrap().image;
                Image::print_img(frame, area);
            }
        }
    }
}

#[async_trait]
impl EventHandler for VideoPlayer {
    async fn handle_events<'a>(&mut self, event: Event) -> Option<Widgets<'a>> {
        match event {
            Event::Key(key) => match key.code {
                KeyCode::Esc => {
                    Screen::exit_app();
                    None
                }
                _ => None,
            },
            _ => None,
        }
    }
}
