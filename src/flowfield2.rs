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
    flowfield: Vec<Vector2>,
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

    //let flowfield = Vector2::from_angle(0.0);
    let flowfield = vec![Vector2::from_angle(0.0)];

    Model {
        noise,
        scale,
        cols,
        rows,
        width,
        height,
        flowfield,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let t = (app.elapsed_frames() as f64) * 0.01;

    // model.position = pt2(
    //     (model.noise.get([t, 0.0]) * 500.0) as f32,
    //     (model.noise.get([t + 10.0, 0.0]) * 500.0) as f32,
    // );

    // clear flowfield vectors
    model.flowfield.clear();

    // create new vectors from perlin map
    for i in 0..model.cols {
        for j in 0..model.rows {
            let x = i as f64 * 0.008;
            let y = j as f64 * 0.008;
            let angle = model.noise.get([x, y, t]) as f32 * 2.0 * PI; // SCALE NOISE BY 2PI FOR FULL ROTATION
            model
                .flowfield
                .push(Vector2::from_angle(angle).with_magnitude(model.scale as f32));
        }
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    // Prepare to draw.
    let draw = app.draw();

    //let mut row_ends: Vec<Vector2> = Vec::new();
    // draw flowfield
    for i in 0..model.cols {
        //row_ends.clear();
        let mut row_ends: Vec<Vector2> = Vec::new();

        for j in 0..model.rows {
            let start = pt2(
                (i * model.scale) as f32 - model.width / 2.0,
                (j * model.scale) as f32 - model.height / 2.0,
            );
            let flow_vector = model.flowfield[(i * j) as usize];
            let end = pt2(flow_vector.x + start.x, flow_vector.y + start.y);
            row_ends.push(end);

            // draw.ellipse()
            //     .color(WHITE)
            //     .w_h(3.0, 3.0)
            //     .x_y(start.x, start.y);

            // draw.ellipse()
            //     .color(STEELBLUE)
            //     .w_h(3.0, 3.0)
            //     .x_y(end.x, end.y);

            // draw.line()
            //     .color(STEELBLUE)
            //     .weight(2.0)
            //     .start(start)
            //     .end(end);
        }

        // let row_ends_mapped = row_ends
        //     .into_iter()
        //     .map(|i| (pt2(i.x, i.y as f32), STEELBLUE));

        // vertical lines
        draw.polyline().weight(2.0).points_colored(
            row_ends
            .iter()
            .map(|i| (pt2(i.x, i.y as f32), STEELBLUE))
        );

        // horizontal lines
        draw.polyline().weight(2.0).points_colored(
            row_ends
            .iter()
            .map(|i| (pt2(i.y, i.x as f32), STEELBLUE))
        );
    }

    draw.background().color(BLACK);

    draw.to_frame(app, &frame).unwrap();
}
