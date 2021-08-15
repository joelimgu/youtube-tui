use crate::model::tui::widgets::yb_search_results::YBSearchResults;
use crate::model::tui::youtube_results_list::ResList;

#[derive(Clone)]
pub enum Widgets<'a> {
    VideoSelector,
    ResList(ResList<'a>),
    YBSearchResults(YBSearchResults<'a>),
}
