use getrandom::getrandom;

pub fn next_move(difficulty: bool, board: Vec<Vec<u8>>) -> Vec<i32> {
    let winner = check_winner(&board);

    let mut result = Vec::new();

    let is_draw = board.iter().all(|row| row.iter().all(|&cell| cell != 0));

    match winner {
        0 if !is_draw => {
            let next_move = if difficulty {
                calculate_hard_move(&board)
            } else {
                calculate_easy_move(&board)
            };
            result.push(0);
            result.push(next_move.0 as i32);
            result.push(next_move.1 as i32);
        },
        _ if is_draw => {
            result.push(4);
            result.push(-1);
            result.push(-1);
        },
        _ => {
            result.push(winner as i32);
            result.push(-1);
            result.push(-1);
        },
    }

    result
}

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

pub fn calculate_easy_move(board: &Vec<Vec<u8>>) -> (usize, usize) {
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

    let mut available_columns = Vec::new();
    for col in 0..cols {
        if board.iter().any(|row| row[col] == 0) {
            available_columns.push(col);
        }
    }

    fn random_choice_from_list(list: &Vec<usize>) -> usize {
        let mut buf = [0u8; 1];
        getrandom(&mut buf).expect("Failed to generate random data");
        let index = (buf[0] as usize) % list.len();
        list[index]
    }

    if !available_columns.is_empty() {
        let col = random_choice_from_list(&available_columns);
        for x in (0..rows).rev() {
            if board[x][col] == 0 {
                return (x, col);
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