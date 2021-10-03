pub struct Board {
    col: usize,
    values: Vec<u8>,
}

impl Board {
    pub fn new(row: usize, col: usize) -> Board {
        Board {
            col,
            values: vec![0; row * col],
        }
    }
    pub fn set(&mut self, x: usize, y: usize, value: u8) {
        self.values[y + x * self.col] = value;
    }
    pub fn get(&self, x: usize, y: usize) -> u8 {
        self.values[y + x * self.col]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn board_init() {
        let row = 3;
        let col = 4;
        let one_board = Board::new(row, col);

        for x in 0..row {
            for y in 0..col {
                assert_eq!(0, one_board.get(x, y));
            }
        }
    }

    #[test]
    fn board_set() {
        let row = 3;
        let col = 4;
        let mut one_board = Board::new(row, col);

        let value = 9;
        let x = 2;
        let y = 1;
        one_board.set(x, y, value);

        assert_eq!(value, one_board.get(x, y));
    }
}
