use crate::*;
use cgmath::*;
use std::error::Error;

const TITLE_FONT_SIZE: f32 = 48.0;
const HEADER_FONT_SIZE: f32 = 32.0;
const ROW_FONT_SIZE: f32 = 32.0;

pub struct ResultsListScreen {
    need_font_recalc: bool,
    list_title: Label,
    table_headers: Vec<Label>,
    table_rows: Vec<Vec<Label>>,
}

impl ResultsListScreen {
    fn table_header_label(text: String, gfx_window: &mut GfxWindow) -> Label {
        Label::new(
            HEADER_FONT_SIZE,
            gfx_window.fonts.roboto_font_id,
            BLACK,
            text,
            gfx_window,
        )
    }

    fn table_cell_label(text: String, gfx_window: &mut GfxWindow) -> Label {
        Label::new(
            ROW_FONT_SIZE,
            gfx_window.fonts.roboto_font_id,
            BLACK,
            text,
            gfx_window,
        )
    }

    pub fn new(gfx_window: &mut GfxWindow) -> Self {
        let read_typing_results = match storage::read_results_from_file() {
            Ok(results) => results,
            Err(_) => storage::ReadTypingResults::default(),
        };
        if read_typing_results.records_need_upgrading {
            // do the upgrade
        }
        let mut table_rows = Vec::new();
        // FIXME: sort-by time reverse...
        // TODO: Click on column to sort by that column
        for typing_result in read_typing_results.results {
            table_rows.push(vec![
                Self::table_cell_label(format!("{}", typing_result.wpm), gfx_window),
                Self::table_cell_label(format!("{}", typing_result.correct_words), gfx_window),
                Self::table_cell_label(format!("{}", typing_result.incorrect_words), gfx_window),
                Self::table_cell_label(format!("{}", typing_result.backspaces), gfx_window),
                Self::table_cell_label(format!("{}", typing_result.time), gfx_window),
            ]);
        }
        Self {
            need_font_recalc: true,
            list_title: Label::new(
                TITLE_FONT_SIZE,
                gfx_window.fonts.roboto_font_id,
                BLACK,
                String::from("Previous typing speed results:"),
                gfx_window,
            ),
            table_headers: vec![
                Self::table_header_label(String::from("WPM"), gfx_window),
                Self::table_header_label(String::from("Correct words"), gfx_window),
                Self::table_header_label(String::from("Incorrect words"), gfx_window),
                Self::table_header_label(String::from("Backspaces"), gfx_window),
                Self::table_header_label(String::from("Date"), gfx_window),
            ],
            table_rows,
        }
    }

    fn col_width(&self, col: usize) -> f32 {
        let widest_col_width = self
            .table_rows
            .iter()
            .map(|row| row[col].rect.bounds.x)
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
            .iter()
            .map(|cell| cell.rect.bounds.y)
            .max_by(|height_a, height_b| {
                height_a
                    .partial_cmp(height_b)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .unwrap_or(0.0)
    }

    fn update_font_metrics(&mut self, _gfx_window: &mut GfxWindow) {
        let top_padding = 30.0;
        let padding_between_heading_and_table = 20.0;
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

        self.list_title.rect.position.y = top_padding;
        self.list_title.rect.position.x = left_padding;

        let top_of_table = top_padding + title_height + padding_between_heading_and_table;
        let inter_col_padding = 15.0;
        let mut x_offset = left_padding;
        for (i, table_header) in self.table_headers.iter_mut().enumerate() {
            table_header.rect.position.y = top_of_table;
            table_header.rect.position.x = x_offset;
            x_offset += *col_widths.get(i).expect("Column width to exist!") + inter_col_padding;
        }

        let inter_row_padding = 5.0;
        let mut y_offset = top_of_table + header_height;
        for row in self.table_rows.iter_mut() {
            let mut x_offset = left_padding;
            for (i, cell) in row.iter_mut().enumerate() {
                cell.rect.position.y = y_offset;
                cell.rect.position.x = x_offset;
                x_offset += *col_widths.get(i).expect("Column width to exist!") + inter_col_padding;
            }
            y_offset += max_row_height + inter_row_padding;
        }
    }
}

impl Screen for ResultsListScreen {
    fn maybe_change_to_screen(&self, _gfx_window: &mut GfxWindow) -> Option<Box<Screen>> {
        None
    }

    fn mouse_click(&mut self, _position: Vector2<f32>) {
        // check if mouse is positioned over one of the results rows
    }

    fn update(
        &mut self,
        _dt: f32,
        _mouse_position: Vector2<f32>,
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
            .clear(&gfx_window.quad_bundle.data.out_color, BG_COLOR);
        gfx_window
            .encoder
            .clear_depth(&gfx_window.quad_bundle.data.out_depth, 1.0);

        let mut title_section = self.list_title.section(gfx_window);
        title_section.bounds = self.list_title.rect.bounds.into();
        title_section.screen_position = self.list_title.rect.position.into();
        gfx_window.glyph_brush.queue(title_section);

        for header_label in &self.table_headers {
            let mut section = header_label.section(gfx_window);
            section.bounds = header_label.rect.bounds.into();
            section.screen_position = header_label.rect.position.into();
            gfx_window.glyph_brush.queue(section);
        }

        for row in &self.table_rows {
            for label in row {
                let mut section = label.section(gfx_window);
                section.bounds = label.rect.bounds.into();
                section.screen_position = label.rect.position.into();
                gfx_window.glyph_brush.queue(section);
            }
        }

        gfx_window.glyph_brush.draw_queued(
            &mut gfx_window.encoder,
            &gfx_window.quad_bundle.data.out_color,
            &gfx_window.quad_bundle.data.out_depth,
        )?;

        Ok(())
    }
}
