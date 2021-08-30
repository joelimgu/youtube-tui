use crate::model::tui::screen::{Screen, SearchResponse};
use crate::model::tui::widgets::clear_area;
use crate::model::tui::widgets::screens::Widgets;
use crate::model::tui::widgets::video_downloader;
use crate::model::tui::{
    widgets::{base_widget::EventHandler, image_widget::Image},
    youtube_results_list::ResList,
};
use async_trait::async_trait;
use crossterm::event::{Event, KeyCode};
use hyper::body::Bytes;
use image::io::Reader as ImageReader;
use image::RgbImage;
use std::io::Cursor;
use tui::buffer::Buffer;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::widgets::Widget;

#[derive(Clone)]
pub struct YBSearchResults<'a> {
    list: ResList<'a>,
    thumbnail: Image, // todo caching
    thumbnail_url: String,
    area: Option<(Rect, Buffer)>,
}

#[async_trait]
impl EventHandler for YBSearchResults<'_> {
    async fn handle_events<'a>(&mut self, event: Event) -> Option<Widgets<'a>> {
        self.list.handle_events(event).await;
        match event {
            Event::Key(key) => match key.code {
                KeyCode::Esc => {
                    Screen::exit_app();
                    None
                }
                KeyCode::Up => {
                    self.update_thumbnail().await;
                    None
                }
                KeyCode::Down => {
                    self.update_thumbnail().await;
                    None
                }
                KeyCode::Enter => {
                    clear_area::clear();
                    Some(Widgets::DownloadScreen(video_downloader::Download::new(
                        self.list.res.items[self.list.current as usize]
                            .id
                            .videoId
                            .clone(),
                    )))
                }
                _ => None,
            },
            _ => None,
        }
    }
}

impl YBSearchResults<'_> {
    pub(crate) async fn new_from_res<'a>(res: SearchResponse) -> YBSearchResults<'a> {
        let http_res = reqwest::get("https://i.ytimg.com/vi/dQw4w9WgXcQ/hqdefault.jpg")
            .await
            .unwrap();

        let bytes: Bytes = http_res.bytes().await.unwrap();
        let image: RgbImage = ImageReader::new(Cursor::new(&bytes))
            .with_guessed_format()
            .expect("can't guess the format")
            .decode()
            .expect("Could not decode the image")
            .to_rgb8();

        let list = ResList::new(res.clone());
        let thumbnail = Image::new(image);
        YBSearchResults {
            list,
            thumbnail,
            thumbnail_url: res.items[0].snippet.thumbnails.default.url.clone(),
            area: Option::None,
        }
    }

    async fn update_thumbnail(&mut self) {
        let http_res = reqwest::get(
            self.list.res.items[self.list.current as usize]
                .snippet
                .thumbnails
                .default
                .url
                .as_str(),
        )
        .await
        .expect("Error downloading the thumbnail for the video");

        let bytes: Bytes = http_res.bytes().await.unwrap();
        let image: RgbImage = ImageReader::new(Cursor::new(&bytes))
            .with_guessed_format()
            .expect("can't guess the format")
            .decode()
            .expect("Could not decode the image")
            .to_rgb8();
        let image = Image::new(image);
        self.thumbnail = image.clone();
        match &self.area {
            None => {}
            Some(area) => {
                self.thumbnail
                    .clone()
                    .render(area.0.clone(), &mut area.1.clone());
            }
        }
    }
}

impl Widget for YBSearchResults<'_> {
    fn render(mut self, area: Rect, buf: &mut Buffer) {
        self.area = Option::from((area, buf.clone()));
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(60), Constraint::Percentage(40)].as_ref())
            .split(area);
        self.thumbnail.render(chunks[1], buf);
        self.list.render(chunks[0], buf);
    }
}
