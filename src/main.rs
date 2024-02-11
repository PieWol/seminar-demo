use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

struct Model {
    points: Vec<Point2>,
    drawn_until: usize, // Tracks how many points have been drawn
}

fn model(app: &App) -> Model {
    // Generate points for the curve within the positive quadrant
    let points = generate_polynomial_curve_points(200, app.window_rect().pad(50.0));
    Model {
        points,
        drawn_until: 0,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    // Increment the drawn_until counter to animate the curve drawing
    let elapsed_frames = app.elapsed_frames();
    model.drawn_until = (elapsed_frames as f32 * 0.5).min(model.points.len() as f32) as usize;
    // Adjust speed here
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    if frame.nth() == 0 {
        draw.background().color(WHITE);
    }

    let win = app.window_rect();
    let axis_offset = 30.0;

    // Draw positive axes with labels only once
    if frame.nth() == 0 {
        draw_line(
            &draw,
            0.0,
            0.0,
            win.w() - axis_offset,
            0.0,
            srgba(0.5, 0.5, 0.5, 1.0),
        ); // X-axis
        draw_line(
            &draw,
            0.0,
            0.0,
            0.0,
            win.h() - axis_offset,
            srgba(0.5, 0.5, 0.5, 1.0),
        ); // Y-axis
        draw.text("X")
            .color(BLACK)
            .font_size(16)
            .xy(pt2(win.w() - axis_offset, 15.0)); // X label
        draw.text("Y")
            .color(BLACK)
            .font_size(16)
            .xy(pt2(15.0, win.h() - axis_offset)); // Y label
    }

    // Draw the portion of the polynomial curve based on current progress
    for i in 0..model.drawn_until.saturating_sub(1) {
        draw.line()
            .start(model.points[i])
            .end(model.points[i + 1])
            .color(RED)
            .stroke_weight(2.0);
    }

    draw.to_frame(app, &frame).unwrap();
}

fn generate_polynomial_curve_points(count: usize, rect: Rect) -> Vec<Point2> {
    let a = 0.01; // Quadratic coefficient
    let b = 0.0; // Linear coefficient
    let c = 0.0; // Constant term

    (0..count)
        .map(|i| {
            let x = map_range(i, 0, count - 1, rect.x.start, rect.x.end);
            let y = a * x.powi(2) + b * x + c; // Quadratic polynomial function
            pt2(x, y)
        })
        .collect()
}

fn draw_line(draw: &Draw, start_x: f32, start_y: f32, end_x: f32, end_y: f32, color: Srgba) {
    draw.line()
        .start(pt2(start_x, start_y))
        .end(pt2(end_x, end_y))
        .color(color)
        .stroke_weight(1.0);
}
