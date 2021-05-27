use nannou::noise::*;
use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    noise: Perlin,
}

fn model(app: &App) -> Model {
    let _window = app.new_window().size(800, 800).view(view).build().unwrap();

    let mut noise = Perlin::new();
    noise = noise.set_seed(1);
    Model { noise }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, _model: &Model, frame: Frame) {
    // Prepare to draw.
    let draw = app.draw();

    let pos = 

    // Draw a blue ellipse at the x/y coordinates 0.0, 0.0
    draw.ellipse().color(STEELBLUE).x_y(0.0, 0.0);

    draw.to_frame(app, &frame).unwrap();
}