extern crate games;
extern crate gl;
extern crate glutin;
extern crate nanovg;

use games::tic_tac_toe::TicTacToe;
use games::Game;
use glutin::GlContext;

fn main() {
    let mut width = 600;
    let mut height = 600;

    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
        .with_title("0")
        .with_dimensions(width, height);
    let context = glutin::ContextBuilder::new()
        .with_vsync(true)
        .with_multisampling(4)
        .with_srgb(true);
    let gl_window = glutin::GlWindow::new(window, context, &events_loop).unwrap();

    let _ = unsafe { gl_window.make_current() };

    let context = nanovg::ContextBuilder::new()
        .stencil_strokes()
        .build()
        .unwrap();

    let mut ttt = TicTacToe::new();

    unsafe {
        gl_window.make_current().unwrap();
        gl::load_with(|symbol| gl_window.get_proc_address(symbol) as *const _);
        gl::ClearColor(0.0, 0.0, 0.0, 1.0);
    }

    let mut running = true;

    while running {
        events_loop.poll_events(|event| {
            if let glutin::Event::WindowEvent { event, .. } = event {
                match event {
                    glutin::WindowEvent::CloseRequested => running = false,
                    glutin::WindowEvent::Resized(w, h) => {
                        gl_window.resize(w, h);
                        width = w;
                        height = h;
                    }
                    _ => (),
                }

                ttt.handle_event(&event)
            }
        });

        // Set viewport and clear
        unsafe {
            gl::Viewport(0, 0, width as i32, height as i32);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT | gl::STENCIL_BUFFER_BIT);
        }

        ttt.draw(&context, &gl_window);

        gl_window.swap_buffers().unwrap();
    }
}
