use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    points: Vec<Point2>,
}

fn model(app: &App) -> Model {
    let _window = app.new_window().size(800, 800).view(view).build().unwrap();

    let mut points = Vec::new();

    // go through an entire circle (360 degrees) and pick points
    let mut swap = false;
    for i in 0..360 {
        // switch magnitude range every 15
        let magnitude = if i % 15 == 0 && !swap {
            swap = !swap;
            // println!("far {}", i);
            random_range(300, 350) as f32
        } else if i % 15 == 0 && swap {
            swap = !swap;
            // println!("close {}", i);
            random_range(150, 200) as f32
        } else {
            -1.0
        };

        // create the vector with angle and magnitude if magnitude is not 0
        if magnitude != -1.0 {
            let p = Vector2::from_angle(i as f32 * (PI / 180.0)).with_magnitude(magnitude);
            points.push(p);
        }
    }

    Model { points }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, model: &Model, frame: Frame) {
    // Prepare to draw.
    let draw = app.draw();

    // center point
    draw.ellipse().color(WHITE).w_h(5.0, 5.0).x_y(0.0, 0.0);

    // fill in flower
    for i in 0..model.points.len() - 3 {
        let (p1, p2, p3) = (model.points[i], model.points[i + 1], model.points[i + 2]);
        draw.tri().color(WHITE).points(p1, p2, p3);
    }

    // flower points
    // for p in model.points.iter() {
    //     draw.ellipse().color(STEELBLUE).w_h(5.0, 5.0).x_y(p.x, p.y);
    // }

    // background
    draw.background().color(BLACK);

    draw.to_frame(app, &frame).unwrap();
}
