use macroquad::prelude::*;

// Car movement directions
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Direction {
    Top,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Car {
    pub x: i32,
    pub y: i32,
    pub dir: Direction,
    pub color: Color,
    pub turned: bool,
    pub moving: bool,
}

const PURPLE_RGB: Color = Color {
    r: 160.0 / 255.0,
    g: 32.0 / 255.0,
    b: 240.0 / 255.0,
    a: 1.0,
};

impl Car {
    pub fn new(x: i32, y: i32, dir: Direction, color: Color) -> Car {
        Car {
            x,
            y,
            dir,
            color,
            turned: false,
            moving: false,
        }
    }

    pub fn move_car(&mut self) {
        match self.dir {
            Direction::Top => self.y += 1,
            Direction::Down => self.y -= 1,
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
        }
    }

    pub fn redirect(&mut self) {
        // from Top
        if self.dir == Direction::Top {
            if self.x == 360 && self.y == 260 && self.color == PURPLE_RGB {
                self.dir = Direction::Left;
                self.turned = true;
            } else if self.x == 360 && self.y == 310 && self.color == YELLOW {
                self.dir = Direction::Right;
                self.turned = true;
            }
        }
        // from Down
        else if self.dir == Direction::Down {
            if self.y == 260 && self.x == 410 && self.color == YELLOW {
                self.dir = Direction::Left;
                self.turned = true;
            } else if self.y == 310 && self.x == 410 && self.color == PURPLE_RGB {
                self.dir = Direction::Right;
                self.turned = true;
            }
        }
        // from Right
        else if self.dir == Direction::Right {
            if self.x == 360 && self.y == 310 && self.color == PURPLE_RGB {
                self.dir = Direction::Top;
                self.turned = true;
            } else if self.x == 410 && self.y == 310 && self.color == YELLOW {
                self.dir = Direction::Down;
                self.turned = true;
            }
        }
        // from Left
        else if self.dir == Direction::Left {
            if self.x == 360 && self.y == 260 && self.color == YELLOW {
                self.dir = Direction::Top;
                self.turned = true;
            } else if self.x == 410 && self.y == 260 && self.color == PURPLE_RGB {
                self.dir = Direction::Down;
                self.turned = true;
            }
        }
    }

    pub fn random_c() -> Color {
        match macroquad::rand::gen_range(0, 3) {
            0 => BLUE,
            1 => YELLOW,
            _ => PURPLE_RGB,
        }
    }

    pub fn next_car(&self, cars_iter: &Vec<Car>) -> bool {
        const SAFE_DISTANCE: i32 = 65;
        for other in cars_iter {
            if other.dir == self.dir {
                match self.dir {
                    Direction::Top => {
                        if other.y > self.y && other.y - self.y <= SAFE_DISTANCE {
                            return true;
                        }
                    }
                    Direction::Down => {
                        if other.y < self.y && self.y - other.y <= SAFE_DISTANCE {
                            return true;
                        }
                    }
                    Direction::Left => {
                        if other.x < self.x && self.x - other.x <= SAFE_DISTANCE {
                            return true;
                        }
                    }
                    Direction::Right => {
                        if other.x > self.x && other.x - self.x <= SAFE_DISTANCE {
                            return true;
                        }
                    }
                }
            }
        }
        false
    }
}

pub fn get_last_car_dir(cars_vec: &mut Vec<Car>, dir: Direction) -> Option<Car> {
    cars_vec.iter().rev().find(|car| car.dir == dir).cloned()
}

pub fn push_car(
    cars_vec: &mut Vec<Car>,
    car: Car,
    check_y: Option<i32>,
    check_x: Option<i32>,
    dir: Direction,
) {
    const MAX_CARS: usize = 20;

    let can_push = if cars_vec.is_empty() {
        true
    } else {
        let last = get_last_car_dir(cars_vec, dir);
        match last {
            Some(last_car) => match (check_y, check_x) {
                (Some(y_limit), None) => {
                    if car.dir == Direction::Down {
                        last_car.y < y_limit
                    } else {
                        last_car.y > y_limit
                    }
                }
                (None, Some(x_limit)) => {
                    if car.dir == Direction::Right {
                        last_car.x > x_limit
                    } else {
                        last_car.x < x_limit
                    }
                }
                _ => false,
            },
            None => true,
        }
    };

    if can_push && cars_vec.len() < MAX_CARS {
        cars_vec.push(car);
    }
}

pub fn key_up(cars_vec: &mut Vec<Car>) {
    let car = Car::new(410, 600, Direction::Down, Car::random_c());
    push_car(cars_vec, car, Some(510), None, Direction::Down);
}

pub fn key_down(cars_vec: &mut Vec<Car>) {
    let car = Car::new(360, -30, Direction::Top, Car::random_c());
    push_car(cars_vec, car, Some(60), None, Direction::Top);
}

pub fn key_left(cars_vec: &mut Vec<Car>) {
    let car = Car::new(-30, 310, Direction::Right, Car::random_c());
    push_car(cars_vec, car, None, Some(60), Direction::Right);
}

pub fn key_right(cars_vec: &mut Vec<Car>) {
    let car = Car::new(800, 260, Direction::Left, Car::random_c());
    push_car(cars_vec, car, None, Some(740), Direction::Left);
}

pub fn key_r(cars_vec: &mut Vec<Car>) {
    match macroquad::rand::gen_range(0, 4) {
        0 => key_up(cars_vec),    // from bottom, going up
        1 => key_down(cars_vec),  // from top, going down
        2 => key_left(cars_vec),  // from left, going right
        _ => key_right(cars_vec), // from right, going left
    }
}