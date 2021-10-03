pub struct Board {
    row: usize,
    col: usize,
    values: Vec<u8>,
}

impl Board {
    pub fn new(row: usize, col: usize) -> Board {
        Board {
            row,
            col,
            values: vec![0; row * col],
        }
    }
    pub fn set(&mut self, x: usize, y: usize, value: u8) -> Result<(), &'static str> {
        if x >= self.row || y >= self.col {
            Err("Out of bounds")
        } else {
            self.values[y + x * self.col] = value;
            Ok(())
        }
    }
    pub fn get(&self, x: usize, y: usize) -> Option<u8> {
        if x >= self.row || y >= self.col {
            None
        } else {
            Some(self.values[y + x * self.col])
        }
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
                assert_eq!(Some(0), one_board.get(x, y));
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
        let res = one_board.set(x, y, value);

        assert_eq!(Ok(()), res);
        assert_eq!(Some(value), one_board.get(x, y));
    }

    #[test]
    fn board_set_failed() {
        let row = 3;
        let col = 4;
        let mut one_board = Board::new(row, col);

        let res = one_board.set(row, col - 1, 3);

        assert_eq!(Err("Out of bounds"), res);
    }

    #[test]
    fn board_get_failed() {
        let row = 3;
        let col = 4;
        let one_board = Board::new(row, col);

        let res = one_board.get(row, col - 1);
        assert_eq!(None, res);
    }
}
