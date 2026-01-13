
#[derive(PartialEq, Eq)]
pub enum Direction {
    Up,
    Left,
    Down,
    Right,
}


pub struct Character {
    pub current_index : i32,
    pub future_index: i32,
    pub number_of_squares_per_row : i32,
    pub last_position_direction: Direction,
    pub timer: f32,
    pub tail_positions: Vec<i32>,
    pub tail_length: i32
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

    pub fn add_tail_length(&mut self) {
        self.tail_length+=1;
    }

    pub fn update(&mut self, delta: f32) {
        self.timer+= delta;
        if self.timer > 0.5 {
            self.timer = 0.0;
            self.update_tail_positions();
            self.set_future_index();
            self.set_current_index();
        }
    }

    fn update_tail_positions(&mut self) {
        self.tail_positions.insert(0,self.current_index);
        let end = self.tail_length as usize;
        let end = end.min(self.tail_positions.len());
        let new_vector = self.tail_positions[0..end].to_vec();
        self.tail_positions = new_vector;
    }
    pub fn move_to(&mut self, direction: Direction) {
        if direction == self.last_position_direction {
            return;
        }
        self.last_position_direction = direction;
        self.update_tail_positions();
        self.set_future_index();
        self.set_current_index();
        self.timer = 0.0;
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
