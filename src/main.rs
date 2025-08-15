mod car;
mod lights;

pub use car::*;
pub use lights::*;

use macroquad::prelude::*;

// ---------- tiny draw helpers ----------
fn grass_bg(tile: f32) {
    clear_background(Color { r: 0.2, g: 0.55, b: 0.18, a: 1.0 });
    let patch = Color { r: 0.1, g: 0.45, b: 0.14, a: 1.0 };
    let w = screen_width();
    let h = screen_height();
    let s = tile;
    for y in (0..h as i32).step_by(s as usize) {
        for x in (0..w as i32).step_by(s as usize) {
            if (x / (s as i32) + y / (s as i32)) % 3 == 0 {
                draw_rectangle(
                    (x as f32) + s * 0.05,
                    (y as f32) + s * 0.05,
                    s,
                    s,
                    patch
                );
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
    let stripe_w = 6.0;
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
    let stripe_h = 6.0;
    let step = 14.0;
    let mut y = y0;
    while y <= y1 - stripe_h {
        draw_rectangle(x, y, stripe_w, stripe_h, WHITE);
        y += step;
    }
}

// ---------- sprites ----------
struct CarSprites {
    blue: Texture2D,
    yellow: Texture2D,
    purple: Texture2D,
}

async fn load_sprites() -> CarSprites {
    let blue = load_texture("assets/car_blue.png").await.expect("assets/car_blue.png");
    let yellow = load_texture("assets/car_yellow.png").await.expect("assets/car_yellow.png");
    let purple = load_texture("assets/car_purple.png").await.expect("assets/car_purple.png");
    CarSprites { blue, yellow, purple }
}

// Base sprite faces UP/North
fn angle_for(dir: Direction) -> f32 {
    match dir {
        Direction::Down => 0.0,
        Direction::Right => std::f32::consts::FRAC_PI_2,
        Direction::Top => std::f32::consts::PI,
        Direction::Left => -std::f32::consts::FRAC_PI_2,
    }
}

// Return a reference
fn sprite_for<'a>(s: &'a CarSprites, c: &Car) -> &'a Texture2D {
    if c.color == BLUE { &s.blue } else if c.color == YELLOW { &s.yellow } else { &s.purple }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "🟢🔴 Road Intersection ➕".to_owned(),
        window_width: 800,
        window_height: 700,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let sprites = load_sprites().await;

    let mut cars_vec: Vec<Car> = Vec::new();

    let mut lights = TrafficLights {
        lights_top_left: false,
        lights_down_right: false,
        lights_top_right: false,
        lights_down_left: false,
        time: std::time::Instant::now(),
        current_direction: Direction::Right,
        state: false,
    };

    // ----- geometry -----------
    let center_x = 400.0;
    let center_y = 350.0;
    let road_w = 100.0;
    let h_road_y = 300.0;
    let v_road_x = 350.0;
    let asphalt = Color { r: 0.12, g: 0.12, b: 0.12, a: 1.0 };
    let curb = Color { r: 0.8, g: 0.8, b: 0.8, a: 1.0 };
    let yellow = Color { r: 1.0, g: 0.84, b: 0.0, a: 1.0 };

    // traffic light circles
    let r = 15.0;
    let top_left = (315.0 + r, 265.0 + r);
    let down_left = (315.0 + r, 405.0 + r);
    let down_right = (455.0 + r, 405.0 + r);
    let top_right = (455.0 + r, 265.0 + r);

    loop {
        // grass
        grass_bg(15.0);

        // roads
        draw_rectangle(0.0, h_road_y, 800.0, road_w, asphalt);
        draw_rectangle(v_road_x, 0.0, road_w, 700.0, asphalt);

        // curbs
        let curb_t = 4.0;
        draw_rectangle(0.0, h_road_y - curb_t, 800.0, curb_t, curb);
        draw_rectangle(0.0, h_road_y + road_w, 800.0, curb_t, curb);
        draw_rectangle(v_road_x - curb_t, 0.0, curb_t, 700.0, curb);
        draw_rectangle(v_road_x + road_w, 0.0, curb_t, 700.0, curb);
        // asphalt masks to hide curb inside the crosswalk spans
        draw_rectangle(v_road_x, h_road_y - curb_t, road_w, curb_t, asphalt);
        draw_rectangle(v_road_x, h_road_y + road_w, road_w, curb_t, asphalt);
        draw_rectangle(v_road_x - curb_t, h_road_y, curb_t, road_w, asphalt);
        draw_rectangle(v_road_x + road_w, h_road_y, curb_t, road_w, asphalt);

        // darker intersection pad + center dot
        let pad = Color { r: 0.08, g: 0.08, b: 0.08, a: 1.0 };
        draw_rectangle(v_road_x, h_road_y, road_w, road_w, pad);
        draw_circle(center_x, center_y, 10.0, Color { r: 0.06, g: 0.5, b: 0.06, a: 1.0 });
        draw_circle_lines(center_x, center_y, 9.5, 1.0, GRAY);

        // zebra crosswalks
        crosswalk_h(h_road_y - 14.0, v_road_x, v_road_x + road_w);
        crosswalk_h(h_road_y + road_w, v_road_x, v_road_x + road_w);
        crosswalk_v(v_road_x - 14.0, h_road_y, h_road_y + road_w);
        crosswalk_v(v_road_x + road_w, h_road_y, h_road_y + road_w);

        // dashed centerlines
        let dash = 14.0;
        let gap = 10.0;
        let thick = 3.0;
        dashed_line_y(center_x, 0.0, h_road_y - 10.0, dash, gap, thick, yellow);
        dashed_line_y(center_x, h_road_y + road_w + 10.0, 700.0, dash, gap, thick, yellow);
        dashed_line_x(center_y, 0.0, v_road_x - 10.0, dash, gap, thick, yellow);
        dashed_line_x(center_y, v_road_x + road_w + 10.0, 800.0, dash, gap, thick, yellow);

        // update lights
        traffic_lights_sys(&mut lights, &cars_vec);

        // lights as circles
        draw_circle(top_left.0, top_left.1, r, if lights.lights_top_left { GREEN } else { RED });
        draw_circle(down_left.0, down_left.1, r, if lights.lights_down_left { GREEN } else { RED });
        draw_circle(down_right.0, down_right.1, r, if lights.lights_down_right { GREEN } else { RED });
        draw_circle(top_right.0, top_right.1, r, if lights.lights_top_right { GREEN } else { RED });

        // instructions
        draw_text("Arrows, R to spawn | Esc quit", 12.0, 24.0, 24.0, WHITE);

        // cars update + draw (sprites)
        let copy_cars = cars_vec.clone();
        for car in &mut cars_vec {
            traffic_lights(car, &lights);
            if car.moving && !car.next_car(&copy_cars) {
                car.move_car();
            }
            if !car.turned {
                car.redirect();
            }

            // sprite draw (30×40 footprint, rotate around center)
            let tex = sprite_for(&sprites, car);
            let x = car.x as f32;
            let y = car.y as f32;
            draw_texture_ex(tex, x, y, WHITE, DrawTextureParams {
                dest_size: Some(vec2(30.0, 40.0)),
                rotation: angle_for(car.dir),
                pivot: Some(vec2(x + 15.0, y + 20.0)),
                ..Default::default()
            });
        }

        cars_vec.retain(|car| car.y <= 740 && car.y >= -40 && car.x <= 840 && car.x >= -40);

        // input
        if is_key_pressed(KeyCode::Up) {
            key_up(&mut cars_vec);
        }
        if is_key_pressed(KeyCode::Down) {
            key_down(&mut cars_vec);
        }
        if is_key_pressed(KeyCode::Left) {
            key_right(&mut cars_vec);
        }
        if is_key_pressed(KeyCode::Right) {
            key_left(&mut cars_vec);
        }
        if is_key_pressed(KeyCode::R) {
            key_r(&mut cars_vec);
        }
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        next_frame().await;
    }
}
