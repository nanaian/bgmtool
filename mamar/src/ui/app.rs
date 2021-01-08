use conrod_core::widget_ids;

widget_ids!(pub struct Ids {
    text,
});

pub struct App {
    // ...
}

impl App {
    pub fn new() -> Self {
        App {}
    }

    pub fn update(&mut self, ui: &mut conrod_core::UiCell, ids: &Ids) {
        use conrod_core::*;

        widget::Text::new("Hello World! (now with threading!)")
            .middle_of(ui.window)
            .color(color::WHITE)
            .font_size(32)
            .set(ids.text, ui);
    }
}

pub fn theme() -> conrod_core::Theme {
    use conrod_core::position::{Align, Direction, Padding, Position, Relative};

    conrod_core::Theme {
        name: "Mamar Theme".to_string(),
        padding: Padding::none(),
        x_position: Position::Relative(Relative::Align(Align::Start), None),
        y_position: Position::Relative(Relative::Direction(Direction::Backwards, 20.0), None),
        background_color: conrod_core::color::DARK_CHARCOAL,
        shape_color: conrod_core::color::LIGHT_CHARCOAL,
        border_color: conrod_core::color::BLACK,
        border_width: 0.0,
        label_color: conrod_core::color::WHITE,
        font_id: None,
        font_size_large: 26,
        font_size_medium: 18,
        font_size_small: 12,
        widget_styling: conrod_core::theme::StyleMap::default(),
        mouse_drag_threshold: 0.0,
        double_click_threshold: std::time::Duration::from_millis(500),
    }
}
