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
    colors: [Rgb8; 4],
    background: Rgb8, // flowfield: Vec<Vector2>,
                      // noisefield: Vec<f32>
}

fn model(app: &App) -> Model {
    let _window = app.new_window().size(800, 800).view(view).build().unwrap();

    let mut noise = Perlin::new();
    noise = noise.set_seed(random_f32() as u32);

    let scale = 10; // smaller = worse performance

    let width = app.main_window().rect().w();
    let height = app.main_window().rect().h();

    let cols = width as i32 / scale;
    let rows = height as i32 / scale;

    //['#ec643b', '#56b7ab', '#f8cb57', '#1f1e43']
    //'#f7f2df'
    let colors = [
        rgb_u32(0xec643b),
        rgb_u32(0x56b7ab),
        rgb_u32(0xf8cb57),
        rgb_u32(0x1f1e43),
    ];
    // .into_iter()
    // .map(|i| {
    //     let (r, g, b) = i.into_components(); 
    //     rgba(r, g, b, 0.3 as u8)
    // }).collect();
    let background = rgb_u32(0xf7f2df);

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

            // draw.ellipse()
            //     .color(WHITE)
            //     .w_h(3.0, 3.0)
            //     .x_y(start.x, start.y);
            let x = i as f64 * 0.08;
            let y = j as f64 * 0.08;
            let noisevalue = model.noise.get([x, y, t]) as f32;
            draw.ellipse()
                .color(model.colors[((i * j) % 4) as usize])
                .w_h(30.0 * noisevalue, 30.0 * noisevalue)
                .x_y(start.x, start.y);
        }
    }

    draw.background().color(model.background);

    draw.to_frame(app, &frame).unwrap();
}
