use std::time::Instant;
use std::{path::Path, rc::Rc};

use gpui::{AnyElement, App, Context, Entity, SharedString, Window, div, prelude::*};
use gpui_component::{
    ActiveTheme,
    table::{Column, Table, TableDelegate, TableState},
};
use rd_core::hex_string;

use crate::{Route, RustDump};

pub struct Hexdump {
    dump: Rc<Vec<Hexrow>>,
    table_delegate: HexDelegate,
    table_state: Entity<TableState<HexDelegate>>,
}

#[derive(Clone)]
pub struct Hexrow {
    pub offset: String,
    pub bytes: String,
    pub ascii: String,
}

#[derive(Clone)]
struct HexDelegate {
    data: Rc<Vec<Hexrow>>,
    columns: Vec<Column>,
}

impl HexDelegate {
    fn new(data: Vec<rd_core::OutputRow>, window: &mut Window) -> Self {
        let col_w = window.bounds().size.width.to_f64() / 3.;
        let data = data
            .iter()
            .map(|row| Hexrow {
                offset: hex_string(&row.offset.to_le_bytes()),
                bytes: row.ascii.clone(),
                ascii: row.bytes_string(),
            })
            .collect();
        Self {
            data: Rc::new(data),
            columns: vec![
                Column::new("offset", "Offset")
                    .width(col_w)
                    .resizable(false),
                Column::new("hex", "Hex").width(col_w).resizable(false),
                Column::new("ascii", "Ascii").width(col_w).resizable(false),
            ],
        }
    }
}

impl TableDelegate for HexDelegate {
    fn columns_count(&self, _: &App) -> usize {
        self.columns.len()
    }

    fn rows_count(&self, _: &App) -> usize {
        self.data.len()
    }

    fn column(&self, col_ix: usize, _: &App) -> &Column {
        &self.columns[col_ix]
    }

    fn render_td(
        &mut self,
        row_ix: usize,
        col_ix: usize,
        _: &mut Window,
        _: &mut Context<TableState<Self>>,
    ) -> impl IntoElement {
        let row = &self.data[row_ix];
        let col = &self.columns[col_ix];

        match col.key.as_ref() {
            "offset" => div().child(row.offset.clone()),
            "hex" => div().child(row.ascii.clone()),
            "ascii" => div().child(row.ascii.clone()),
            _ => div().child("".to_string()),
        }
    }
}

impl Hexdump {
    pub fn new(window: &mut Window, cx: &mut Context<RustDump>) -> Self {
        Self {
            dump: Rc::new(Vec::new()),
            table_delegate: HexDelegate::new(vec![], window),
            table_state: cx.new(|cx| TableState::new(HexDelegate::new(vec![], window), window, cx)),
        }
    }

    pub fn load_file(&mut self, path: &Path, cx: &mut Context<RustDump>, window: &mut Window) {
        let start = Instant::now();
        let mut dump = vec![];
        let _ = rd_core::create_dump(path.to_path_buf(), &mut dump);
        println!("Dump time: {}", start.elapsed().as_secs_f64());
        let delegate = HexDelegate::new(dump.clone(), window);
        let state = cx.new(|cx| TableState::new(delegate.clone(), window, cx));

        self.table_delegate = delegate;
        self.table_state = state;
    }
    pub fn render_route(&self, cx: &Context<RustDump>) -> AnyElement {
        div()
            .font_family(SharedString::from("Diodrum Cyrillic"))
            .size_full()
            .bg(cx.theme().background)
            .text_color(cx.theme().foreground)
            .child(Table::new(&self.table_state).bordered(false).stripe(false))
            .into_any_element()
    }
}

impl Route for Hexdump {
    fn render(&self, cx: &mut Context<RustDump>, _app: &RustDump) -> AnyElement {
        self.render_route(cx)
    }

    fn load(&mut self, cx: &mut Context<RustDump>, window: &mut Window, path: &std::path::Path) {
        self.load_file(path, cx, window);
    }
}
