use std::{
    borrow::Cow,
    collections::HashMap,
    path::{Path, PathBuf},
};

use crate::{
    assets::{CombinedAssets, CustomAssets},
    components::titlebar::AppTitlebar,
    routes::{
        assembly::assembly::Assembly, hexdump::hexdump::Hexdump, info::info::Info,
        starting::starting::Starting,
    },
};
use gpui::{
    AnyElement, App, Application, AssetSource, Bounds, Context, DefiniteLength, Entity,
    FocusHandle, Focusable, KeyBinding, SharedString, TitlebarOptions, Window, WindowBounds,
    WindowOptions, actions, div, prelude::*, px, size, transparent_black,
};
use gpui_component::{
    ActiveTheme, Root, StyledExt, ThemeMode, TitleBar,
    button::{Button, ButtonCustomVariant, ButtonVariants},
    input::{Input, InputState},
};

mod assets;
mod components;
mod routes;

actions!(rustdump, [OpenFile]);

#[derive(PartialEq, Eq, Hash)]
pub enum RouteName {
    Starting,
    Info,
    Hexdump,
    Assembly,
}

pub enum InfoDisplayPage {
    DOSHeaders,
    DOSStub,
    FileHdr,
    OptHdr,
    Section(String),
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
    pub expand_nt: bool,
    pub expand_sct: bool,
    pub expand_section: bool,
    pub assembly_data: Vec<String>,
    pub titlebar: AppTitlebar,
}

impl RustDump {
    // Create a new instance with window parameter
    fn new(cx: &mut Context<Self>, window: &mut Window) -> Self {
        let mut routes: HashMap<RouteName, Box<dyn Route>> = HashMap::new();
        routes.insert(RouteName::Starting, Box::new(Starting::new()));
        routes.insert(RouteName::Info, Box::new(Info::new(window, cx)));
        routes.insert(RouteName::Hexdump, Box::new(Hexdump::new(window, cx)));
        routes.insert(RouteName::Assembly, Box::new(Assembly::new(window, cx)));

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
            titlebar: AppTitlebar::new(custom_button.clone()),
            custom_button,
            expand_section: false,
            expand_nt: false,
            expand_sct: false,
            assembly_data: vec![],
        }
    }

    fn open_file(&mut self, _: &OpenFile, window: &mut Window, cx: &mut Context<Self>) {
        let path = rfd::FileDialog::new().pick_file();
        if let Some(path) = path {
            self.current_route = RouteName::Info;

            for (_, route) in &mut self.routes {
                route.load(cx, window, &path)
            }

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
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .v_flex()
            .track_focus(&self.focus_handle)
            .key_context("rustdump")
            .on_action(cx.listener(Self::open_file))
            .size_full()
            .text_color(cx.theme().foreground)
            .child(self.titlebar.render(window, cx))
            .child(
                div()
                    .v_flex()
                    .size_full()
                    .child(self.routes[&self.current_route].render(cx, self)),
            )
    }
}

fn main() {
    let assets = CombinedAssets::new();
    assets.load("icons/file-text.svg").unwrap().unwrap();
    assets.load("icons/file-spreadsheet.svg").unwrap().unwrap();
    assets.load("icons/list-tree.svg").unwrap().unwrap();
    let font = assets
        .load("fonts/DiodrumCyrillic-Regular.ttf")
        .unwrap()
        .unwrap();
    Application::new().with_assets(assets).run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(1920.), px(1080.)), cx);
        cx.bind_keys(vec![KeyBinding::new("ctrl-o", OpenFile, None)]);

        let _ = cx.text_system().add_fonts(vec![font]);

        gpui_component::init(cx);
        gpui_component::Theme::change(ThemeMode::Dark, None, cx);
        let theme = gpui_component::Theme::global_mut(cx);
        theme.font_family = "Diodrum Cyrillic".into();

        let window = cx
            .open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    //titlebar: Some(TitleBar::title_bar_options()),
                    ..Default::default()
                },
                |window, cx| {
                    let view = cx.new(|cx| RustDump::new(cx, window));
                    cx.new(|cx| Root::new(view, window, cx))
                },
            )
            .unwrap();

        window
            .update(cx, |root, window, cx| {
                cx.activate(true);
            })
            .unwrap();
    });
}
