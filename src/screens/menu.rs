use crate::layout::ElementLayout;
use crate::screens;
use crate::*;
use cgmath::*;
use glutin::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use std::error::Error;

const MENU_FONT_SIZE: f32 = 48.0;

pub struct Menu {
    need_font_recalc: bool,
    typing_test_label: Label,
    start_typing_test: bool,
    results_list_label: Label,
    show_results_list: bool,
}

impl Menu {
    pub fn new(gfx_window: &mut GfxWindow) -> Self {
        Self {
            need_font_recalc: true,
            typing_test_label: Label::new(
                MENU_FONT_SIZE,
                gfx_window.fonts.roboto_font_id,
                TEXT_COLOR,
                String::from("Start typing test"),
                gfx_window,
            ),
            start_typing_test: false,
            results_list_label: Label::new(
                MENU_FONT_SIZE,
                gfx_window.fonts.roboto_font_id,
                TEXT_COLOR,
                String::from("Results"),
                gfx_window,
            ),
            show_results_list: false,
        }
    }

    fn update_font_metrics(&mut self, gfx_window: &mut GfxWindow) {
        ElementLayout::center_horizontally(
            gfx_window.window_dim(),
            self.typing_test_label.rect.bounds,
            &mut self.typing_test_label.rect.position,
        );

        ElementLayout::center_horizontally(
            gfx_window.window_dim(),
            self.results_list_label.rect.bounds,
            &mut self.results_list_label.rect.position,
        );

        {
            let mut v_centered = ElementLayout::vertical(gfx_window.window_dim());
            let typing_test_elem = v_centered.add_bounds(self.typing_test_label.rect.bounds);
            let result_list_elem = v_centered.add_bounds(self.results_list_label.rect.bounds);
            v_centered.calc_positions();
            self.typing_test_label.rect.position.y =
                v_centered.element_position(typing_test_elem).y;
            self.results_list_label.rect.position.y =
                v_centered.element_position(result_list_elem).y;
        }
    }
}

impl Screen for Menu {
    fn maybe_change_to_screen(
        &self,
        gfx_window: &mut GfxWindow,
        config: &Config,
    ) -> Option<Box<dyn Screen>> {
        if self.start_typing_test {
            Some(Box::new(screens::TestScreen::new(gfx_window, config)))
        } else if self.show_results_list {
            Some(Box::new(screens::ResultsListScreen::new(gfx_window)))
        } else {
            None
        }
    }

    fn process_event(&mut self, _event: &Event, _gfx_window: &mut GfxWindow) -> bool {
        let mut update_and_render = false;
        // if let Event::WindowEvent {

        // }
        update_and_render
    }

    fn mouse_click(&mut self, position: Vector2<f32>) {
        if self.typing_test_label.rect.contains_point(position) {
            self.start_typing_test = true;
        } else if self.results_list_label.rect.contains_point(position) {
            self.show_results_list = true;
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

        gfx_window.queue_label(&self.typing_test_label);
        gfx_window.queue_label(&self.results_list_label);

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
