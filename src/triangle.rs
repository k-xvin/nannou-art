use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    tri_vectors: (Vector2, Vector2, Vector2),
    magnitude: f32,
    tri_points: (Point2, Point2, Point2),
    period: f32,
}

fn model(app: &App) -> Model {
    let _window = app.new_window().size(800, 800).view(view).build().unwrap();

    let tri_vectors = (
        Vector2::from_angle(0.0),
        Vector2::from_angle(2.0 * PI / 3.0),
        Vector2::from_angle(4.0 * PI / 3.0),
    );
    let tri_points = (pt2(0.0, 0.0), pt2(0.0, 0.0), pt2(0.0, 0.0));
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

    // Update vector direction
    model.tri_vectors.0 = Vector2::from_angle(0.0 + rad_to_add);
    model.tri_vectors.1 = Vector2::from_angle((2.0 * PI / 3.0) + rad_to_add);
    model.tri_vectors.2 = Vector2::from_angle((4.0 * PI / 3.0) + rad_to_add);

    // Update point
    model.tri_points.0 = model.tri_vectors.0.with_magnitude(model.magnitude);
    model.tri_points.1 = model.tri_vectors.1.with_magnitude(model.magnitude);
    model.tri_points.2 = model.tri_vectors.2.with_magnitude(model.magnitude);
}

fn view(app: &App, model: &Model, frame: Frame) {
    // Prepare to draw.
    let draw = app.draw();

    // circle around it
    draw.ellipse()
        .color(BLACK)
        .stroke(WHITE)
        .stroke_weight(5.0)
        .radius(model.magnitude);

    // draw the triangle
    draw.tri()
        .color(WHITE)
        .points(model.tri_points.0, model.tri_points.1, model.tri_points.2);

    // circle in middle
    draw.ellipse().color(WHITE).w(3.0).h(3.0);

    // clear background
    draw.background().color(BLACK);

    draw.to_frame(app, &frame).unwrap();
}
