mod car;
mod lights;

pub use car::*;
pub use lights::*;

use macroquad::prelude::*;

// ---------- tiny art helpers ----------
fn draw_radial_grass() {
    // base grass
    let base = Color { r: 0.13, g: 0.54, b: 0.18, a: 1.0 };
    clear_background(base);

    // gentle radial brighten toward the center
    let center = vec2(screen_width() * 0.5, screen_height() * 0.5);
    let max_r = (screen_width().hypot(screen_height())) * 0.55;
    for i in 0..8 {
        let t = i as f32 / 8.0;
        let r = max_r * (1.0 - t * 0.9);
        let overlay = Color { r: 0.16, g: 0.60, b: 0.20, a: 0.06 };
        draw_circle(center.x, center.y, r, overlay);
    }
}

fn draw_tree(x: f32, y: f32, size: f32, variant: i32) {
    // two variants: round canopy & "pine" stacked triangles
    let trunk = Color { r: 0.45, g: 0.26, b: 0.07, a: 1.0 };
    let leaf  = Color { r: 0.08, g: 0.55, b: 0.17, a: 1.0 };
    let leaf_d= Color { r: 0.05, g: 0.46, b: 0.14, a: 1.0 };

    // trunk
    let tw = size * 0.20;
    let th = size * 0.38;
    draw_rectangle(x + size*0.5 - tw*0.5, y + size*0.62, tw, th, trunk);

    if variant % 2 == 0 {
        // round canopy cluster
        let cx = x + size * 0.5;
        let cy = y + size * 0.50;
        let r1 = size * 0.38;
        let r2 = size * 0.28;
        draw_circle(cx, cy, r1, leaf);
        draw_circle(cx - size*0.18, cy - size*0.08, r2, leaf_d);
        draw_circle(cx + size*0.18, cy - size*0.04, r2, leaf_d);
    } else {
        // "pine": three triangles
        let h = size * 0.55;
        let w = size * 0.62;
        let base_y = y + size * 0.58;
        for k in 0..3 {
            let s = 1.0 - k as f32 * 0.18;
            let hw = (w * s) * 0.5;
            let top = Vec2::new(x + size*0.5, base_y - h * (0.15 + 0.28 * k as f32));
            let left= Vec2::new(top.x - hw, top.y + h*0.35);
            let right=Vec2::new(top.x + hw, top.y + h*0.35);
            draw_triangle(top, left, right, if k==0 {leaf} else {leaf_d});
        }
    }
}

fn sprinkle_trees(cell: f32) {
    let w = screen_width() as i32;
    let h = screen_height() as i32;
    let s = cell as i32;

    for gy in (0..h).step_by(s as usize) {
        for gx in (0..w).step_by(s as usize) {
            // avoid roads area
            let x = gx as f32;
            let y = gy as f32;
            if (x > 300.0 && x < 500.0) || (y > 200.0 && y < 400.0) { continue; }

            // light scatter
            let salt = ((gx / s) * 73 + (gy / s) * 97) % 7;
            if salt == 0 || salt == 3 {
                let jitter = vec2(
                    macroquad::rand::gen_range(-cell*0.2, cell*0.2),
                    macroquad::rand::gen_range(-cell*0.2, cell*0.2)
                );
                draw_tree(x + jitter.x, y + jitter.y, cell * macroquad::rand::gen_range(0.8, 1.15), salt);
            }
        }
    }
}

fn dashed_line_y(x: f32, y0: f32, y1: f32, dash: f32, gap: f32, thick: f32, col: Color) {
    let mut y = y0;
    while y < y1 {
        let y2 = (y + dash).min(y1);
        draw_line(x, y, x, y2, thick, col);
        y += dash + gap;
    }
}
fn dashed_line_x(y: f32, x0: f32, x1: f32, dash: f32, gap: f32, thick: f32, col: Color) {
    let mut x = x0;
    while x < x1 {
        let x2 = (x + dash).min(x1);
        draw_line(x, y, x2, y, thick, col);
        x += dash + gap;
    }
}
fn crosswalk_h(y: f32, x0: f32, x1: f32) {
    let stripe_w = 8.0;
    let stripe_h = 14.0;
    let step = 14.0;
    let mut x = x0;
    while x <= x1 - stripe_w {
        draw_rectangle(x, y, stripe_w, stripe_h, WHITE);
        x += step;
    }
}
fn crosswalk_v(x: f32, y0: f32, y1: f32) {
    let stripe_w = 14.0;
    let stripe_h = 8.0;
    let step = 14.0;
    let mut y = y0;
    while y <= y1 - stripe_h {
        draw_rectangle(x, y, stripe_w, stripe_h, WHITE);
        y += step;
    }
}

// glow for traffic lights
fn light_glow(x: f32, y: f32, color: Color) {
    let mut c = color; c.a = 0.12;
    for k in 1..=4 {
        draw_circle(x, y, 18.0 * k as f32, c);
    }
}

// car shadow (soft circle under car)
fn car_shadow(x: f32, y: f32, w: f32, h: f32) {
    let mut c = BLACK; c.a = 0.18;
    draw_circle(x + w*0.5 + 3.0, y + h*0.7, (w.max(h))*0.35, c);
}

// lane arrows (simple triangles)
fn lane_arrow_up(cx: f32, y: f32, col: Color) {
    let hw = 8.0; let h = 16.0;
    let top = Vec2::new(cx, y);
    let l = Vec2::new(cx - hw, y + h);
    let r = Vec2::new(cx + hw, y + h);
    draw_triangle(top, l, r, col);
}
fn lane_arrow_right(x: f32, cy: f32, col: Color) {
    let hw = 16.0; let h = 8.0;
    let tip = Vec2::new(x + hw, cy);
    let t  = Vec2::new(x, cy - h);
    let b  = Vec2::new(x, cy + h);
    draw_triangle(tip, t, b, col);
}

// ---------- sprites ----------
struct CarSprites { blue: Texture2D, yellow: Texture2D, purple: Texture2D }
async fn load_sprites() -> CarSprites {
    let blue   = load_texture("assets/car_blue.png").await.expect("assets/car_blue.png");
    let yellow = load_texture("assets/car_yellow.png").await.expect("assets/car_yellow.png");
    let purple = load_texture("assets/car_purple.png").await.expect("assets/car_purple.png");
    blue.set_filter(FilterMode::Nearest);
    yellow.set_filter(FilterMode::Nearest);
    purple.set_filter(FilterMode::Nearest);
    CarSprites { blue, yellow, purple }
}
fn angle_for(dir: Direction) -> f32 {
    match dir {
        Direction::Down  => 0.0,
        Direction::Right => std::f32::consts::FRAC_PI_2,
        Direction::Top   => std::f32::consts::PI,
        Direction::Left  => -std::f32::consts::FRAC_PI_2,
    }
}
fn sprite_for<'a>(s: &'a CarSprites, c: &Car) -> &'a Texture2D {
    if c.color == BLUE { &s.blue } else if c.color == YELLOW { &s.yellow } else { &s.purple }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Road Intersection — art mode".to_owned(),
        window_width: 800,
        window_height: 600,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let sprites = load_sprites().await;

    let mut cars_vec: Vec<Car> = Vec::new();
    let mut lights = TrafficLights {
        lights_top: false, lights_down: false, lights_left: false, lights_right: false,
        time: std::time::Instant::now(), current_direction: Direction::Right, state: false,
    };

    // geometry
    let center_x = 400.0; let center_y = 300.0;
    let road_w = 100.0; let h_road_y = 250.0; let v_road_x = 350.0;
    let asphalt  = Color { r: 0.11, g: 0.11, b: 0.12, a: 1.0 };
    let curb     = Color { r: 0.84, g: 0.84, b: 0.84, a: 1.0 };
    let yellow   = Color { r: 0.98, g: 0.83, b: 0.15, a: 1.0 };

    // light circles
    let r = 15.0;
    let top_center    = (318.0 + r, 218.0 + r);
    let right_center  = (318.0 + r, 352.0 + r);
    let down_center   = (452.0 + r, 352.0 + r);
    let left_center   = (452.0 + r, 218.0 + r);

    loop {
        // Grass & trees
        draw_radial_grass();
        sprinkle_trees(44.0);

        // roads
        draw_rectangle(0.0, h_road_y, 800.0, road_w, asphalt);
        draw_rectangle(v_road_x, 0.0,  road_w, 600.0, asphalt);

        // sidewalks/curbs
        let curb_t = 5.0;
        let sidewalk = Color { r: 0.93, g: 0.93, b: 0.93, a: 1.0 };
        // outer sidewalk slabs
        draw_rectangle(0.0, h_road_y - 18.0, 800.0, 18.0, sidewalk);
        draw_rectangle(0.0, h_road_y + road_w, 800.0, 18.0, sidewalk);
        draw_rectangle(v_road_x - 18.0, 0.0, 18.0, 600.0, sidewalk);
        draw_rectangle(v_road_x + road_w, 0.0, 18.0, 600.0, sidewalk);
        // curb lines
        draw_rectangle(0.0, h_road_y - curb_t, 800.0, curb_t, curb);
        draw_rectangle(0.0, h_road_y + road_w, 800.0, curb_t, curb);
        draw_rectangle(v_road_x - curb_t, 0.0, curb_t, 600.0, curb);
        draw_rectangle(v_road_x + road_w, 0.0, curb_t, 600.0, curb);

        // darker intersection pad + subtle tile lines
        let pad = Color { r: 0.08, g: 0.08, b: 0.09, a: 1.0 };
        draw_rectangle(v_road_x, h_road_y, road_w, road_w, pad);
        for i in 1..4 {
            let t = i as f32 * (road_w/4.0);
            let line = Color { r: 1.0, g: 1.0, b: 1.0, a: 0.04 };
            draw_line(v_road_x, h_road_y + t, v_road_x + road_w, h_road_y + t, 1.0, line);
            draw_line(v_road_x + t, h_road_y, v_road_x + t, h_road_y + road_w, 1.0, line);
        }

        // zebra crosswalks
        crosswalk_h(h_road_y - 14.0, v_road_x, v_road_x + road_w);
        crosswalk_h(h_road_y + road_w, v_road_x, v_road_x + road_w);
        crosswalk_v(v_road_x - 14.0, h_road_y, h_road_y + road_w);
        crosswalk_v(v_road_x + road_w, h_road_y, h_road_y + road_w);

        // dashed centerlines
        let dash = 14.0; let gap  = 10.0; let thick = 3.0;
        dashed_line_y(center_x, 0.0, h_road_y - 12.0, dash, gap, thick, yellow);
        dashed_line_y(center_x, h_road_y + road_w + 12.0, 600.0, dash, gap, thick, yellow);
        dashed_line_x(center_y, 0.0, v_road_x - 12.0, dash, gap, thick, yellow);
        dashed_line_x(center_y, v_road_x + road_w + 12.0, 800.0, dash, gap, thick, yellow);

        // lane arrows near entries
        lane_arrow_up(center_x + 40.0, h_road_y + road_w - 28.0, Color { r:1.0,g:1.0,b:1.0,a:0.65});
        lane_arrow_up(center_x - 90.0, h_road_y + 12.0,          Color { r:1.0,g:1.0,b:1.0,a:0.65});
        lane_arrow_right(v_road_x + 12.0, center_y - 90.0,       Color { r:1.0,g:1.0,b:1.0,a:0.65});
        lane_arrow_right(v_road_x + road_w - 28.0, center_y + 40.0, Color { r:1.0,g:1.0,b:1.0,a:0.65});

        // update & draw lights (+glow)
        traffic_lights_sys(&mut lights);
        let (g, rcol) = (GREEN, RED);
        let lights_arr = [
            (top_center,   lights.lights_top),
            (right_center, lights.lights_right),
            (down_center,  lights.lights_down),
            (left_center,  lights.lights_left),
        ];
        for ((x,y), on) in lights_arr {
            let c = if on { g } else { rcol };
            light_glow(x, y, c);
            draw_circle(x, y, r, c);
        }

        // header
        draw_text("Arrows, R to spawn | Esc quit", 12.0, 26.0, 24.0, WHITE);

        // cars update + draw (with soft shadows)
        let copy_cars = cars_vec.clone();
        for car in &mut cars_vec {
            traffic_lights(car, &lights);
            if car.moving && !car.next_car(&copy_cars) { car.move_car(); }
            if !car.turned { car.redirect(); }

            // shadow
            car_shadow(car.x as f32, car.y as f32, 30.0, 40.0);

            // sprite
            let tex = sprite_for(&sprites, car);
            let x = car.x as f32; let y = car.y as f32;
            draw_texture_ex(
                tex, x, y, WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(30.0, 40.0)),
                    rotation: angle_for(car.dir),
                    pivot: Some(vec2(x + 15.0, y + 20.0)),
                    ..Default::default()
                },
            );
        }

        // bounds
        cars_vec.retain(|c| c.x >= -30 && c.x <= 830 && c.y >= -40 && c.y <= 640);

        // input (Left/Right swapped, as you had)
        if is_key_pressed(KeyCode::Up)    { key_up(&mut cars_vec); }
        if is_key_pressed(KeyCode::Down)  { key_down(&mut cars_vec); }
        if is_key_pressed(KeyCode::Left)  { key_right(&mut cars_vec); }
        if is_key_pressed(KeyCode::Right) { key_left(&mut cars_vec); }
        if is_key_pressed(KeyCode::R)     { key_r(&mut cars_vec); }
        if is_key_pressed(KeyCode::Escape) { break; }

        next_frame().await;
    }
}
