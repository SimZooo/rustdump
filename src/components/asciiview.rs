use gpui::{AnyElement, Context, Div, IntoElement, ParentElement, SharedString, Styled, div};
use gpui_component::ActiveTheme;

use crate::RustDump;

pub struct AsciiView {
    pub rows: Vec<SharedString>,
}

impl AsciiView {
    pub fn new(data: Vec<u8>, cx: &Context<RustDump>) -> Self {
        let chunks = data
            .iter()
            .map(|b| {
                if b.is_ascii_graphic() {
                    SharedString::from((*b as char).to_string())
                } else {
                    SharedString::from('.'.to_string())
                }
            })
            .collect::<Vec<SharedString>>();
        Self { rows: chunks }
    }

    pub fn render(&self, cx: &Context<RustDump>) -> impl IntoElement {
        div().grid().grid_cols(16).children(self.rows.clone())
    }
}
