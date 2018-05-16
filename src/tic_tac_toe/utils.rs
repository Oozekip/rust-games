use nanovg::{Color, Frame, StrokeOptions};

pub fn draw_line(frame: &Frame, start: (f32, f32), end: (f32, f32), color: Color, width: f32) {
    frame.path(
        |path| {
            path.move_to(start);
            path.line_to(end);
            path.stroke(
                color,
                StrokeOptions {
                    width,
                    ..Default::default()
                },
            );
        },
        Default::default(),
    );
}
