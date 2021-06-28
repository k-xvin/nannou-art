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

    for _i in 0..101 {
        let x = random_range(-350, 350) as f32;
        let y = random_range(-350, 350) as f32;
        points.push(pt2(x, y));
    }


    // generate the initial points

    Model { points }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, model: &Model, frame: Frame) {
    // Prepare to draw.
    let draw = app.draw();

    // middle dot
    draw.ellipse().color(WHITE).w_h(5.0, 5.0).x_y(0.0, 0.0);

    // current time (as elapsed frames)
    let t = app.elapsed_frames();

    // draw dots
    // drawn position of the dot is based on time
    // time progresses -> dots move
    for p in model.points.iter() {
        draw_point(&draw, *p, t as f32, STEELBLUE, 1.0);
    }

    //draw_point(&draw, pt2(50.0, 0.0), t as f32, STEELBLUE, 1.0);

    //draw_point(&draw, pt2(0.0, -80.0), t as f32, STEELBLUE, 1.5);

    // black bg
    draw.background().color(BLACK);

    draw.to_frame(app, &frame).unwrap();
}

// draws a point that has a trail
fn draw_point(draw: &Draw, start: Point2, t: f32, rgb8: Rgb8, speed_mult: f32) {
    let (r, g, b) = rgb8.into_components();

    // draw 5 points, each behind the other along a circle
    // vary the alpha as we draw points for the "trail"
    let trail_length = 20;
    for i in 0..(trail_length + 1) {
        let color = rgba(
            r as f32 / 255.0,
            g as f32 / 255.0,
            b as f32 / 255.0,
            1.0 - ((1.0 / trail_length as f32) * i as f32),
        );

        // unit vector that has the angle we want to add to the start point
        let angle_vector = Vector2::from_angle((t - i as f32) * 0.005 * (2.0 * PI) * speed_mult);

        // magnitude of the start point
        let magnitude = start.magnitude();



        // construct a new vector with the sum of angles and correct magnitude
        let final_angle = angle_vector.angle() + start.angle();
        let final_vector = Vector2::from_angle(final_angle).with_magnitude(magnitude);

        if i == 0 {
            draw.ellipse()
                .color(color)
                .w_h(5.0, 5.0)
                .x_y(final_vector.x, final_vector.y);
        }
        // trail dots are slightly smaller
        else {
            draw.ellipse()
                .color(color)
                .w_h(4.5, 4.5)
                .x_y(final_vector.x, final_vector.y);
        }
    }
}
