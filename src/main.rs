use raylib::{color::Color, ffi::KeyboardKey, math::Rectangle, prelude::RaylibDraw};

use rand::prelude::*;

const STARTER_WINDOW: (i32, i32) = (800,600);

const TARGET_FPS: u32 = 60;

const WINDOW_TITLE :&str = "RUST SNAKE";

const INITIAL_SQUARES_PER_ROW :i32 = 16;

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
    pub fn spawn_coin(&mut self, occupied_positions: &Vec<i32>) {
        let all_positions: Vec<i32> = (0..self.number_of_squares).collect();
        let filtered: Vec<i32> = all_positions
            .into_iter()
            .filter(|pos| !occupied_positions.contains(pos))
            .collect();

        if filtered.is_empty() {
            self.coin_position = 0;
        } else {
            let random_index = Items::random(filtered.len() as i32);
            self.coin_position = filtered[random_index as usize];
        }
    }

    fn random(range: i32) -> i32 {
        let mut rng = rand::rng();
        rng.random_range(0..range)
    }

}

struct Character {
    pub current_index : i32,
    future_index: i32,
    number_of_squares_per_row : i32,
    last_position_direction: Direction,
    timer: f32,
    tail_positions: Vec<i32>,
    tail_length: i32
}

impl Character {

    pub fn init(number_of_squares_per_row: i32) -> Self {
        Self {
            current_index: 0,
            future_index: 1,
            last_position_direction: Direction::Left,
            timer: 0.0,
            number_of_squares_per_row: number_of_squares_per_row,
            tail_positions: vec![],
            tail_length: 0,
        }
    }

    fn reset (&mut self) {
        self.tail_positions.clear();
        self.current_index = 0;
        self.future_index = 1;
        self.tail_length = 0;
        self.last_position_direction = Direction::Left;
        self.timer = 0.0;
    }

    fn set_current_index (&mut self) {
        if self.tail_positions.contains(&self.future_index) {
            self.reset();
        }
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
        if self.timer > 0.1 {
            self.timer = 0.0;
            self.tail_positions.insert(0,self.current_index);
            let end = self.tail_length as usize;
            let end = end.min(self.tail_positions.len());
            let new_vector = self.tail_positions[0..end].to_vec();
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

    let mut position = Character::init(INITIAL_SQUARES_PER_ROW);
    let mut items = Items::init(position.number_of_squares_per_row * position.number_of_squares_per_row);
    let unit_size = canvas.width as i32 / position.number_of_squares_per_row;

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
            position.add_tail_length();
            items.spawn_coin(&position.tail_positions);
        }

        position.update(rl.get_frame_time());

        let mut starts_at_x = canvas.x as i32;
        let mut starts_at_y = canvas.y as i32;

        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::new(123, 66, 145, 255));

        let mut squares: Vec<i32> = Vec::new();

        squares.resize((position.number_of_squares_per_row * position.number_of_squares_per_row) as usize, unit_size);

        d.draw_rectangle(canvas.x as i32, canvas.y as i32, canvas.width as i32, canvas.height as i32, Color::new(116, 51, 121, 255));

        for (i, _num) in squares.into_iter().enumerate() {
            if i % position.number_of_squares_per_row as usize == 0 && i >= position.number_of_squares_per_row as usize {
                starts_at_x = canvas.x as i32;
                starts_at_y += unit_size;
            }
            if i == position.current_index as usize {
                d.draw_rectangle(starts_at_x, starts_at_y, unit_size, unit_size, Color::new(42,148,150,255));
            } else if i == items.coin_position as usize {
                d.draw_rectangle(starts_at_x, starts_at_y, unit_size, unit_size, Color::new(244,215,112,255));
            }

            for tail in position.tail_positions.as_slice() {
                if *tail == i as i32 {
                    d.draw_rectangle(starts_at_x, starts_at_y, unit_size, unit_size, Color::new(42,148,150,120));
                }
            }

            starts_at_x += unit_size;
        }

    }
}
