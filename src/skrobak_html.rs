use scraper::{node::Element, Html, Selector};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SkrobakHtml {
    html: Html,
}

impl SkrobakHtml {
    fn new() -> Self {
        Self {
            html: Html::new_document(),
        }
    }
}
