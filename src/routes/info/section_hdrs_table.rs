use gpui::{App, AppContext, Context, Entity, IntoElement, Window};
use gpui_component::{
    Sizable,
    table::{Column, Table, TableDelegate, TableState},
};
use pe_parse::SectionHeader;
use rd_core::hex_string;
use serde_json::Value;

use crate::RustDump;

pub struct SectionsTable {
    pub table_state: Entity<TableState<SectionsTableDelegate>>,
}

impl SectionsTable {
    pub fn new(window: &mut Window, cx: &mut Context<RustDump>) -> Self {
        let header_table = SectionsTableDelegate::new(vec![]);
        let table_state = cx.new(|cx| TableState::new(header_table, window, cx));
        Self { table_state }
    }

    pub fn load(
        &mut self,
        data: &Vec<SectionHeader>,
        window: &mut Window,
        cx: &mut Context<RustDump>,
        image_base: u64,
    ) {
        let data = data
            .iter()
            .map(|sct| SectionData {
                name: String::from_utf8_lossy(&sct.name).to_string(),
                addr: hex_string(&(sct.virtual_address as u64 + image_base).to_le_bytes()),
                raw_size: sct.size_of_raw_data.to_string(),
                characteristics: hex_string(&sct.characteristics.to_le_bytes()),
            })
            .collect();

        let delegate = SectionsTableDelegate::new(data);
        self.table_state = cx.new(|cx| TableState::new(delegate, window, cx));
    }

    pub fn render(&self) -> impl IntoElement {
        Table::new(&self.table_state).stripe(false).bordered(false)
    }
}

#[derive(Debug)]
pub struct SectionData {
    pub name: String,
    pub addr: String,
    pub raw_size: String,
    pub characteristics: String,
}

#[derive(Debug)]
pub struct SectionsTableDelegate {
    data: Vec<SectionData>,
    cols: Vec<Column>,
}

impl SectionsTableDelegate {
    pub fn new(data: Vec<SectionData>) -> Self {
        Self {
            data,
            cols: vec![
                Column::new("name", "Name".to_string())
                    .resizable(true)
                    .width(75.),
                Column::new("addr", "Addr".to_string())
                    .resizable(true)
                    .width(200.),
                Column::new("raw_size", "Raw Size".to_string())
                    .resizable(true)
                    .width(150.),
                Column::new("chars", "Characteristics".to_string())
                    .resizable(true)
                    .width(175.),
            ],
        }
    }
}

impl TableDelegate for SectionsTableDelegate {
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
            "name" => row.name.clone(),
            "addr" => row.addr.clone(),
            "raw_size" => row.raw_size.clone(),
            "chars" => row.characteristics.clone(),
            _ => "".to_string(),
        }
    }
}
