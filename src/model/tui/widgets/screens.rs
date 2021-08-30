use crate::model::tui::widgets::input::Input;
use crate::model::tui::widgets::video_downloader::Download;
use crate::model::tui::widgets::video_player::VideoPlayer;
use crate::model::tui::widgets::yb_search_results::YBSearchResults;
use crate::model::tui::youtube_results_list::ResList;

#[allow(dead_code)]
pub enum Widgets<'a> {
    VideoSelector,
    ResList(ResList<'a>),
    YBSearchResults(YBSearchResults<'a>),
    Input(Input),
    DownloadScreen(Download),
    VideoPlayer(VideoPlayer),
}
