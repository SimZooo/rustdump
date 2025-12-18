use std::{borrow::Cow, path::PathBuf};

use crate::routes::hexdump::hexdump::Hexdump;
use crate::routes::starting::starting::Starting;
use gpui::{
    actions, div, prelude::*, px, size, App, Application, AssetSource, Bounds, Context,
    DefiniteLength, Entity, FocusHandle, Focusable, KeyBinding, KeyDownEvent, Keystroke,
    SharedString, Window, WindowBounds, WindowOptions,
};
use gpui_component::{
    black,
    resizable::ResizableState,
    sidebar::{
        Sidebar, SidebarGroup, SidebarHeader, SidebarMenu, SidebarMenuItem, SidebarToggleButton,
    },
    white, ActiveTheme, Icon, IconName, Side, StyledExt, ThemeMode,
};

mod routes;

actions!(rustdump, [OpenFile]);

#[derive(rust_embed::RustEmbed)]
#[folder = "assets"]
#[include = "icons/**/*.svg"]
#[include = "images/**/*.png"]
#[include = "fonts/**/*.ttf"]
pub struct Assets;

#[derive(PartialEq)]
enum Route {
    Starting,
    Hexdump,
}

struct RustDump {
    current_route: Route,
    focus_handle: FocusHandle,
    curr_file: Option<PathBuf>,
    hexdump: Option<Hexdump>,
    starting: Starting,
    sidebar_state: Entity<ResizableState>,
}

impl RustDump {
    // Create a new instance with window parameter
    fn new(cx: &mut Context<Self>, window: &mut Window) -> Self {
        // Set up key handler HERE, not in render
        let resize_state = cx.new(|_| ResizableState::default());

        Self {
            sidebar_state: resize_state,
            current_route: Route::Starting,
            focus_handle: cx.focus_handle(),
            curr_file: None,
            starting: Starting::new(),
            hexdump: None,
        }
    }

    fn open_file(&mut self, _: &OpenFile, window: &mut Window, cx: &mut Context<Self>) {
        let path = rfd::FileDialog::new().pick_file();
        if let Some(path) = path {
            println!("{:?}", path);
            self.curr_file = Some(path);
            self.current_route = Route::Hexdump;
            self.hexdump = Some(Hexdump::new(self, cx, window));

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
            .h_flex()
            .track_focus(&self.focus_handle)
            .key_context("rustdump")
            .on_action(cx.listener(Self::open_file))
            .size_full()
            .text_color(cx.theme().foreground)
            .child(
                Sidebar::new(Side::Left)
                    .w_40()
                    .font_family(SharedString::from("Diodrum Cyrillic"))
                    .collapsible(true)
                    .header(SidebarHeader::new().child(div().child("Rustdump")))
                    .child(
                        SidebarGroup::new("Menu").child(
                            SidebarMenu::new().child(
                                SidebarMenuItem::new("Hexdump")
                                    .icon(IconName::Folder)
                                    .on_click(cx.listener(|app, _, _, _| {
                                        if app.curr_file.is_some() {
                                            app.current_route = Route::Hexdump;
                                        }
                                    })),
                            ),
                        ),
                    ),
            )
            .child(
                div()
                    .v_flex()
                    .size_full()
                    .child(
                        div()
                            .h_10()
                            .w_full()
                            .border_b_1()
                            .bg(cx.theme().background)
                            .border_color(cx.theme().border),
                    )
                    .child(match self.current_route {
                        Route::Starting => self.starting.render(cx),
                        Route::Hexdump => {
                            if let Some(hexdump) = &mut self.hexdump {
                                hexdump.render(window, cx)
                            } else {
                                self.starting.render(cx)
                            }
                        }
                    }),
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
    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(1920.), px(1080.)), cx);
        cx.bind_keys(vec![KeyBinding::new("ctrl-o", OpenFile, None)]);

        let _ = cx.text_system().add_fonts(vec![Assets
            .load("fonts/DiodrumCyrillic-Regular.ttf")
            .unwrap()
            .unwrap()]);

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
