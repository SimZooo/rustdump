use gpui::{
    App, AppContext, Context, Entity, IntoElement, ParentElement, Render, SharedString, Styled,
    Window, div,
};
use gpui_component::table::{Column, Table, TableDelegate, TableState};
use rd_core::push_hex;

use crate::RustDump;

pub struct HexviewData {
    offset: String,
    hex_data: Vec<String>,
}

struct HexviewDelegate {
    data: Vec<HexviewData>,
    cols: Vec<Column>,
}

impl HexviewDelegate {
    pub fn new(data: Vec<HexviewData>) -> Self {
        let columns_chars = "0123456789ABCDEF"
            .as_bytes()
            .iter()
            .map(|b| *b as char)
            .collect::<Vec<char>>();
        let mut cols = columns_chars
            .iter()
            .map(|ch| Column::new(ch.to_ascii_lowercase().to_string(), ch.to_string()).width(50.))
            .collect::<Vec<Column>>();
        cols.insert(0, Column::new("offset", "Offset").movable(false).width(75.));
        Self { data, cols }
    }
}

impl TableDelegate for HexviewDelegate {
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
            "offset" => div().child(row.offset.clone()),
            _ => div()
                .child(row.hex_data[col_ix - 1].to_string())
                .text_center(),
        }
    }
}

pub struct Hexview {
    state: Entity<TableState<HexviewDelegate>>,
}

impl Hexview {
    pub fn new(window: &mut Window, cx: &mut Context<RustDump>) -> Self {
        let delegate = HexviewDelegate::new(vec![]);
        let state = cx.new(|cx| TableState::new(delegate, window, cx));
        Self { state }
    }

    pub fn render(&self) -> impl IntoElement {
        Table::new(&self.state).bordered(false).stripe(false)
    }

    pub fn load_data(
        &mut self,
        data: Vec<u8>,
        window: &mut Window,
        cx: &mut Context<RustDump>,
        idx_offset: u16,
    ) {
        let chunks = data.chunks(16);
        let hex_data = chunks
            .into_iter()
            .map(|d| {
                let bt_arr = d
                    .iter()
                    .map(|b| {
                        let mut hex = String::with_capacity(2);
                        push_hex(&mut hex, *b);
                        hex
                    })
                    .collect::<Vec<String>>();
                bt_arr
            })
            .enumerate()
            .map(|(i, d)| {
                let mut offset = String::new();
                (i as u16 * 16 + idx_offset)
                    .to_le_bytes()
                    .iter()
                    .for_each(|b| push_hex(&mut offset, *b));

                HexviewData {
                    offset,
                    hex_data: d,
                }
            })
            .collect();
        let delegate = HexviewDelegate::new(hex_data);
        self.state = cx.new(|cx| TableState::new(delegate, window, cx));
    }
}
