use std::{fs, path::Path};
const DIR_NAMES: [&str; 16] = [
    "Export Directory",
    "Import Directory",
    "Resource Directory",
    "Exception Directory",
    "Security Directory",
    "Base Relocation Directory",
    "Debug Directory",
    "Architecture Directory",
    "Global Pointer Directory",
    "TLS Directory",
    "Load Configuration Directory",
    "Bound Import Directory",
    "Import Address Table Directory",
    "Delay Import Directory",
    "COM Descriptor Directory",
    "Reserved Directory",
];

use gpui::{
    AnyElement, Context, IntoElement, ParentElement, SharedString, Styled, Window, div,
    transparent_black,
};
use gpui_component::{
    ActiveTheme, Icon, StyledExt,
    button::{Button, ButtonCustomVariant, ButtonVariants},
};
use pe_parse::{
    ImageDataDirectory, ImageDosHeader, ImageFileHeader, OptionalHeaders, OptionalHeaders32,
    OptionalHeaders64,
};
use rd_core::push_hex;
use serde_json::{Map, Value};

use crate::{
    InfoDisplayPage, Route, RustDump,
    components::{
        asciiview::AsciiView,
        headertable::{HeaderData, HeaderTable},
        hexview::Hexview,
    },
};

pub struct Info {
    pe_header: Option<pe_parse::PEHeader>,
    custom_btn: ButtonCustomVariant,
    dos_table: HeaderTable,
    dos_stub_hexview: Hexview,
    dos_ascii_view: AsciiView,
    file_header_table: HeaderTable,
    opt_header_table: HeaderTable,
    data_dir_table: HeaderTable,
}

impl Info {
    pub fn new(window: &mut Window, cx: &mut Context<RustDump>) -> Self {
        let custom_button = ButtonCustomVariant::new(cx)
            .color(cx.theme().background)
            .foreground(cx.theme().foreground)
            .border(transparent_black())
            .hover(cx.theme().background)
            .active(cx.theme().accent);

        Self {
            pe_header: None,
            custom_btn: custom_button,
            dos_table: HeaderTable::new(window, cx),
            file_header_table: HeaderTable::new(window, cx),
            opt_header_table: HeaderTable::new(window, cx),
            dos_stub_hexview: Hexview::new(window, cx),
            dos_ascii_view: AsciiView::new(vec![], cx),
            data_dir_table: HeaderTable::new(window, cx),
        }
    }
    pub fn render_route(&self, cx: &mut Context<RustDump>, app: &RustDump) -> AnyElement {
        let sidebar = div()
            .v_flex()
            .text_left()
            .child(
                div()
                    .h_flex()
                    .child(
                        Button::new("dos_header")
                            .on_click(cx.listener(|app, _event, _window, _cx| {
                                app.info_page = InfoDisplayPage::DOSHeaders;
                            }))
                            .child(
                                Icon::new(Icon::empty())
                                    .path("icons/file-text.svg")
                                    .text_color(cx.theme().foreground),
                            )
                            .child("DOS Header")
                            .custom(self.custom_btn),
                    )
                    .gap_2(),
            )
            .child(
                div()
                    .h_flex()
                    .child(
                        Button::new("dos_stub")
                            .on_click(cx.listener(|app, _event, _window, _cx| {
                                app.info_page = InfoDisplayPage::DOSStub;
                            }))
                            .child(
                                Icon::new(Icon::empty())
                                    .path("icons/file-spreadsheet.svg")
                                    .text_color(cx.theme().foreground),
                            )
                            .child("DOS Stub")
                            .custom(self.custom_btn),
                    )
                    .gap_2(),
            )
            .child(
                div()
                    .h_flex()
                    .child(
                        Button::new("nt_headers")
                            .dropdown_caret(true)
                            .on_click(cx.listener(|app, _event, _window, _cx| {
                                app.expand_nt = !app.expand_nt;
                            }))
                            .child(
                                Icon::new(Icon::empty())
                                    .path("icons/list-tree.svg")
                                    .text_color(cx.theme().foreground),
                            )
                            .child("NT Hdrs")
                            .custom(self.custom_btn),
                    )
                    .gap_2(),
            )
            .child(
                div()
                    .ml_2()
                    .flex()
                    .text_left()
                    .justify_start()
                    .child(if app.expand_nt {
                        div().children(vec![
                            Button::new("file_header")
                                .flex()
                                .justify_start()
                                .text_left()
                                .on_click(cx.listener(|app, _event, _window, _cx| {
                                    app.info_page = InfoDisplayPage::FileHdr;
                                }))
                                .child(
                                    Icon::new(Icon::empty())
                                        .path("icons/file-spreadsheet.svg")
                                        .text_color(cx.theme().foreground),
                                )
                                .child("File Hdr")
                                .custom(self.custom_btn),
                            Button::new("opt_header")
                                .flex()
                                .justify_start()
                                .text_left()
                                .on_click(cx.listener(|app, _event, _window, _cx| {
                                    app.info_page = InfoDisplayPage::OptHdr;
                                }))
                                .child(
                                    Icon::new(Icon::empty())
                                        .path("icons/file-spreadsheet.svg")
                                        .text_color(cx.theme().foreground),
                                )
                                .child("Optional Hdr")
                                .custom(self.custom_btn),
                        ])
                    } else {
                        div()
                    }),
            )
            .child(
                div()
                    .h_flex()
                    .child(
                        Button::new("section_headers")
                            .dropdown_caret(true)
                            .on_click(cx.listener(|app, _event, _window, _cx| {
                                app.expand_section = !app.expand_section;
                            }))
                            .child(
                                Icon::new(Icon::empty())
                                    .path("icons/list-tree.svg")
                                    .text_color(cx.theme().foreground),
                            )
                            .child("Section Hdrs")
                            .custom(self.custom_btn),
                    )
                    .gap_2(),
            )
            .child(if app.expand_section {
                div().children(vec![""])
            } else {
                div()
            });

        div()
            .h_flex()
            .font_family(SharedString::from("Diodrum Cyrillic"))
            .pl_1()
            .size_full()
            .bg(cx.theme().background)
            .text_color(cx.theme().foreground)
            .child(
                div()
                    .v_flex()
                    .w_40()
                    .h_full()
                    .border_r_1()
                    .border_color(cx.theme().sidebar_border)
                    .child(sidebar),
            )
            .child(
                div().size_full().child(match app.info_page {
                    InfoDisplayPage::DOSHeaders => div().child(self.dos_table.render()).size_full(),
                    InfoDisplayPage::DOSStub => div()
                        .grid()
                        .grid_cols(4)
                        .grid_rows(1)
                        .child(div().child(self.dos_stub_hexview.render()).col_span(3))
                        .child(
                            div()
                                .child(self.dos_ascii_view.render(cx))
                                .mt(gpui::rems(2.)),
                        )
                        .size_full(),
                    InfoDisplayPage::FileHdr => {
                        div().child(self.file_header_table.render()).size_full()
                    }
                    InfoDisplayPage::OptHdr => div()
                        .grid()
                        .grid_cols(2)
                        .grid_rows(1)
                        .child(self.opt_header_table.render())
                        .child(self.data_dir_table.render())
                        .size_full(),
                }),
            )
            .into_any_element()
    }

    pub fn load_file(&mut self, path: &Path, cx: &mut Context<RustDump>, window: &mut Window) {
        let Ok(bytes) = fs::read(path) else {
            panic!("Failed to read file");
        };

        let pe_header = pe_parse::parse_pe_header(&bytes[..]).unwrap();

        // Load DOS header
        let dos_header = serde_json::to_value(&pe_header.dos_header);
        let Ok(dos_header) = dos_header else { return };
        let data = parse_data(dos_header, |idx| {
            let mut offset_string = String::new();
            let offset_bytes = ImageDosHeader::get_offset(idx).unwrap_or(0).to_be_bytes();
            offset_bytes
                .iter()
                .for_each(|b| push_hex(&mut offset_string, *b));
            offset_string
        });

        self.dos_table.load(data, window, cx);

        // Load file header
        let file_header = serde_json::to_value(&pe_header.nt_header.image_file_header);
        let Ok(file_header) = file_header else { return };
        let data = parse_data(file_header, |idx| {
            let mut offset_string = String::new();
            let offset_bytes = ImageFileHeader::get_offset(idx)
                .and_then(|offst| Some(offst + pe_header.dos_header.e_lfanew as u16))
                .unwrap_or(0)
                .to_be_bytes();
            offset_bytes
                .iter()
                .for_each(|b| push_hex(&mut offset_string, *b));
            offset_string
        });

        self.file_header_table.load(data, window, cx);

        // Load OPT headers
        match pe_header.nt_header.optional_headers {
            OptionalHeaders::OptionalHeaders32(opt32) => {
                let opt_header = serde_json::to_value(&opt32);
                let Ok(opt_header) = opt_header else { return };

                let dir_data =
                    parse_data_directory_from_array(&opt32.data_directory, |idx| idx.to_string());
                self.data_dir_table.load(dir_data, window, cx);

                let data = parse_data(opt_header, |idx| {
                    let mut offset_string = String::new();
                    let offset_bytes = OptionalHeaders32::get_offset(idx)
                        .and_then(|offst| Some(offst + pe_header.dos_header.e_lfanew as u16 + 24))
                        .unwrap_or(0)
                        .to_be_bytes();
                    offset_bytes
                        .iter()
                        .for_each(|b| push_hex(&mut offset_string, *b));
                    offset_string
                });
                self.opt_header_table.load(data, window, cx);
            }
            OptionalHeaders::OptionalHeaders64(opt64) => {
                let opt_header = serde_json::to_value(&opt64);
                let Ok(opt_header) = opt_header else { return };

                let dir_data =
                    parse_data_directory_from_array(&opt64.data_directory, |idx| idx.to_string());
                self.data_dir_table.load(dir_data, window, cx);

                let data = parse_data(opt_header, |idx| {
                    let mut offset_string = String::new();
                    let offset_bytes = OptionalHeaders64::get_offset(idx)
                        .and_then(|offst| Some(offst + pe_header.dos_header.e_lfanew as u16 + 24))
                        .unwrap_or(0)
                        .to_be_bytes();
                    offset_bytes
                        .iter()
                        .for_each(|b| push_hex(&mut offset_string, *b));
                    offset_string
                });
                self.opt_header_table.load(data, window, cx);
            }
        }

        self.pe_header = Some(pe_header);
    }
}

impl Route for Info {
    fn render(&self, cx: &mut Context<RustDump>, app: &RustDump) -> AnyElement {
        self.render_route(cx, app)
    }

    fn load(&mut self, cx: &mut Context<RustDump>, window: &mut Window, path: &Path) {
        self.load_file(path, cx, window);
        if let Some(pe_header) = &self.pe_header {
            self.dos_stub_hexview
                .load_data(pe_header.dos_stub.clone(), window, cx, 64);
            self.dos_ascii_view = AsciiView::new(pe_header.dos_stub.clone(), cx);
        }
    }
}

fn parse_data(
    data_value: Value,
    //data_chunk: Map<String, Value>,
    offset_fn: impl Fn(usize) -> String,
) -> Vec<HeaderData> {
    let serde_json::Value::Object(data_chunk) = data_value else {
        return vec![];
    };

    data_chunk
        .iter()
        .enumerate()
        .map(|(i, (k, v))| {
            let offset = offset_fn(i);
            return HeaderData {
                offset,
                name: k.clone(),
                value: v.clone(),
                meaning: String::from("Test"),
            };
        })
        .collect()
}

fn parse_data_directory_from_array(
    data_directory: &[ImageDataDirectory; 16],
    offset_fn: impl Fn(usize) -> String,
) -> Vec<HeaderData> {
    data_directory
        .iter()
        .enumerate()
        .map(|(i, dir)| {
            let dir_name = DIR_NAMES.get(i).copied().unwrap_or("Unknown Directory");

            HeaderData {
                offset: offset_fn(i),
                name: dir_name.to_string(),
                value: Value::String(format!(
                    "VA: 0x{:08X}, Size: {} (0x{:X})",
                    dir.virtual_address, dir.size, dir.size
                )),
                meaning: if dir.virtual_address == 0 && dir.size == 0 {
                    "Not present".to_string()
                } else {
                    "Present".to_string()
                },
            }
        })
        .collect()
}
