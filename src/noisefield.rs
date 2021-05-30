use nannou::color::*;
use nannou::noise::*;
use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    noise: Perlin,
    scale: i32, // the size of a flow vector square
    cols: i32,
    rows: i32,
    width: f32,
    height: f32,
    colors: Vec<Rgba>,
    background: Rgba, // flowfield: Vec<Vector2>,
                      // noisefield: Vec<f32>
}

fn model(app: &App) -> Model {
    let _window = app.new_window().size(800, 800).view(view).build().unwrap();

    let mut noise = Perlin::new();
    noise = noise.set_seed(random_f32() as u32);

    let scale = 15; // smaller = worse performance

    let width = app.main_window().rect().w();
    let height = app.main_window().rect().h();

    let cols = width as i32 / scale;
    let rows = height as i32 / scale;

    //['#ec643b', '#56b7ab', '#f8cb57', '#1f1e43']
    //'#f7f2df'
    let _colors_hex = [0xec643b, 0x56b7ab, 0xf8cb57, 0x1f1e43];

    // see examples/offline/colors in nannou repo
    let _colors2: Vec<Rgba> = _colors_hex
        .to_vec()
        .into_iter()
        .map(|c| {
            let blue: u8 = (c & 0xFF) as u8;
            let green: u8 = ((c >> 8) & 0xFF) as u8;
            let red: u8 = ((c >> 16) & 0xFF) as u8;
            rgba(
                red as f32 / 255.0,
                green as f32 / 255.0,
                blue as f32 / 255.0,
                0.3,
            )
        })
        .collect();

    let colors_u8 = [
        rgb_u32(0xec643b),
        rgb_u32(0x56b7ab),
        rgb_u32(0xf8cb57),
        rgb_u32(0x1f1e43),
    ];

    let colors: Vec<Rgba> = colors_u8.to_vec().into_iter().map(|c| {
        let (r, g, b) = c.into_components();
        rgba(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0, 0.8)
    }).collect();


    let (r, g, b) = rgb_u32(0xf7f2df).into_components();
    let background = rgba(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0, 1.0);
    //background.alpha = 0.5;

    Model {
        noise,
        scale,
        cols,
        rows,
        width,
        height,
        colors,
        background
        // flowfield,
        // noisefield,
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    // let t = (app.elapsed_frames() as f64) * 0.01;

    // // model.position = pt2(
    // //     (model.noise.get([t, 0.0]) * 500.0) as f32,
    // //     (model.noise.get([t + 10.0, 0.0]) * 500.0) as f32,
    // // );

    // // clear flowfield vectors
    // model.flowfield.clear();
    // model.noisefield.clear();

    // // create new vectors from perlin map
    // for i in 0..model.cols {
    //     for j in 0..model.rows {
    //         let x = i as f64 * 0.08;
    //         let y = j as f64 * 0.08;
    //         let noisevalue = model.noise.get([x, y, t]) as f32;
    //         let angle = noisevalue * 2.0 * PI; // SCALE NOISE BY 2PI FOR FULL ROTATION
    //         model
    //             .flowfield
    //             .push(Vector2::from_angle(angle).with_magnitude(model.scale as f32));
    //         model.noisefield.push(noisevalue);
    //     }
    // }
}

fn view(app: &App, model: &Model, frame: Frame) {
    // Prepare to draw.
    let draw = app.draw();

    let t = (app.elapsed_frames() as f64) * 0.01;

    for i in 0..model.cols {
        for j in 0..model.rows {
            let start = pt2(
                (i * model.scale) as f32 - model.width / 2.0,
                (j * model.scale) as f32 - model.height / 2.0,
            );

            let color = model.colors[((i * j) % 4) as usize];

            // draw.ellipse()
            //     .color(WHITE)
            //     .w_h(3.0, 3.0)
            //     .x_y(start.x, start.y);
            let x = i as f64 * 0.08;
            let y = j as f64 * 0.08;
            let noisevalue = model.noise.get([x, y, t]) as f32;
            draw.ellipse()
                .color(color)
                .w_h(100.0 * noisevalue, 100.0 * noisevalue)
                .x_y(start.x, start.y);
        }
    }

    draw.background().color(model.background);

    draw.to_frame(app, &frame).unwrap();
}
