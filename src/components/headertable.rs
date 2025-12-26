use gpui::{App, AppContext, Context, Entity, IntoElement, Window};
use gpui_component::{
    Sizable,
    table::{Column, Table, TableDelegate, TableState},
};
use serde_json::Value;

use crate::RustDump;

pub struct HeaderTable {
    pub table_state: Entity<TableState<HeaderTableDelegate>>,
}

impl HeaderTable {
    pub fn new(window: &mut Window, cx: &mut Context<RustDump>) -> Self {
        let header_table = HeaderTableDelegate::new(vec![]);
        let table_state = cx.new(|cx| TableState::new(header_table, window, cx));
        Self { table_state }
    }

    pub fn load(&mut self, data: Vec<HeaderData>, window: &mut Window, cx: &mut Context<RustDump>) {
        let delegate = HeaderTableDelegate::new(data);
        self.table_state = cx.new(|cx| TableState::new(delegate, window, cx));
    }

    pub fn render(&self) -> impl IntoElement {
        Table::new(&self.table_state).stripe(false).bordered(false)
    }
}

#[derive(Debug)]
pub struct HeaderData {
    pub offset: String,
    pub name: String,
    pub value: Value,
    pub meaning: String,
}

#[derive(Debug)]
pub struct HeaderTableDelegate {
    data: Vec<HeaderData>,
    cols: Vec<Column>,
}

impl HeaderTableDelegate {
    pub fn new(data: Vec<HeaderData>) -> Self {
        Self {
            data,
            cols: vec![
                Column::new("offset", "Offset".to_string())
                    .resizable(true)
                    .width(75.),
                Column::new("name", "Name".to_string())
                    .resizable(true)
                    .width(150.),
                Column::new("value", "Value".to_string())
                    .resizable(true)
                    .width(250.),
                Column::new("meaning", "Meaning".to_string())
                    .resizable(true)
                    .width(175.),
            ],
        }
    }
}

impl TableDelegate for HeaderTableDelegate {
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
