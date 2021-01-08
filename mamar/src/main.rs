mod support;
mod icon;
mod ui;

use conrod_glium::Renderer;

type ImageMap = conrod_core::image::Map<glium::Texture2d>;

fn main() {
    use std::{
        thread::Builder as Thread,
        sync::mpsc::channel
    };
    use winit::window::WindowBuilder;
    use glium::glutin::dpi::LogicalSize;

    // Build the window.
    let event_loop = glium::glutin::event_loop::EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Mamar")
        .with_inner_size(LogicalSize::new(ui::WIDTH, ui::HEIGHT))
        .with_min_inner_size(LogicalSize::new(ui::WIDTH, ui::HEIGHT))
        .with_window_icon(icon::get_icon());
    let context = glium::glutin::ContextBuilder::new()
        .with_vsync(true)
        .with_multisampling(4);
    let display = glium::Display::new(window, context, &event_loop).unwrap();

    let mut renderer = Renderer::new(&display).unwrap();
    let image_map = ImageMap::new();

    let (event_tx, event_rx) = channel(); // Events (main thread -> ui thread)
    let (render_tx, render_rx) = channel(); // Render primitives (ui thread -> main thread)
    let event_loop_proxy = event_loop.create_proxy(); // So we can interrupt the EL when ready to draw

    Thread::new()
        .name("ui".to_string())
        .spawn(move || ui::run(event_rx, render_tx, event_loop_proxy))
        .unwrap();

    let mut is_waken = false;
    let mut latest_primitives = None;
    support::run_loop(display, event_loop, move |request, display| {
        match request {
            support::Request::Event {
                event,
                should_update_ui,
                should_exit,
            } => {
                // Pass events on to the ui thread if relevant
                if let Some(event) = support::convert_event(&event, &display.gl_window().window()) {
                    event_tx.send(event).unwrap();
                }

                match event {
                    glium::glutin::event::Event::WindowEvent { event, .. } => match event {
                        // Break from the loop upon `Escape`.
                        glium::glutin::event::WindowEvent::CloseRequested
                        | glium::glutin::event::WindowEvent::KeyboardInput {
                            input:
                                glium::glutin::event::KeyboardInput {
                                    virtual_keycode:
                                        Some(glium::glutin::event::VirtualKeyCode::Escape),
                                    ..
                                },
                            ..
                        } => *should_exit = true,
                        // We must re-draw on `Resized`, as the event loops become blocked during resize on macOS.
                        glium::glutin::event::WindowEvent::Resized(..) => {
                            if let Some(primitives) = render_rx.try_iter().last() {
                                latest_primitives = Some(primitives);
                            }
                            if let Some(primitives) = &latest_primitives {
                                draw(&display, &mut renderer, &image_map, primitives);
                            }
                        }
                        _ => {}
                    },
                    glium::glutin::event::Event::UserEvent(()) => {
                        is_waken = true;
                        // HACK: This triggers the `SetUi` request so that we can request a redraw.
                        *should_update_ui = true;
                    }
                    _ => {}
                }
            }
            support::Request::SetUi { needs_redraw } => {
                *needs_redraw = is_waken;
                is_waken = false;
            }
            support::Request::Redraw => {
                // Draw the most recently received `conrod_core::render::Primitives` sent from the ui.
                if let Some(primitives) = render_rx.try_iter().last() {
                    latest_primitives = Some(primitives);
                }
                if let Some(primitives) = &latest_primitives {
                    draw(&display, &mut renderer, &image_map, primitives);
                }
            }
        }
    })
}

fn draw(
    display: &glium::Display,
    renderer: &mut Renderer,
    image_map: &ImageMap,
    primitives: &conrod_core::render::OwnedPrimitives,
) {
    use glium::Surface;

    renderer.fill(display, primitives.walk(), &image_map);

    let mut target = display.draw();

    target.clear_color(0.0, 0.0, 0.0, 1.0);
    renderer.draw(display, &mut target, &image_map).unwrap();

    target.finish().unwrap();
}
