use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    points: vec![],
    colored_points: std::iter::Map,
    color: Rgb8,
    radius: f32
}

fn model(app: &App) -> Model {
    let _window = app.new_window().size(800, 800).view(view).build().unwrap();

    let color = REBECCAPURPLE;
    let radius = 50.0;

    let colored_points = (0..=360).map(|i| {      // map over an array of integers from 0 to 360 to represent the degrees in a circle

        let radian = deg_to_rad(i as f32); // convert each degree to radians
        let x = radian.sin() * radius;     // get the sine of the radian to find the x-co-ordinate of
                                           // this point of the circle, and multiply it by the radius
        let y = radian.cos() * radius;     // do the same with cosine to find the y co-ordinate
        (pt2(x,y), color)              // construct and return a point object with a color
    });

    Model {
        colored_points,
        color,
        radius,
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

    // circle in middle
    draw.ellipse().color(WHITE).w(3.0).h(3.0);

    // clear background
    draw.background().color(BLACK);

    draw.to_frame(app, &frame).unwrap();
}