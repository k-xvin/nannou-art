use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    points: [Point2; 100],
}

fn model(app: &App) -> Model {
    let _window = app.new_window().size(800, 800).view(view).build().unwrap();

    let mut points = [pt2(0.0, 0.0); 100];
    for i in 1..100 {
        let x = random_range(-400, 401) as f32;
        let y = random_range(-400, 401) as f32;
        points[i] = pt2(x, y);
    }
    Model { points }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    //let mut points = [pt2(0.0, 0.0); 100];
    for i in 1..100 {
        let x = random_range(-400, 401) as f32;
        let y = random_range(-400, 401) as f32;
        model.points[i] = pt2(x, y);
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    // Prepare to draw.
    let draw = app.draw();

    let color = rgba(
        STEELBLUE.into_components().0 as f32 / 255.0,
        STEELBLUE.into_components().1 as f32 / 255.0,
        STEELBLUE.into_components().2 as f32 / 255.0,
        0.3,
    );
    for i in 0..model.points.len() {
        draw.ellipse()
            .color(color)
            .w(3.0)
            .h(3.0)
            .x_y(model.points[i].x, model.points[i].y);
    }
    //draw.background().color(BLACK);

    draw.to_frame(app, &frame).unwrap();
}
