use raylib::{color::Color, ffi::KeyboardKey, math::Rectangle, prelude::RaylibDraw};

use rand::prelude::*;

const STARTER_WINDOW: (i32, i32) = (800,600);

const TARGET_FPS: u32 = 60;

const WINDOW_TITLE :&str = "RUST SNAKE";

struct Screen {
    width: i32,
    height: i32,
    base_unit: i32
}

impl Screen {
    pub fn init(width:i32, height:i32) -> Self {
        Self {
            width,
            height,
            base_unit: width / height
        }
    }

    pub fn resize(&mut self, width: i32, height: i32){
        self.width = width;
        self.height = height;
        self.base_unit = width / height;
    }
}

enum Direction {
    Up,
    Left,
    Down,
    Right,
}

struct Items {
    coin_position: i32,
    number_of_squares: i32
}

impl Items {
    pub fn init(number_of_squares: i32) -> Self {
        Self {
            number_of_squares: number_of_squares,
            coin_position: Items::random(number_of_squares),
        }
    }
    pub fn spawn_coin(&mut self) {
        self.coin_position = Items::random(self.number_of_squares);
    }

    fn random(range: i32) -> i32 {
        let mut rng = rand::rng();
        rng.random_range(0..range)
    }

}

struct Position {
    pub current_index : i32,
    future_index: i32,
    number_of_squares_per_row : i32,
    last_position_direction: Direction,
    timer: f32,
    tail_positions: Vec<i32>,
    tail_length: i32
}

impl Position {

    pub fn init() -> Self {
        Self {
            current_index: 0,
            future_index: 1,
            last_position_direction: Direction::Left,
            timer: 0.0,
            number_of_squares_per_row: 8,
            tail_positions: vec![],
            tail_length: 0,
        }
    }



    fn set_current_index (&mut self) {
        self.current_index = self.future_index;
    }

    pub fn set_direction(&mut self, new_dir: Direction) {
        self.last_position_direction = new_dir;
    }

    pub fn add_tail_length(&mut self) {
        self.tail_length+=1;
    }

    pub fn update(&mut self, delta: f32) {
        self.timer+= delta;
        if self.timer > 0.3 {
            self.timer = 0.0;
            self.tail_positions.insert(0,self.current_index);
            let new_vector = self.tail_positions[0..self.tail_length as usize].to_vec();
            self.tail_positions = new_vector;
            self.set_future_index();
            self.set_current_index();
        }
    }

    fn set_future_index(&mut self) {
        let total_squares = self.number_of_squares_per_row * self.number_of_squares_per_row;
        self.future_index = match self.last_position_direction {
            Direction::Down => {
                let future = self.current_index + self.number_of_squares_per_row;
                if future > total_squares {
                    future - total_squares
                } else {
                    future
                }
            }
            Direction::Left => {
                let future = self.current_index + 1;
                if future % self.number_of_squares_per_row == 0 {
                    self.current_index - self.number_of_squares_per_row + 1
                } else {
                    future
                }
            }
            Direction::Right => {
                let future = self.current_index - 1;
                if self.current_index % self.number_of_squares_per_row == 0 {
                    self.current_index + self.number_of_squares_per_row - 1
                } else if future < 0 {
                    self.current_index + self.number_of_squares_per_row - 1
                } else {
                    future
                }
            }
            Direction::Up => {
                let future = self.current_index - self.number_of_squares_per_row;
                if future < 0 {
                    future + total_squares
                } else {
                    future
                }
            }
        }
    }
}

fn main() {
    let mut screen = Screen::init(STARTER_WINDOW.0,STARTER_WINDOW.1);

    let (mut rl, thread) = raylib::init()
        .size(screen.width, screen.height)
        .title(WINDOW_TITLE)
        .resizable()
        .build();
    rl.set_target_fps(TARGET_FPS);

    let canvas = Rectangle::new(rl.get_screen_width() as f32 / 4.0, rl.get_screen_height() as f32 / 8.0, rl.get_screen_width() as f32 / 2.0,  rl.get_screen_width() as f32 / 2.0);

    let number_of_squares_per_row = 8;
    const MAX_NUMBER_OF_SQUARES : i32 = 64;
    let unit_size = canvas.width as i32 / number_of_squares_per_row;

    let mut position = Position::init();
    let mut items = Items::init(MAX_NUMBER_OF_SQUARES);

    while !rl.window_should_close() {

        if rl.is_window_resized() {
            screen.resize(rl.get_screen_width(),rl.get_screen_height());
        }


        if rl.is_key_pressed(KeyboardKey::KEY_S) {
            position.set_direction(Direction::Down);
        } else if rl.is_key_pressed(KeyboardKey::KEY_W) {
            position.set_direction(Direction::Up);
        } else if rl.is_key_pressed(KeyboardKey::KEY_A) {
            position.set_direction(Direction::Right);
        } else if rl.is_key_pressed(KeyboardKey::KEY_D) {
            position.set_direction(Direction::Left);
        }

        if position.current_index == items.coin_position {
            items.spawn_coin();
            position.add_tail_length();
        }

        position.update(rl.get_frame_time());

        let mut starts_at_x = canvas.x as i32;
        let mut starts_at_y = canvas.y as i32;

        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::BLACK);

        for (i, _num) in [ unit_size ; MAX_NUMBER_OF_SQUARES as usize].into_iter().enumerate() {
            if i % number_of_squares_per_row as usize == 0 && i >= number_of_squares_per_row as usize {
                starts_at_x = canvas.x as i32;
                starts_at_y += unit_size;
            }
            if i == position.current_index as usize {
                d.draw_rectangle(starts_at_x, starts_at_y, unit_size, unit_size, Color::BEIGE);
            } else if i == items.coin_position as usize {
                d.draw_rectangle(starts_at_x, starts_at_y, unit_size, unit_size, Color::GREEN);
            } else {
                d.draw_rectangle_lines(starts_at_x, starts_at_y, unit_size, unit_size, Color::BEIGE);
            }

            for tail in position.tail_positions.as_slice() {
                if *tail == i as i32 {
                    d.draw_rectangle(starts_at_x, starts_at_y, unit_size, unit_size, Color::DARKGRAY);
                }
            }

            starts_at_x += unit_size;
        }

    }
}
