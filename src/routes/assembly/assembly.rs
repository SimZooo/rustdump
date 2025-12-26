use std::{ops::Range, path::Path, rc::Rc};

use gpui::{
    AnyElement, Background, Context, Div, Entity, HighlightStyle, Pixels, SharedString, Size,
    Window, div, prelude::*, px, size,
};
use gpui_component::{
    ActiveTheme, Rope, VirtualListScrollHandle, highlighter::SyntaxHighlighter, input::InputState,
    scroll::Scrollbar, v_virtual_list, white,
};
use iced_x86::{SpecializedFormatter, SpecializedFormatterTraitOptions};

struct AssemblyEditor {
    editor: Entity<InputState>,
}

impl AssemblyEditor {}

use crate::{Route, RustDump};
use rd_core;

struct TraitOptions;
impl SpecializedFormatterTraitOptions for TraitOptions {
    const ENABLE_DB_DW_DD_DQ: bool = false;
}

type CustomFormatter = SpecializedFormatter<TraitOptions>;

pub struct Assembly {
    data: Rc<Vec<SharedString>>,
    item_sizes: Rc<Vec<Size<Pixels>>>,
    scroll_handle: VirtualListScrollHandle,
}

impl Assembly {
    pub fn new(window: &mut Window, cx: &mut Context<RustDump>) -> Self {
        Self {
            data: Rc::new(vec![]),
            scroll_handle: VirtualListScrollHandle::new(),
            item_sizes: Rc::new(vec![]),
        }
    }

    pub fn load_file(&mut self, path: &Path, cx: &mut Context<RustDump>, window: &mut Window) {
        println!("loading assembly");
        let instructions = rd_core::load_assembly(path);
        let mut formatter = CustomFormatter::new();
        let mut item_sizes = vec![];

        let mut out_vec = vec![];
        for instr in instructions {
            let mut output = String::new();
            output.clear();
            formatter.format(&instr, &mut output);

            item_sizes.push(size(px(16. * output.len() as f32), px(22.)));

            let line = SharedString::new(output);
            out_vec.push(line);
        }
        self.item_sizes = Rc::new(item_sizes);

        self.data = Rc::new(out_vec);
    }
    pub fn render_route(&self, cx: &Context<RustDump>) -> AnyElement {
        let data = self.data.clone();
        div()
            .font_family(SharedString::from("Diodrum Cyrillic"))
            .size_full()
            .bg(cx.theme().background)
            .text_color(cx.theme().foreground)
            .child(
                v_virtual_list(
                    cx.entity().clone(),
                    "assembly_list",
                    self.item_sizes.clone(),
                    move |_view, visible_range, _, cx| {
                        visible_range
                            .map(|ix| {
                                div()
                                    .child(data[ix].clone())
                                    .hover(|s| s.bg(cx.theme().info_hover).h(gpui::px(22.)))
                            })
                            .collect()
                    },
                )
                .track_scroll(&self.scroll_handle),
            )
            .child(
                div()
                    .absolute()
                    .top_0()
                    .left_0()
                    .bottom_0()
                    .child(Scrollbar::new(&self.scroll_handle)),
            )
            .size_full()
            .into_any_element()
    }
}

fn create_highlighted(text: SharedString, styles: Vec<(Range<usize>, HighlightStyle)>) -> Div {
    let mut text_element = div();
    let text = text.clone();
    for (range, style) in styles {
        let span_text = SharedString::new(&text[range]);

        text_element = text_element.child(match style.color {
            Some(color) => div().text_color(color).child(span_text.clone()),
            None => div().child(span_text.clone()),
        });
    }
    text_element
}

impl Route for Assembly {
    fn render(&self, cx: &mut Context<RustDump>, _app: &RustDump) -> AnyElement {
        self.render_route(cx)
    }

    fn load(&mut self, cx: &mut Context<RustDump>, window: &mut Window, path: &std::path::Path) {
        self.load_file(path, cx, window);
    }
}
