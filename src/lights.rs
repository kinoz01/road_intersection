use crate::{ Car, Direction };
pub use std::time::{ Duration, Instant };

pub struct TrafficLights {
    pub lights_top: bool,
    pub lights_down: bool,
    pub lights_left: bool,
    pub lights_right: bool,
    pub current_direction: Direction,
    pub state: bool,
    pub time: Instant,
}

pub fn traffic_lights(car: &mut Car, lights: &TrafficLights) {
    if
        (!lights.lights_down && car.dir == Direction::Down && car.y == 420) ||
        (!lights.lights_top  && car.dir == Direction::Top  && car.y == 240) ||
        (!lights.lights_right && car.dir == Direction::Right && car.x == 300) ||
        (!lights.lights_left  && car.dir == Direction::Left  && car.x == 470)
    {
        car.moving = false;
    } else {
        car.moving = true;
    }
}

pub fn traffic_lights_sys(lights: &mut TrafficLights) {
    let green_duration = Duration::new(3, 0);
    let off_duration = Duration::new(0, 1); // 0.5s all-red

    let elapsed = lights.time.elapsed();

    if lights.state {
        if elapsed >= green_duration {
            lights.lights_right = false;
            lights.lights_top = false;
            lights.lights_left = false;
            lights.lights_down = false;

            lights.state = false;
            lights.time = Instant::now();
        }
    } else if elapsed >= off_duration {
        lights.current_direction = match lights.current_direction {
            Direction::Right => Direction::Top,
            Direction::Top => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
        };

        match lights.current_direction {
            Direction::Right => lights.lights_right = true,
            Direction::Top => lights.lights_top = true,
            Direction::Left => lights.lights_left = true,
            Direction::Down => lights.lights_down = true,
        }

        lights.state = true;
        lights.time = Instant::now();
    }
}
