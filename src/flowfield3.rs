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
    background_color: Rgba,
    flowfield: Vec<Vector2>,
    particles: Vec<Particle>,
}

struct Particle {
    position: Vector2,
    velocity: Vector2,
    acceleration: Vector2,
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

    let colors: Vec<Rgba> = colors_u8
        .to_vec()
        .into_iter()
        .map(|c| {
            let (r, g, b) = c.into_components();
            rgba(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0, 0.8)
        })
        .collect();

    let (r, g, b) = rgb_u32(0xf7f2df).into_components();
    let background_color = rgba(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0, 1.0);

    // make an initial flowfield
    let mut flowfield = Vec::new();
    for i in 0..cols {
        for j in 0..rows {
            // let _start = pt2(
            //     (i * scale) as f32 - width / 2.0,
            //     (j * scale) as f32 - height / 2.0,
            // )

            let x = i as f64 * 0.08;
            let y = j as f64 * 0.08;
            let noisevalue = noise.get([x, y, 0.0]) as f32;
            let angle = noisevalue * 2.0 * PI;
            let angle_vector = Vector2::from_angle(angle).with_magnitude(scale as f32);
            flowfield.push(angle_vector);
            //let end = pt2(angle_vector.x + start.x, angle_vector.y + start.y);
        }
    }

    // put starting position of a bunch of particles in random spots
    let mut particles = Vec::new();
    for _i in 1..1000 {
        let x = random_range(-400, 400) as f32;
        let y = random_range(-400, 400) as f32;
        particles.push(Particle {
            position: pt2(x, y),
            velocity: pt2(0.0, 0.0),
            acceleration: pt2(0.0, 0.0),
        });
    }

    Model {
        noise,
        scale,
        cols,
        rows,
        width,
        height,
        colors,
        background_color,
        flowfield,
        particles,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    // update the position of each particle
    let mut updated_particles: Vec<Particle> = Vec::new();
    for i in 0..model.particles.len() {
        // current particle
        let p = &model.particles[i];
        let p_pos = p.position;
        let p_vel = p.velocity;
        let p_acc = p.acceleration;

        // find flowfield vector current particle is on
        // we have to offset negative coords
        let col: i32 = ((p_pos.x + (model.width / 2.0)) / model.width) as i32 * model.cols;
        let row: i32 = ((p_pos.y + (model.height / 2.0)) / model.height) as i32 * model.cols;
        let flow_vector = model.flowfield[(col * model.cols + row) as usize];
        //let flow_vector = model.flowfield[0 as usize];
        //println!("flow x and y are {} and {}", flow_vector.x, flow_vector.y);

        // bounds
        let max_height = model.height / 2.0;
        let min_height = model.height / -2.0;
        let max_width = model.width / 2.0;
        let min_width = model.width / -2.0;

        // PLAN
        // calculate new values + boundary checks
        // update position with velocity vector
        // update velocity with accel from flowfield
        // update accel with the flowfield

        let np_pos_x = {
            if p_pos.x > max_width {
                // right bound, reset to left
                model.width / -2.0
            } else if p_pos.x < min_width {
                // left bound, reset to right
                model.width / 2.0
            } else {
                // update position with velocity vector
                p_pos.x + p_vel.x
            }
        };
        let np_pos_y = {
            if p_pos.y > max_height {
                // top bound, reset to bottom
                model.height / -2.0
            } else if p_pos.y < min_height {
                // bottom bound, reset to top
                model.height / 2.0
            } else {
                // update position with velocity vector
                p_pos.y + p_vel.y
            }
        };

        let np_vel_x = {
            // cap velocity magnitude at 5
            if abs(p_vel.x + p_acc.x) > 5.0 {
                p_vel.x
            } else {
                p_vel.x + p_acc.x
            }
        };
        let np_vel_y = {
            // cap velocity magnitude at 5
            if abs(p_vel.y + p_acc.y) > 5.0 {
                p_vel.y
            } else {
                p_vel.y + p_acc.y
            }
        };
        //println!("vel x and y are {} and {}", np_vel_x, np_vel_y);

        let np_acc_x = flow_vector.x * 0.1;
        let np_acc_y = flow_vector.y * 0.1;

        // new particle
        let np = Particle {
            position: pt2(np_pos_x, np_pos_y),

            velocity: pt2(np_vel_x, np_vel_y),
            //velocity: pt2(np_acc_x, np_acc_y),

            acceleration: pt2(np_acc_x, np_acc_y),
        };
        // add to new vec
        updated_particles.push(np);
    }
    model.particles = updated_particles;
}

fn view(app: &App, model: &Model, frame: Frame) {
    // Prepare to draw.
    let draw = app.draw();

    //let t = (app.elapsed_frames() as f64) * 0.01;

    // draw the flowfield
    for i in 0..model.cols {
        for j in 0..model.rows {
            let start = pt2(
                (i * model.scale) as f32 - model.width / 2.0,
                (j * model.scale) as f32 - model.height / 2.0,
            );

            //let color = model.colors[((i * j) % 4) as usize];

            let angle_vector = model.flowfield[(i * model.cols + j) as usize];
            let end = pt2(angle_vector.x + start.x, angle_vector.y + start.y);

            draw.line()
                .color(STEELBLUE)
                .weight(2.0)
                .start(start)
                .end(end);

            draw.ellipse()
                .color(BLACK)
                .w_h(3.0, 3.0)
                .x_y(start.x, start.y);
        }
    }

    // draw the particles
    for i in 0..model.particles.len() {
        // draw.line()
        //     .color(STEELBLUE)
        //     .weight(2.0)
        //     .start(pt2(0.0, 0.0))
        //     .end(model.particles[i]);
        draw.ellipse()
            .color(RED)
            .w_h(5.0, 5.0)
            .x_y(model.particles[i].position.x, model.particles[i].position.y);
    }

    // draw bg ONCE
    //if app.time <= 0.01 {
    draw.background().color(model.background_color);
    //}

    draw.to_frame(app, &frame).unwrap();
}
