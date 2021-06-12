use nannou::color::*;
use nannou::prelude::*;
//use nannou::rand::*;

fn main() {
    nannou::app(model).update(update).run();
    //nannou::app(model).run();
}

struct Model {
    color1: Rgb8,
    color2: Rgb8,
    width: f32,
    height: f32,
}

fn model(app: &App) -> Model {
    let _window = app.new_window().size(800, 800).view(view).build().unwrap();

    // palette from https://www.reddit.com/r/generative/comments/ns6hmj/summit/
    let color1 = rgb_u32(0xF2E6CD); // tan
    let color2 = rgb_u32(0x191815); // off-black

    let width = app.main_window().rect().w();
    let height = app.main_window().rect().h();

    Model {
        color1,
        color2,
        width,
        height,
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    let half_width = model.width / 2.0;
    let half_height = model.height / 2.0;

    let scale = 20;
    for i in 0..(model.width as i32 / scale) {
        let y = (i as f32 * scale as f32) - half_height;

        draw.line()
            .color(model.color1)
            .stroke_weight(3.0)
            .start(pt2(-half_width, y))
            .end(pt2(half_height, y))
            .z_degrees(30.0);

        draw.line()
            .color(model.color1)
            .stroke_weight(3.0)
            .start(pt2(-half_width, y))
            .end(pt2(half_height, y))
            .z_degrees(-30.0);

        draw.line()
            .color(model.color1)
            .stroke_weight(3.0)
            .start(pt2(-half_width, y))
            .end(pt2(half_height, y))
            .z_degrees(90.0);
    }

    draw.ellipse()
        .color(rgba(0.0,0.0,0.0,0.0))
        .stroke(model.color2)
        .stroke_weight(250.0)
        .w_h(900.0, 900.0);

    draw.background().color(model.color2);

    draw.to_frame(app, &frame).unwrap();
}
