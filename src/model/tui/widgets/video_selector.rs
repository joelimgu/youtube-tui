use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::widgets::Widget;

// todo implement this to search
struct VideoSelector {
    search_term: String,
}

impl VideoSelector {
    pub fn new(search_term: &str) -> VideoSelector {
        VideoSelector {
            search_term: search_term.to_string(),
        }
    }
}

impl Widget for VideoSelector {
    fn render(self, _area: Rect, _buf: &mut Buffer) {
        // print_img(self.image, area)
    }
}
