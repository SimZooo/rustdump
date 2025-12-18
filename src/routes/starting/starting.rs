use std::any::TypeId;

use gpui::{
    actions, div, prelude::*, App, Context, Div, KeyBinding, KeyDownEvent, KeyEvent, KeyUpEvent,
    Keystroke, Modifiers, SharedString, Window,
};
use gpui_component::ActiveTheme;

use crate::RustDump;

pub struct Starting {
    // Fields and methods go here
}

impl Starting {
    pub fn new() -> Self {
        Self {}
    }

    pub fn render(&mut self, cx: &mut Context<RustDump>) -> Div {
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
    }
}
