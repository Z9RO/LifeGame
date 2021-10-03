use board::Board;
use rand::{prelude::ThreadRng, Rng};

mod board;

pub struct LifeGame {
    row: usize,
    col: usize,
    base_board: Board,
    rng: ThreadRng,
}

fn mask(val: u8) -> u8 {
    val & 0xF
}

const XYS: [(usize, usize); 8] = [
    (0, 0),
    (0, 1),
    (0, 2),
    (1, 0),
    (1, 2),
    (2, 0),
    (2, 1),
    (2, 2),
];

impl LifeGame {
    pub fn new(row: usize, col: usize) -> LifeGame {
        LifeGame {
            row,
            col,
            base_board: Board::new(row + 2, col + 2),
            rng: rand::thread_rng(),
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<bool> {
        if x > self.row || y > self.col {
            None
        } else {
            Some(self.base_board.get(x, y) > 0)
        }
    }

    pub fn set(&mut self, x: usize, y: usize) -> Result<(), &'static str> {
        if x > self.row || y > self.col {
            Err("Out of bounds")
        } else {
            self.base_board.set(x, y, 1);
            Ok(())
        }
    }

    pub fn next(&mut self) {
        for x in 0..self.row {
            for y in 0..self.col {
                let neighbours = (&XYS).into_iter().fold(0, |part_sum, (x, y)| {
                    part_sum + mask(self.base_board.get(*x, *y))
                });

                let state = mask(self.base_board.get(x + 1, y + 1));
                if state > 0 {
                    if neighbours == 2 || neighbours == 3 {
                        self.base_board.set(x + 1, y + 1, 0x11);
                    }
                } else if neighbours == 3 {
                    self.base_board.set(x + 1, y + 1, 0x10);
                }
            }
        }

        for x in 1..=self.row {
            for y in 1..=self.col {
                let state = self.base_board.get(x, y);
                self.base_board.set(x, y, state >> 4);
            }
        }
    }

    pub fn rand_set(&mut self) {
        for x in 1..=self.row {
            for y in 1..=self.col {
                let value = if self.rng.gen_bool(0.5) { 1 } else { 0 };
                self.base_board.set(x, y, value);
            }
        }
    }
}
