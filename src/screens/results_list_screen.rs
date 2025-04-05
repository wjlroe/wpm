use crate::layout::ElementLayout;
use crate::screens;
use crate::*;
use cgmath::*;
use std::error::Error;
use winit::event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent};

const TITLE_FONT_SIZE: f32 = 48.0;
const HEADER_FONT_SIZE: f32 = 32.0;
const ROW_FONT_SIZE: f32 = 32.0;
const TABLE_OUTLINE_WIDTH: f32 = 3.0;

const TABLE_OUTLINE_COLOR: ColorArray = MAGENTA;
const TABLE_HEADER_UNDERLINE: ColorArray = MAGENTA;

fn table_header_label(text: String, gfx_window: &mut GfxWindow) -> Label {
    Label::new(
        HEADER_FONT_SIZE,
        gfx_window.fonts.roboto_font_id,
        TEXT_COLOR,
        text,
        gfx_window,
    )
}

fn table_cell_label(text: String, gfx_window: &mut GfxWindow) -> Label {
    Label::new(
        ROW_FONT_SIZE,
        gfx_window.fonts.roboto_font_id,
        TEXT_COLOR,
        text,
        gfx_window,
    )
}

struct TableRow {
    cells: [Label; 3],
    row_rect: Rect,
    typing_result: TypingResult,
    ui_state: UIState,
}

impl TableRow {
    fn new(gfx_window: &mut GfxWindow, typing_result: TypingResult) -> Self {
        let datetime = if let Some(dt) = typing_result.datetime() {
            format!("{}", dt.format("%H:%M %v"))
        } else {
            "?".to_string()
        };
        let wpm = typing_result.wpm;
        let notes = typing_result.notes.clone();
        Self {
            typing_result,
            cells: [
                table_cell_label(datetime, gfx_window),
                table_cell_label(format!("{}", wpm), gfx_window),
                table_cell_label(notes, gfx_window),
            ],
            row_rect: Rect::default(),
            ui_state: UIState::default(),
        }
    }
}

pub struct ResultsListScreen {
    need_font_recalc: bool,
    back_label: Label,
    list_title: Label,
    table_headers: [Label; 3],
    table_rows: Vec<TableRow>,
    table_rect: Rect,
    table_header_rect: Rect,
    table_rows_rect: Rect,
}

impl ResultsListScreen {
    pub fn new(gfx_window: &mut GfxWindow) -> Self {
        let mut read_typing_results = match storage::read_results_from_file() {
            Ok(results) => results,
            Err(err) => {
                println!("Error reading results from file: {:?}", err);
                storage::ReadTypingResults::default()
            }
        };
        if read_typing_results.records_need_upgrading {
            // TODO: do the record upgrade
        }
        let mut table_rows = Vec::new();
        // TODO: Click on column to sort by that column
        read_typing_results
            .results
            .sort_unstable_by_key(|result| -(result.time as i64));
        for typing_result in read_typing_results.results {
            table_rows.push(TableRow::new(gfx_window, typing_result));
        }
        Self {
            need_font_recalc: true,
            back_label: gfx_window.back_label(),
            list_title: Label::new(
                TITLE_FONT_SIZE,
                gfx_window.fonts.roboto_font_id,
                TEXT_COLOR,
                String::from("Typing speed results:"),
                gfx_window,
            ),
            table_headers: [
                table_header_label(String::from("Date"), gfx_window),
                table_header_label(String::from("WPM"), gfx_window),
                table_header_label(String::from("Notes"), gfx_window),
            ],
            table_rows,
            table_rect: Rect::default(),
            table_header_rect: Rect::default(),
            table_rows_rect: Rect::default(),
        }
    }

    fn set_highlight_row(&mut self, hl_idx: Option<usize>) {
        for (idx, row) in &mut self.table_rows.iter_mut().enumerate() {
            if Some(idx) == hl_idx {
                row.ui_state.highlighted = true;
            } else {
                row.ui_state.highlighted = false;
            }
        }
    }

    fn highlighted_row(&self) -> Option<usize> {
        self.table_rows.iter().enumerate().find_map(|(idx, row)| {
            if row.ui_state.highlighted {
                Some(idx)
            } else {
                None
            }
        })
    }

    fn move_highlight(&mut self, amount: i32) {
        let num_rows = self.table_rows.len() as i32;
        if num_rows == 0 {
            return;
        }
        let last_row = num_rows - 1;
        let mut highlighted_row = self.highlighted_row();
        if highlighted_row.is_none() {
            if amount > 0 {
                highlighted_row = Some(0);
            } else {
                highlighted_row = Some(last_row as usize);
            }
        } else {
            let mut new_row = highlighted_row.unwrap() as i32 + amount;
            new_row %= num_rows;
            if new_row < 0 {
                new_row += num_rows;
            }
            highlighted_row = Some(new_row as usize);
        }
        assert!(highlighted_row.unwrap() < self.table_rows.len());
        self.set_highlight_row(highlighted_row);
    }

    fn selected_row(&self) -> Option<usize> {
        self.table_rows.iter().enumerate().find_map(|(idx, row)| {
            if row.ui_state.selected {
                Some(idx)
            } else {
                None
            }
        })
    }

    fn set_selected_row(&mut self, sel_idx: Option<usize>) {
        for (idx, row) in &mut self.table_rows.iter_mut().enumerate() {
            if Some(idx) == sel_idx {
                row.ui_state.selected = true;
            } else {
                row.ui_state.selected = false;
            }
        }
    }

    fn select_highlighted_row(&mut self) {
        self.set_selected_row(self.highlighted_row());
    }

    fn col_width(&self, col: usize) -> f32 {
        let widest_col_width = self
            .table_rows
            .iter()
            .map(|row| row.cells[col].rect.bounds.x)
            .max_by(|width_a, width_b| {
                width_a
                    .partial_cmp(width_b)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .unwrap_or(0.0);
        f32::max(widest_col_width, self.table_headers[col].rect.bounds.x)
    }

    fn row_height(&self, row: usize) -> f32 {
        self.table_rows[row]
            .cells
            .iter()
            .map(|cell| cell.rect.bounds.y)
            .max_by(|height_a, height_b| {
                height_a
                    .partial_cmp(height_b)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .unwrap_or(0.0)
    }

    fn update_font_metrics(&mut self, gfx_window: &mut GfxWindow) {
        let top_padding = 30.0;
        let padding_between_heading_and_table = 60.0;
        let left_padding = 15.0;

        let title_height = self.list_title.rect.bounds.y;

        let header_height = self
            .table_headers
            .iter()
            .map(|label| label.rect.bounds.y)
            .max_by(|height_a, height_b| {
                height_a
                    .partial_cmp(height_b)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .unwrap_or(0.0);

        let col_widths = (0..self.table_headers.len())
            .map(|col| self.col_width(col))
            .collect::<Vec<_>>();

        let row_heights = (0..self.table_rows.len()).map(|row| self.row_height(row));
        let max_row_height = row_heights
            .max_by(|height_a, height_b| {
                height_a
                    .partial_cmp(height_b)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .unwrap_or(0.0);

        self.back_label.rect.position.x = left_padding;
        self.back_label.rect.position.y = top_padding;

        let mut horizontal_layout = ElementLayout::horizontal(gfx_window.window_dim());
        let list_title_elem = horizontal_layout.add_bounds(self.list_title.rect.bounds);
        horizontal_layout.calc_positions();
        self.list_title.rect.position = horizontal_layout.element_position(list_title_elem);
        self.list_title.rect.position.y = top_padding;

        let top_of_table = top_padding + title_height + padding_between_heading_and_table;
        self.table_rect.position.y = top_of_table;
        self.table_rect.position.x = left_padding;
        let inter_col_padding = 55.0;
        let mut x_offset = left_padding;
        for (i, table_header) in self.table_headers.iter_mut().enumerate() {
            table_header.rect.position.y = top_of_table;
            table_header.rect.position.x = x_offset;
            let col_width = *col_widths.get(i).expect("Column width to exist!") + inter_col_padding;
            x_offset += col_width;
            self.table_rect.bounds.x += col_width;
        }

        self.table_rect.bounds.y += header_height;
        self.table_header_rect.bounds.y = header_height;

        let gap_between_headers_and_rows = 20.0;

        self.table_rect.bounds.y += gap_between_headers_and_rows;
        self.table_rows_rect.position.y =
            top_of_table + header_height + gap_between_headers_and_rows;
        self.table_rows_rect.position.x = left_padding;

        let inter_row_padding = 5.0;
        let row_height = max_row_height + inter_row_padding;
        let mut y_offset = top_of_table + header_height + gap_between_headers_and_rows;
        for table_row in self.table_rows.iter_mut() {
            let mut x_offset = left_padding;
            table_row.row_rect.position.x = x_offset;
            table_row.row_rect.position.y = y_offset;
            table_row.row_rect.bounds.y = row_height;
            for (i, cell) in table_row.cells.iter_mut().enumerate() {
                // FIXME: way to right-align these within their column
                cell.rect.position.x = x_offset;
                cell.rect.position.y = y_offset;
                let cell_width =
                    *col_widths.get(i).expect("Column width to exist!") + inter_col_padding;
                x_offset += cell_width;
                table_row.row_rect.bounds.x += cell_width;
            }
            y_offset += row_height;
            self.table_rect.bounds.y += row_height;
            self.table_rows_rect.bounds.y += row_height;
        }

        // grow the table_rect according to the outline width
        self.table_rect.position.x -= 2.0 * TABLE_OUTLINE_WIDTH;
        self.table_rect.position.y -= 2.0 * TABLE_OUTLINE_WIDTH;
        self.table_rect.bounds.x += 2.0 * TABLE_OUTLINE_WIDTH;
        self.table_rect.bounds.y += 2.0 * TABLE_OUTLINE_WIDTH;
        self.table_header_rect.position.y = self.table_rect.position.y;
        self.table_header_rect.position.x = self.table_rect.position.x;

        self.table_header_rect.bounds.x = self.table_rect.bounds.x;
        self.table_rows_rect.bounds.x = self.table_rect.bounds.x;
    }
}

impl Screen for ResultsListScreen {
    fn maybe_change_to_screen(
        &self,
        gfx_window: &mut GfxWindow,
        _config: &Config,
    ) -> Option<Box<dyn Screen>> {
        if self.back_label.ui_state.pressed {
            Some(Box::new(screens::Menu::new(gfx_window)))
        } else if let Some(goto_row) = self.selected_row() {
            if let Some(table_row) = self.table_rows.get(goto_row) {
                Some(Box::new(screens::ResultsScreen::new(
                    table_row.typing_result.clone(),
                    false,
                    gfx_window,
                )))
            } else {
                None
            }
        } else {
            None
        }
    }

    fn process_event(&mut self, event: &Event<()>, _gfx_window: &mut GfxWindow) -> bool {
        let mut update_and_render = false;
        if let Event::WindowEvent {
            event: win_event, ..
        } = event
        {
            match win_event {
                WindowEvent::KeyboardInput {
                    input: keyboard_input,
                    ..
                } => match keyboard_input {
                    KeyboardInput {
                        state: ElementState::Pressed,
                        modifiers,
                        virtual_keycode,
                        ..
                    } => {
                        if *modifiers == NO_MODS {
                            if let Some(virtual_keycode) = virtual_keycode {
                                match virtual_keycode {
                                    VirtualKeyCode::Down => self.move_highlight(1),
                                    VirtualKeyCode::Up => self.move_highlight(-1),
                                    VirtualKeyCode::Return => self.select_highlighted_row(),
                                    _ => {}
                                }
                                update_and_render = true;
                            }
                        }
                    }
                    _ => {}
                },
                _ => {}
            }
        }
        update_and_render
    }

    fn mouse_click(&mut self, position: Vector2<f32>) {
        // check if mouse is positioned over one of the results rows
        if self.back_label.rect.contains_point(position) {
            self.back_label.ui_state.pressed = true;
        }

        if self.table_rect.contains_point(position) {
            // find the header
            // find the row
            let mut selected_row = None;
            for (i, table_row) in self.table_rows.iter().enumerate() {
                if table_row.row_rect.contains_point(position) {
                    selected_row = Some(i);
                }
            }
            self.set_highlight_row(selected_row);
            self.select_highlighted_row();
        }
    }

    fn update(
        &mut self,
        _dt: f32,
        _mouse_position: Vector2<f32>,
        _config: &Config,
        gfx_window: &mut GfxWindow,
    ) -> bool {
        if self.need_font_recalc {
            self.update_font_metrics(gfx_window);
            self.need_font_recalc = false;
            true
        } else {
            false
        }
    }

    fn window_resized(&mut self, gfx_window: &mut GfxWindow) {
        self.update_font_metrics(gfx_window);
    }

    fn render(&self, _dt: f32, gfx_window: &mut GfxWindow) -> Result<(), Box<dyn Error>> {
        gfx_window
            .encoder
            .clear(&gfx_window.quad_bundle.data.out_color, bg_color());
        gfx_window
            .encoder
            .clear_depth(&gfx_window.quad_bundle.data.out_depth, 1.0);

        let mut header_underline_rect = self.table_header_rect;
        header_underline_rect.position.y += header_underline_rect.bounds.y + 10.0;
        header_underline_rect.bounds.y = 3.0;
        gfx_window.draw_quad(TABLE_HEADER_UNDERLINE, &header_underline_rect, 1.0);
        gfx_window.draw_outline(
            TABLE_OUTLINE_COLOR,
            &self.table_rect,
            1.0 - 0.1,
            TABLE_OUTLINE_WIDTH,
        );
        if let Some(highlighted_row_idx) = self.highlighted_row() {
            if let Some(table_row) = self.table_rows.get(highlighted_row_idx) {
                let bg = if current_bg_color() == BackgroundColor::Dark {
                    LIGHT_BG_COLOR
                } else {
                    DARK_BG_COLOR
                };
                gfx_window.draw_quad(bg, &table_row.row_rect, 1.0 - 0.3);
            }
        }

        gfx_window.queue_label(&self.back_label);
        gfx_window.queue_label(&self.list_title);

        for header_label in &self.table_headers {
            gfx_window.queue_label(header_label);
        }

        for row in &self.table_rows {
            for label in &row.cells {
                gfx_window.queue_label(label);
            }
        }

        gfx_window
            .glyph_brush
            .use_queue()
            .depth_target(&gfx_window.quad_bundle.data.out_depth)
            .draw(
                &mut gfx_window.encoder,
                &gfx_window.quad_bundle.data.out_color,
            )?;

        Ok(())
    }
}
