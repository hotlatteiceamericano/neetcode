use std::collections::HashSet;

fn solution(board: Vec<Vec<char>>) -> bool {
    // check for row
    for row in &board {
        let mut set = HashSet::new();
        let mut invalid_row = false;
        for col in row {
            if *col == '.' {
                continue;
            }
            if set.contains(&col) {
                invalid_row = true;
                break;
            }
            set.insert(col);
        }
        if invalid_row {
            return false;
        }
    }

    // check for columns
    let num_row = board.len();
    let num_col = board[0].len();
    for i in 0..num_row {
        let mut set = HashSet::new();
        let mut invalid_col = false;
        for j in 0..num_col {
            let col = &board[j][i];
            if *col == '.' {
                continue;
            }
            if set.contains(&col) {
                invalid_col = true;
                break;
            }
            set.insert(col);
        }
        if invalid_col {
            return false;
        }
    }

    // check for squares
    for square_i in 0..3 {
        for square_j in 0..3 {
            let mut set = HashSet::new();
            let mut invalid_square = false;
            for i in 0..3 {
                for j in 0..3 {
                    let cell = &board[i + (square_i * 3)][j + (square_j * 3)];
                    if *cell == '.' {
                        continue;
                    }
                    if !set.insert(cell) {
                        invalid_square = true;
                        break;
                    }
                }
                if invalid_square {
                    break;
                }
            }
            if invalid_square {
                return false;
            }
        }
    }

    true
}
