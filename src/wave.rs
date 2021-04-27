use nannou::noise::NoiseFn;
use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

struct Model {
    noise: nannou::noise::Perlin,
    points: Vec<Point2>,
    ynoise: f64,
    xnoise: f64,
}

fn model(app: &App) -> Model {
    let noise = nannou::noise::Perlin::new();
    let ynoise = 0.0;
    let xnoise = 0.0;
    let win = app.window_rect().wh();
    let width = win.x as f32;

    let mut points = Vec::new();
    for i in 1..8 {
        points.push(pt2(width / 16.0 * i as f32, 0.0));
    }
    Model {
        noise: noise,
        points: points,
        ynoise: ynoise,
        xnoise: xnoise,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let t = (app.elapsed_frames() as f64) * 0.008;
    model.ynoise = model.noise.get([0.0, t]);
    model.xnoise = model.noise.get([t, 0.0]);
    let mut noise_points = Vec::new();
    for i in &model.points {
        let n = model.noise.get([i.x as f64, t as f64]) * 150.0;
        noise_points.push(pt2(i.x, n as f32));
    }
    model.points = noise_points;
}

fn view(app: &App, model: &Model, frame: Frame) {
    // Prepare to draw.
    let draw = app.draw();
    
    let points = (0..7).map(|i| {
        //let x = i as f32 - 25.0; //subtract 25 to center the sine wave
        let point = model.points[i]; //scale sine wave by 20.0
        (point, STEELBLUE)
    });
    draw.polyline().weight(3.0).points_colored(points);

    draw.background().color(PLUM);

    draw.to_frame(app, &frame).unwrap();
}
