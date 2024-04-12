pub fn check_winner(board: &Vec<Vec<u8>>) -> u8 {
    let rows = board.len();
    let cols = board[0].len();
    let directions = [(0,1), (1,0), (1,1), (1,-1)];

    for x in 0..rows {
        for y in 0..cols {
            if board[x][y] != 0 {
                for dir in directions.iter() {
                    let mut count = 1;
                    let mut dx = x as i32 + dir.0;
                    let mut dy = y as i32 + dir.1;

                    while dx >= 0 && dx < rows as i32 && dy >= 0 && dy < cols as i32 && board[dx as usize][dy as usize] == board[x][y] {
                        count += 1;
                        if count == 4 {
                            return board[x][y];
                        }
                        dx += dir.0;
                        dy += dir.1;
                    }
                }
            }
        }
    }

    0
}

pub fn calculate_easy_move(board: &Vec<Vec<u8>>, rand: u8) -> (usize, usize) {
    let rows = board.len();
    let cols = board[0].len();

    for priority in [3, 2] {
        for x in 0..rows {
            for y in 0..(cols - 3) {
                for player in [2, 1] {
                    let slice = &board[x][y..=y+3];
                    if slice.iter().filter(|&&cell| cell == player).count() == priority && slice.contains(&0) {
                        let move_pos = y + slice.iter().position(|&cell| cell == 0).unwrap();
                        if x == rows - 1 || board[x + 1][move_pos] != 0 {
                            return (x, move_pos);
                        }
                    }
                }
            }
        }
    }

    let col = rand as usize % cols;

    for offset in 0..cols {
        let current_col = (col + offset) % cols;
        if board[0][current_col] == 0 {
            for x in (0..rows).rev() {
                if board[x][current_col] == 0 {
                    return (x, current_col);
                }
            }
        }
    }

    (0, 0)
}


// Dummy function for calculating a hard move
// Implement a more complex strategy for the hard difficulty
pub fn calculate_hard_move(board: &Vec<Vec<u8>>) -> (usize, usize) {
    // For simplicity, just return a placeholder value
    // Implement your strategy for finding a hard move
    (0, 0)
}