use rand::Rng;

pub struct Items {
    pub coin_position: i32,
    pub number_of_squares: i32
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
