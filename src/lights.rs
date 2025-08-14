use crate::{ Car, Direction };
pub use std::time::{ Duration, Instant };

pub struct TrafficLights {
    pub lights_top: bool,
    pub lights_down: bool,
    pub lights_left: bool,
    pub lights_right: bool,
    pub current_direction: Direction,
    pub state: bool, // true = green on current_direction, false = all-red
    pub time: Instant, // phase start
}

pub fn traffic_lights(car: &mut Car, lights: &TrafficLights) {
    if
        (!lights.lights_down && car.dir == Direction::Down && car.y == 420) ||
        (!lights.lights_top && car.dir == Direction::Top && car.y == 240) ||
        (!lights.lights_right && car.dir == Direction::Right && car.x == 300) ||
        (!lights.lights_left && car.dir == Direction::Left && car.x == 470)
    {
        car.moving = false;
    } else {
        car.moving = true;
    }
}

// Half-line activation, origin-based, no spawn preemption
pub fn traffic_lights_sys(lights: &mut TrafficLights, cars: &[Car]) {
    use Direction::*;

    let green_dur = Duration::from_millis(2500);
    let clearance = Duration::from_millis(1200);

    // (dir, spawn_x, spawn_y, stop_x, stop_y) — matches your spawners/stop-lines
    const LANES: [(Direction, i32, i32, i32, i32); 4] = [
        (Right, -30, 360, 300, 360), // from left → right
        (Top, 360, -30, 360, 240), // from top  → down
        (Left, 800, 310, 470, 310), // from right→ left
        (Down, 410, 700, 410, 420), // from bot  → up
    ];

    // A lane is active only if a car from that ORIGIN has crossed the halfway point toward its stop line
    let active: [bool; 4] = LANES.map(|(dir, sx, sy, ex, ey)| {
        let mid_x = (sx + ex) / 2;
        let mid_y = (sy + ey) / 2;
        cars.iter().any(|c| {
            c.origin == dir &&
                !c.turned &&
                c.dir == dir &&
                (if sx == ex {
                    // vertical approach: fixed x, progress on y toward ey
                    c.x == sx && (if sy < ey { c.y >= mid_y } else { c.y <= mid_y })
                } else {
                    // horizontal approach: fixed y, progress on x toward ex
                    c.y == sy && (if sx < ex { c.x >= mid_x } else { c.x <= mid_x })
                })
        })
    });

    let active_count = active
        .iter()
        .filter(|&&b| b)
        .count();

    #[inline]
    fn dir_idx(d: Direction) -> usize {
        match d {
            Direction::Right => 0,
            Direction::Top => 1,
            Direction::Left => 2,
            Direction::Down => 3,
        }
    }
    #[inline]
    fn idx_dir(i: usize) -> Direction {
        match i {
            0 => Direction::Right,
            1 => Direction::Top,
            2 => Direction::Left,
            _ => Direction::Down,
        }
    }
    #[inline]
    fn set_green(l: &mut TrafficLights, i: usize) {
        l.lights_right = i == 0;
        l.lights_top = i == 1;
        l.lights_left = i == 2;
        l.lights_down = i == 3;
        l.current_direction = idx_dir(i);
    }
    #[inline]
    fn all_red(l: &mut TrafficLights) {
        l.lights_right = false;
        l.lights_top = false;
        l.lights_left = false;
        l.lights_down = false;
    }
    #[inline]
    fn next_active_idx(cur_i: usize, active: [bool; 4]) -> usize {
        for step in 1..=4 {
            let i = (cur_i + step) % 4;
            if active[i] {
                return i;
            }
        }
        cur_i
    }

    // no active lines → all red
    if active_count == 0 {
        all_red(lights);
        lights.state = false;
        return;
    }

    // exactly one active line → keep it green
    if active_count == 1 {
        let i = active
            .iter()
            .position(|&b| b)
            .unwrap();
        set_green(lights, i);
        lights.state = true;
        lights.time = Instant::now();
        return;
    }

    let elapsed = lights.time.elapsed();
    let cur_i = dir_idx(lights.current_direction);

    if lights.state {
        // stay green until timer ends or current line no longer has someone past half-line
        if !active[cur_i] || elapsed >= green_dur {
            all_red(lights);
            lights.state = false;
            lights.time = Instant::now();
        }
    } else if elapsed >= clearance {
        let next_i = next_active_idx(cur_i, active);
        set_green(lights, next_i);
        lights.state = true;
        lights.time = Instant::now();
    }
}
