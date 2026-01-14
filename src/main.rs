use raylib::{color::Color, ffi::KeyboardKey, math::Rectangle, prelude::RaylibDraw};

const STARTER_WINDOW: (i32, i32) = (800, 600);
const TARGET_FPS: u32 = 60;
const WINDOW_TITLE: &str = "RUST SNAKE";
const INITIAL_SQUARES_PER_ROW: i32 = 16;

mod entities;

use crate::entities::character::*;
use crate::entities::items::*;
use crate::entities::screen::*;

fn main() {
    let mut screen = Screen::init(STARTER_WINDOW.0, STARTER_WINDOW.1);

    let (mut rl, thread) = raylib::init()
        .size(screen.width, screen.height)
        .title(WINDOW_TITLE)
        .resizable()
        .build();
    rl.set_target_fps(TARGET_FPS);

    let canvas = Rectangle::new(
        rl.get_screen_width() as f32 / 4.0,
        rl.get_screen_height() as f32 / 8.0,
        rl.get_screen_width() as f32 / 2.0,
        rl.get_screen_width() as f32 / 2.0,
    );

    let mut character = Character::init(INITIAL_SQUARES_PER_ROW);
    let mut items =
        Items::init(character.number_of_squares_per_row * character.number_of_squares_per_row);

    while !rl.window_should_close() {
        if rl.is_window_resized() {
            println!("Are we resizing");
            screen.resize(rl.get_screen_width(), rl.get_screen_height());
            println!("{} width, {} height", screen.width, screen.width);
        }

        let unit_size = canvas.width as i32 / character.number_of_squares_per_row;

        if rl.is_key_pressed(KeyboardKey::KEY_S) {
            character.move_to(Direction::Down);
        } else if rl.is_key_pressed(KeyboardKey::KEY_W) {
            character.move_to(Direction::Up);
        } else if rl.is_key_pressed(KeyboardKey::KEY_A) {
            character.move_to(Direction::Right);
        } else if rl.is_key_pressed(KeyboardKey::KEY_D) {
            character.move_to(Direction::Left);
        }

        if character.current_index == items.coin_position {
            character.add_tail_length();
            character.add_score();
            items.spawn_coin(&character.tail_positions);
        }

        character.update(rl.get_frame_time());

        let mut starts_at_x = canvas.x as i32;
        let mut starts_at_y = canvas.y as i32;

        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::new(123, 66, 145, 255));

        let mut squares: Vec<i32> = Vec::new();

        d.draw_text(&character.score.to_string(), 12, 12, 50, Color::WHITE);

        squares.resize(
            (character.number_of_squares_per_row * character.number_of_squares_per_row) as usize,
            unit_size,
        );

        d.draw_rectangle(
            canvas.x as i32,
            canvas.y as i32,
            canvas.width as i32,
            canvas.height as i32,
            Color::new(116, 51, 121, 255),
        );

        for (i, _num) in squares.into_iter().enumerate() {
            if i % character.number_of_squares_per_row as usize == 0
                && i >= character.number_of_squares_per_row as usize
            {
                starts_at_x = canvas.x as i32;
                starts_at_y += unit_size;
            }
            if i == character.current_index as usize {
                d.draw_rectangle(
                    starts_at_x,
                    starts_at_y,
                    unit_size,
                    unit_size,
                    Color::new(42, 148, 150, 255),
                );
            } else if i == items.coin_position as usize {
                d.draw_rectangle(
                    starts_at_x,
                    starts_at_y,
                    unit_size,
                    unit_size,
                    Color::new(244, 215, 112, 255),
                );
            }

            for tail in character.tail_positions.as_slice() {
                if *tail == i as i32 {
                    d.draw_rectangle(
                        starts_at_x,
                        starts_at_y,
                        unit_size,
                        unit_size,
                        Color::new(42, 148, 150, 120),
                    );
                }
            }

            starts_at_x += unit_size;
        }
    }
}
