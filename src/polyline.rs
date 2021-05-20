use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    sin_points: [Point2; 800],
    amplitude: f32,
}

fn model(app: &App) -> Model {
    let _window = app.new_window().size(800, 800).view(view).build().unwrap();

    let sin_points = [pt2(0.0, 0.0); 800];
    let amplitude = 100.0;
    Model {
        sin_points,
        amplitude,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let time = app.time;

    for i in 0..model.sin_points.len() {
        let x = i as f32 - model.sin_points.len() as f32 / 2.0;
        let y = (time + i as f32 * 10.0 as f32).sin() * model.amplitude;
        //let y = (time + (i as f32 / 10.0) ).sin() * model.amplitude;
        model.sin_points[i] = pt2(x, y);
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    // Prepare to draw.
    let draw = app.draw();

    // circle in middle
    draw.ellipse().color(WHITE).w(3.0).h(3.0);

    // draw.polyline()
    //     .weight(5.0)
    //     .points_colored(std::array::IntoIter::new(model.sin_points).map(|i| (i, BLUE)));

    draw.polyline()
        .weight(3.0)
        .points_colored(std::array::IntoIter::new(model.sin_points).map(|i| (i, STEELBLUE))); //https://stackoverflow.com/a/30467494

    // clear background
    draw.background().color(BLACK);

    draw.to_frame(app, &frame).unwrap();
}
