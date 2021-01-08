mod app;

pub const WIDTH: u32 = 800;
pub const HEIGHT: u32 = 600;

pub fn run(
    event_rx: std::sync::mpsc::Receiver<conrod_core::event::Input>,
    render_tx: std::sync::mpsc::Sender<conrod_core::render::OwnedPrimitives>,
    event_loop: glium::glutin::event_loop::EventLoopProxy<()>,
) {
    let mut ui = conrod_core::UiBuilder::new([WIDTH as f64, HEIGHT as f64])
        .theme(app::theme())
        .build();

    ui.fonts.insert(rusttype::Font::from_bytes(include_bytes!("ui/pmdialog2.ttf")).unwrap());

    let mut app = app::App::new();
    let ids = app::Ids::new(ui.widget_id_generator());

    let mut needs_update = true;
    'conrod: loop {
        // Collect any pending events.
        let mut events = Vec::new();
        while let Ok(event) = event_rx.try_recv() {
            events.push(event);
        }

        // If there are no events pending, wait for some.
        if events.is_empty() && !needs_update {
            match event_rx.recv() {
                Ok(event) => events.push(event),
                Err(_) => break 'conrod,
            };
        }

        needs_update = false;

        // Input each event into the ui.
        for event in events {
            ui.handle_event(event);
            needs_update = true; // Some widgets require a second frame to finish drawing after clicks or hovers.
        }

        app.update(&mut ui.set_widgets(), &ids);

        // Render the ui to a list of primitives and send them to the main thread.
        if let Some(primitives) = ui.draw_if_changed() {
            needs_update = true;

            if render_tx.send(primitives.owned()).is_err() || event_loop.send_event(()).is_err() {
                // oh no
                break 'conrod;
            }
        }
    }
}
