use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    dt_frequency: f32, // discrete time (cycles/sample)
    sample_points: [Point2; 800],
    amplitude: f32,
    speed: f32,
}

fn model(app: &App) -> Model {
    let _window = app.new_window().size(800, 800).view(view).build().unwrap();

    let dt_frequency = 0.001; // between -0.5 and 0.5
    let sample_points = [pt2(0.0, 0.0); 800];
    let amplitude = 50.0;
    let speed = 4.0;

    Model {
        dt_frequency,
        sample_points,
        amplitude,
        speed,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let time = app.time;

    let mut x: f32;
    let mut y: f32;
    for i in 0..model.sample_points.len() {
        x = i as f32 - model.sample_points.len() as f32 / 2.0;
        y = model.amplitude
            * (2.0 * PI * model.dt_frequency * i as f32 + (time * model.speed)).sin();
        //let y = (time + (i as f32 / 10.0) ).sin() * model.amplitude;
        model.sample_points[i] = pt2(x, y);
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    // Prepare to draw.
    let draw = app.draw();

    // circle in middle
    //draw.ellipse().color(WHITE).w(3.0).h(3.0);

    for n in -5..5 {
        // plot sin wave
        draw.polyline().weight(7.0).points_colored(
            std::array::IntoIter::new(model.sample_points)
                .map(|i| (pt2(i.x, i.y - n as f32 * 60.0), STEELBLUE)),
        );
    }

    // plot sin wave
    // draw.polyline()
    //     .weight(3.0)
    //     .points_colored(std::array::IntoIter::new(model.sample_points).map(|i| (i, STEELBLUE)));

    // clear background
    draw.background().color(TAN);

    draw.to_frame(app, &frame).unwrap();
}
