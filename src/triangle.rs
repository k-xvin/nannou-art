use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    tri_vectors: [Vector2; 3],
    magnitude: f32,
    tri_points: [Point2; 3],
    period: f32,
}

fn model(app: &App) -> Model {
    let _window = app.new_window().size(800, 800).view(view).build().unwrap();

    let tri_vectors = [
        Vector2::from_angle(0.0),
        Vector2::from_angle(2.0 * PI / 3.0),
        Vector2::from_angle(4.0 * PI / 3.0),
    ];
    let tri_points = [pt2(0.0, 0.0), pt2(0.0, 0.0), pt2(0.0, 0.0)];
    Model {
        tri_vectors,
        tri_points,
        magnitude: 100.0,
        period: 5.0,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    // update vector directions
    let time = app.time;

    // we want a full rotation every x seconds
    // that means add 2PI radians over every x seconds
    let rad_to_add = -((2.0 * PI) / model.period) * (time % model.period);

    // Update vector direction and point
    for i in 0..model.tri_vectors.len() {
        model.tri_vectors[i] = Vector2::from_angle((2.0 * i as f32 * PI / 3.0) + rad_to_add);
        model.tri_points[i] = model.tri_vectors[i].with_magnitude(model.magnitude);
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    // Prepare to draw.
    let draw = app.draw();

    // central circle
    draw.ellipse()
        .color(BLACK)
        .stroke(REBECCAPURPLE)
        .stroke_weight(5.0)
        .radius(model.magnitude);

    // draw the triangle
    // draw.tri().color(REBECCAPURPLE).points(
    //     model.tri_points[0],
    //     model.tri_points[1],
    //     model.tri_points[2],
    // );

    // circle on points
    for i in 0..model.tri_vectors.len() {
        draw.ellipse()
            .color(rgba(0.0, 0.0, 0.0, 0.0))
            .stroke(REBECCAPURPLE)
            .stroke_weight(5.0)
            .x_y(model.tri_points[i].x, model.tri_points[i].y)
            .radius(model.magnitude);
    }

    // circle in middle
    draw.ellipse().color(WHITE).w(3.0).h(3.0);

    // clear background
    draw.background().color(BLACK);

    draw.to_frame(app, &frame).unwrap();
}
