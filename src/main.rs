use raylib::{color::Color, ffi::KeyboardKey, math::Rectangle, prelude::RaylibDraw};

const STARTER_WINDOW: (i32, i32) = (800,600);
const TARGET_FPS: u32 = 60;
const WINDOW_TITLE :&str = "RUST SNAKE";
const INITIAL_SQUARES_PER_ROW :i32 = 16;

mod entities;

use crate::entities::character::{*};
use crate::entities::items::{*};
use crate::entities::screen::{*};


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
            position.move_to(Direction::Down);
        } else if rl.is_key_pressed(KeyboardKey::KEY_W) {
            position.move_to(Direction::Up);
        } else if rl.is_key_pressed(KeyboardKey::KEY_A) {
            position.move_to(Direction::Right);
        } else if rl.is_key_pressed(KeyboardKey::KEY_D) {
            position.move_to(Direction::Left);
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
