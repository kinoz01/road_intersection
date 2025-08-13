mod car;
mod lights;

pub use car::*;
pub use lights::*;

use macroquad::prelude::*;

#[macroquad::main("Road Intersection")]
async fn main() {
    let mut cars_vec: Vec<Car> = Vec::new();

    let mut lights = TrafficLights {
        lights_top: false,
        lights_down: false,
        lights_left: false,
        lights_right: false,
        time: std::time::Instant::now(),
        current_direction: Direction::Right,
        state: false,
    };

    let r = 15.0;
    let top_center    = (318.0 + r, 218.0 + r);
    let right_center  = (318.0 + r, 352.0 + r);
    let down_center   = (452.0 + r, 352.0 + r);
    let left_center   = (452.0 + r, 218.0 + r);

    loop {
        clear_background(WHITE);

        draw_rectangle(0.0, 250.0, 800.0, 100.0, GRAY);
        draw_rectangle(350.0, 0.0, 100.0, 600.0, GRAY);

        for i in (0..=600).step_by(15) {
            if i < 250 || i > 350 {
                draw_line(400.0, i as f32, 400.0, (i + 2) as f32, 2.0, LIGHTGRAY);
            } else {
                draw_line(400.0, 250.0, 400.0, 350.0, 2.0, LIGHTGRAY);
            }
        }
        for i in (0..=800).step_by(15) {
            if i < 350 || i > 450 {
                draw_line(i as f32, 300.0, (i + 2) as f32, 300.0, 2.0, LIGHTGRAY);
            } else {
                draw_line(350.0, 300.0, 450.0, 300.0, 2.0, LIGHTGRAY);
            }
        }

        // outer lane borders
        draw_line(350.0, 0.0, 350.0, 600.0, 2.0, WHITE);
        draw_line(450.0, 0.0, 450.0, 600.0, 2.0, WHITE);
        draw_line(0.0, 250.0, 800.0, 250.0, 2.0, WHITE);
        draw_line(0.0, 350.0, 800.0, 350.0, 2.0, WHITE);

        // update lights
        traffic_lights_sys(&mut lights);

        // draw lights as circles
        let green = GREEN;
        let red = RED;
        draw_circle(top_center.0,   top_center.1,   r, if lights.lights_top   { green } else { red });
        draw_circle(right_center.0, right_center.1, r, if lights.lights_right { green } else { red });
        draw_circle(down_center.0,  down_center.1,  r, if lights.lights_down  { green } else { red });
        draw_circle(left_center.0,  left_center.1,  r, if lights.lights_left  { green } else { red });

        // instructions
        draw_text("Arrows, R to spawn | Esc quit", 12.0, 24.0, 22.0, BLACK);

        // cars update + draw
        let copy_cars = cars_vec.clone();
        for car in &mut cars_vec {
            traffic_lights(car, &lights);
            if car.moving && !car.next_car(&copy_cars) {
                car.move_car();
            }
            if !car.turned {
                car.redirect();
            }
            draw_rectangle(car.x as f32, car.y as f32, 30.0, 30.0, car.color);
        }

        // keep cars within screen bounds
        cars_vec.retain(|car| car.y <= 630 && car.y >= -30 && car.x <= 830 && car.x >= -30);

        // input spawn
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
        if is_key_pressed(KeyCode::Escape) || is_quit_requested() {
            break;
        }

        next_frame().await;
    }
}
