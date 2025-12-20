use gpui::{AnyElement, Context, SharedString, Window, div, prelude::*};
use gpui_component::ActiveTheme;

use crate::{Route, RustDump};

pub struct Starting {}

impl Route for Starting {
    fn render(&self, cx: &mut Context<RustDump>, _app: &RustDump) -> AnyElement {
        self.render_route(cx)
    }

    fn load(&mut self, _cx: &mut Context<RustDump>, _window: &mut Window, _path: &std::path::Path) {
    }
}

impl Starting {
    pub fn new() -> Self {
        Self {}
    }

    pub fn render_route(&self, cx: &mut Context<RustDump>) -> AnyElement {
        div()
            .size_full()
            .flex()
            .flex_col()
            .justify_center()
            .items_center()
            .border_0()
            .bg(cx.theme().background)
            .text_color(cx.theme().foreground)
            .font_family(SharedString::from("Diodrum Cyrillic"))
            .child("Open file with Ctrl + O")
            .into_any_element()
    }
}
