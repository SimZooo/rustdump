use std::{
    borrow::Cow,
    collections::HashMap,
    path::{Path, PathBuf},
};

use crate::routes::{hexdump::hexdump::Hexdump, info::info::Info, starting::starting::Starting};
use gpui::{
    AnyElement, App, Application, AssetSource, Bounds, Context, DefiniteLength, FocusHandle,
    Focusable, KeyBinding, SharedString, Window, WindowBounds, WindowOptions, actions, div,
    prelude::*, px, size, transparent_black,
};
use gpui_component::{
    ActiveTheme, StyledExt, ThemeMode,
    button::{Button, ButtonCustomVariant, ButtonVariants},
};

mod routes;

actions!(rustdump, [OpenFile]);

#[derive(rust_embed::RustEmbed)]
#[folder = "assets"]
#[include = "icons/**/*.svg"]
#[include = "images/**/*.png"]
#[include = "fonts/**/*.ttf"]
pub struct Assets;

#[derive(PartialEq, Eq, Hash)]
pub enum RouteName {
    Starting,
    Info,
    Hexdump,
}

pub enum InfoDisplayPage {
    DOSHeaders,
    DOSStub,
    NTHeaders,
    SectionHeaders,
}

pub trait Route {
    fn render(&self, cx: &mut Context<RustDump>, app: &RustDump) -> AnyElement;
    fn load(&mut self, cx: &mut Context<RustDump>, window: &mut Window, path: &Path);
}

pub struct RustDump {
    pub current_route: RouteName,
    pub focus_handle: FocusHandle,
    pub routes: HashMap<RouteName, Box<dyn Route>>,
    pub curr_file: Option<PathBuf>,
    pub custom_button: ButtonCustomVariant,
    pub info_page: InfoDisplayPage,
}

impl RustDump {
    // Create a new instance with window parameter
    fn new(cx: &mut Context<Self>, window: &mut Window) -> Self {
        let mut routes: HashMap<RouteName, Box<dyn Route>> = HashMap::new();
        routes.insert(RouteName::Starting, Box::new(Starting::new()));
        routes.insert(RouteName::Info, Box::new(Info::new(window, cx)));
        routes.insert(RouteName::Hexdump, Box::new(Hexdump::new(window, cx)));

        // Set up key handler HERE, not in render
        let custom_button = ButtonCustomVariant::new(cx)
            .color(cx.theme().background)
            .foreground(cx.theme().foreground)
            .border(transparent_black())
            .hover(cx.theme().background)
            .active(cx.theme().accent);

        Self {
            info_page: InfoDisplayPage::DOSHeaders,
            routes,
            current_route: RouteName::Starting,
            focus_handle: cx.focus_handle(),
            curr_file: None,
            custom_button,
        }
    }

    fn open_file(&mut self, _: &OpenFile, window: &mut Window, cx: &mut Context<Self>) {
        let path = rfd::FileDialog::new().pick_file();
        if let Some(path) = path {
            // Load file into current route and change hashmap-value
            self.current_route = RouteName::Info;

            // Load all paths
            self.routes
                .values_mut()
                .for_each(|route| route.load(cx, window, &path));

            self.curr_file = Some(path);

            cx.notify();
        }
    }
}

impl Focusable for RustDump {
    fn focus_handle(&self, cx: &App) -> gpui::FocusHandle {
        cx.focus_handle()
    }
}

impl Render for RustDump {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .v_flex()
            .track_focus(&self.focus_handle)
            .key_context("rustdump")
            .on_action(cx.listener(Self::open_file))
            .size_full()
            .text_color(cx.theme().foreground)
            .child(
                div()
                    .h_flex()
                    .w_full()
                    .h_8()
                    .border_b_1()
                    .border_color(cx.theme().sidebar_border)
                    .bg(cx.theme().background)
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
                    ),
            )
            .child(
                div()
                    .v_flex()
                    .size_full()
                    .child(self.routes[&self.current_route].render(cx, self)),
            )
    }
}

impl AssetSource for Assets {
    fn load(&self, path: &str) -> anyhow::Result<Option<Cow<'static, [u8]>>> {
        if path.is_empty() {
            return Ok(None);
        }

        Self::get(path)
            .map(|f| Some(f.data))
            .ok_or_else(|| anyhow::anyhow!("could not find asset at path \"{path}\""))
    }

    fn list(&self, path: &str) -> anyhow::Result<Vec<SharedString>> {
        Ok(Self::iter()
            .filter_map(|p| p.starts_with(path).then(|| p.into()))
            .collect())
    }
}

fn main() {
    Application::new().with_assets(Assets).run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(1920.), px(1080.)), cx);
        cx.bind_keys(vec![KeyBinding::new("ctrl-o", OpenFile, None)]);
        Assets.load("icons/file-text.svg").unwrap().unwrap();
        Assets.load("icons/file-spreadsheet.svg").unwrap().unwrap();
        Assets.load("icons/list-tree.svg").unwrap().unwrap();

        let _ = cx.text_system().add_fonts(vec![
            Assets
                .load("fonts/DiodrumCyrillic-Regular.ttf")
                .unwrap()
                .unwrap(),
        ]);

        gpui_component::init(cx);
        gpui_component::Theme::change(ThemeMode::Dark, None, cx);
        let theme = gpui_component::Theme::global_mut(cx);
        theme.font_family = "Diodrum Cyrillic".into();

        let window = cx
            .open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    ..Default::default()
                },
                |window, cx| cx.new(|cx| RustDump::new(cx, window)),
            )
            .unwrap();

        window
            .update(cx, |rust_dump, window, cx| {
                window.focus(&rust_dump.focus_handle(cx));
                cx.activate(true);
            })
            .unwrap();
    });
}
