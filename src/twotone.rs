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

    //let half_width = model.width / 2.0;
    //let half_height = model.height / 2.0;

    draw.ellipse()
        .color(model.color2)
        .stroke(model.color1)
        .stroke_weight(3.0)
        .w_h(50.0, 50.0);

    let radius = 150.0;
    let points = (0..=360).map(|i| {
        let radian = deg_to_rad(i as f32);
        let x = radian.sin() * radius;
        let y = radian.cos() * radius;
        (pt2(x, y), model.color1)
    });

    draw.polyline().weight(3.0).points_colored(points);

    let points = (0..=360).map(|i| {
        let radian = deg_to_rad(i as f32);
        let offset = random_range(10.0, 20.0);
        let x = radian.sin() * (radius + offset);
        let y = radian.cos() * (radius + offset);
        (pt2(x, y), model.color1)
    });

    draw.polyline().weight(3.0).points_colored(points);

    // for i in -400..400 {
    //     // if i % 4 == 0 {
    //     //     continue;
    //     // }
    //     for j in 2..400 {
    //         let rand_y = random_range(1.0, j as f32);

    //         draw.ellipse()
    //             .color(model.color1)
    //             .w_h(1.0, 1.0)
    //             .x_y(i as f32, -rand_y);
    //     }
    // }

    draw.background().color(model.color2);

    draw.to_frame(app, &frame).unwrap();
}
