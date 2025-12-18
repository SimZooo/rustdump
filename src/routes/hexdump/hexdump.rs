use std::{rc::Rc, sync::Arc};

use gpui::{div, prelude::*, App, Context, Div, Entity, SharedString, Window};
use gpui_component::{
    table::{Column, Table, TableDelegate, TableState},
    ActiveTheme,
};

use crate::RustDump;

pub struct Hexdump {
    dump: Rc<Vec<core::OutputRow>>,
    table_delegate: HexDelegate,
    table_state: Entity<TableState<HexDelegate>>,
}

#[derive(Clone)]
struct HexDelegate {
    data: Rc<Vec<core::OutputRow>>,
    columns: Vec<Column>,
}

impl HexDelegate {
    fn new(data: Rc<Vec<core::OutputRow>>, window: &mut Window) -> Self {
        let col_w = window.bounds().size.width.to_f64() / 3.;
        Self {
            data: data.clone(),
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
            "offset" => div().child(format!("{:08X}", row.offset)).border_0(),
            "hex" => div().child(row.ascii.clone()),
            "ascii" => div().child(row.bytes_string()),
            _ => div().child("".to_string()),
        }
    }
}

impl Hexdump {
    pub fn new(app: &RustDump, cx: &mut Context<RustDump>, window: &mut Window) -> Self {
        if let Some(file) = app.curr_file.clone() {
            let dump = Rc::new(core::create_dump(file));
            let delegate = HexDelegate::new(dump.clone(), window);
            let state = cx.new(|cx| TableState::new(delegate.clone(), window, cx));
            return Self {
                dump,
                table_delegate: delegate,
                table_state: state,
            };
        }

        let dump = Rc::new(Vec::new());
        let delegate = HexDelegate::new(dump.clone(), window);
        let state = cx.new(|cx| TableState::new(delegate.clone(), window, cx));
        Self {
            dump,
            table_delegate: delegate,
            table_state: state,
        }
    }
    pub fn render(&mut self, window: &mut Window, cx: &mut Context<RustDump>) -> Div {
        div()
            .font_family(SharedString::from("Diodrum Cyrillic"))
            .size_full()
            .bg(cx.theme().foreground)
            .text_color(cx.theme().foreground)
            .child(Table::new(&self.table_state).bordered(false).stripe(false))
    }
}
