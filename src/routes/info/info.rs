use std::{fs, path::Path};

use gpui::{
    AnyElement, App, AppContext, Context, Entity, IntoElement, ParentElement, SharedString, Styled,
    Window, div, transparent_black,
};
use gpui_component::{
    ActiveTheme, Icon, StyledExt,
    button::{Button, ButtonCustomVariant, ButtonVariants},
    table::{Column, Table, TableDelegate, TableState},
};
use serde_json::Value;

use crate::{InfoDisplayPage, Route, RustDump};

pub struct Info {
    pe_header: Option<pe_parse::PEHeader>,
    custom_btn: ButtonCustomVariant,
    table_state: Entity<TableState<DosHeaderTable>>,
}

#[derive(Debug)]
pub struct DosHeaderData {
    offset: usize,
    name: String,
    value: Value,
    meaning: String,
}

#[derive(Debug)]
pub struct DosHeaderTable {
    data: Vec<DosHeaderData>,
    cols: Vec<Column>,
}

impl DosHeaderTable {
    pub fn new(data: Vec<DosHeaderData>) -> Self {
        Self {
            data,
            cols: vec![
                Column::new("offset", "Offset".to_string()),
                Column::new("name", "Name".to_string()),
                Column::new("value", "Value".to_string()),
                Column::new("meaning", "Meaning".to_string()),
            ],
        }
    }
}

impl TableDelegate for DosHeaderTable {
    fn columns_count(&self, _: &App) -> usize {
        self.cols.len()
    }

    fn rows_count(&self, _: &App) -> usize {
        self.data.len()
    }

    fn column(&self, col_ix: usize, _: &App) -> &Column {
        &self.cols[col_ix]
    }

    fn render_td(
        &mut self,
        row_ix: usize,
        col_ix: usize,
        _: &mut Window,
        _: &mut Context<TableState<Self>>,
    ) -> impl IntoElement {
        let row = &self.data[row_ix];
        let col = &self.cols[col_ix];

        match col.key.as_ref() {
            "offset" => row.offset.to_string(),
            "name" => row.name.clone(),
            "value" => row.value.to_string(),
            "meaning" => row.meaning.clone(),
            _ => "".to_string(),
        }
    }
}

impl Info {
    pub fn new(window: &mut Window, cx: &mut Context<RustDump>) -> Self {
        let custom_button = ButtonCustomVariant::new(cx)
            .color(cx.theme().background)
            .foreground(cx.theme().foreground)
            .border(transparent_black())
            .hover(cx.theme().background)
            .active(cx.theme().accent);

        let dos_header_table = DosHeaderTable::new(vec![]);
        let dos_table_state = cx.new(|cx| TableState::new(dos_header_table, window, cx));
        Self {
            pe_header: None,
            custom_btn: custom_button,
            table_state: dos_table_state,
        }
    }
    pub fn render_route(&self, cx: &mut Context<RustDump>, app: &RustDump) -> AnyElement {
        let sidebar = div()
            .v_flex()
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
                            .on_click(cx.listener(|app, _event, _window, _cx| {
                                app.info_page = InfoDisplayPage::NTHeaders;
                            }))
                            .child(
                                Icon::new(Icon::empty())
                                    .path("icons/list-tree.svg")
                                    .text_color(cx.theme().foreground),
                            )
                            .child("NT Headers")
                            .custom(self.custom_btn),
                    )
                    .gap_2(),
            )
            .child(
                div()
                    .h_flex()
                    .child(
                        Button::new("section_headers")
                            .on_click(cx.listener(|app, _event, _window, _cx| {
                                app.info_page = InfoDisplayPage::SectionHeaders;
                            }))
                            .child(
                                Icon::new(Icon::empty())
                                    .path("icons/list-tree.svg")
                                    .text_color(cx.theme().foreground),
                            )
                            .child("Section Headers")
                            .custom(self.custom_btn),
                    )
                    .gap_2(),
            );

        div()
            .h_flex()
            .font_family(SharedString::from("Diodrum Cyrillic"))
            .pl_3()
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
            .child(div().size_full().child(match app.info_page {
                InfoDisplayPage::DOSHeaders => {
                    div().child(Table::new(&self.table_state)).size_full()
                }
                InfoDisplayPage::DOSStub => div().child("DOS Stub"),
                InfoDisplayPage::NTHeaders => div().child("NT Headers"),
                InfoDisplayPage::SectionHeaders => div().child("Section Headers"),
            }))
            .into_any_element()
    }

    pub fn load_file(&mut self, path: &Path, cx: &mut Context<RustDump>, window: &mut Window) {
        let Ok(bytes) = fs::read(path) else {
            panic!("Failed to read file");
        };

        let pe_header = pe_parse::parse_pe_header(&bytes[..]).unwrap();
        let dos_header = serde_json::to_value(&pe_header.dos_header);
        let Ok(dos_header) = dos_header else { return };
        let serde_json::Value::Object(dos_header_obj) = dos_header else {
            return;
        };

        let data = dos_header_obj
            .iter()
            .map(|(k, v)| DosHeaderData {
                offset: 0,
                name: k.clone(),
                value: v.clone(),
                meaning: String::from("Test"),
            })
            .collect();

        println!("{:?}", data);
        let dos_delegate = DosHeaderTable::new(data);
        self.table_state = cx.new(|cx| TableState::new(dos_delegate, window, cx));

        //self.dos_header_table = DosHeaderTable::new();
        self.pe_header = Some(pe_header);
    }
}

impl Route for Info {
    fn render(&self, cx: &mut Context<RustDump>, app: &RustDump) -> AnyElement {
        self.render_route(cx, app)
    }

    fn load(&mut self, cx: &mut Context<RustDump>, window: &mut Window, path: &Path) {
        self.load_file(path, cx, window);
    }
}
