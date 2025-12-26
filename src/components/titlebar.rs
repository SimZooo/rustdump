use gpui::{
    DefiniteLength, InteractiveElement, ParentElement, Render, Styled, WindowControlArea, div,
};
use gpui_component::{
    ActiveTheme, StyledExt, TitleBar,
    button::{Button, ButtonCustomVariant, ButtonVariants},
    white,
};

use crate::{RouteName, RustDump};

pub struct AppTitlebar {
    custom_button: ButtonCustomVariant,
}

impl AppTitlebar {
    pub fn new(custom_button: ButtonCustomVariant) -> Self {
        Self { custom_button }
    }

    pub fn render(
        &mut self,
        window: &mut gpui::Window,
        cx: &mut gpui::Context<RustDump>,
    ) -> impl gpui::IntoElement {
        //TitleBar::new()
        div()
            .border_b_1()
            .border_color(cx.theme().sidebar_border)
            .bg(cx.theme().background)
            .child(
                div()
                    .window_control_area(WindowControlArea::Drag)
                    .h_flex()
                    .size_full()
                    .font_family("Diodrum Cyrillic")
                    .child(
                        Button::new("info")
                            .child("Info")
                            .on_click(cx.listener(|app, _event, _window, cx| {
                                app.current_route = RouteName::Info;
                                cx.notify();
                            }))
                            .custom(self.custom_button)
                            .px_6(),
                    )
                    .child(
                        div()
                            .h(DefiniteLength::Fraction(0.6))
                            .w_1()
                            .border_l_1()
                            .border_color(cx.theme().sidebar_border),
                    )
                    .child(
                        Button::new("hexdump")
                            .child("Hexdump")
                            .on_click(cx.listener(|app, _event, _window, cx| {
                                app.current_route = RouteName::Hexdump;
                                cx.notify();
                            }))
                            .custom(self.custom_button)
                            .px_6(),
                    )
                    .child(
                        div()
                            .h(DefiniteLength::Fraction(0.6))
                            .w_1()
                            .border_l_1()
                            .border_color(cx.theme().sidebar_border),
                    )
                    .child(
                        Button::new("assembly")
                            .child("Assembly")
                            .on_click(cx.listener(|app, _event, _window, cx| {
                                app.current_route = RouteName::Assembly;
                                cx.notify();
                            }))
                            .custom(self.custom_button)
                            .px_6(),
                    ),
            )
    }
}
