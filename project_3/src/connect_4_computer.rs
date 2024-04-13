use getrandom::getrandom;

pub fn next_move(difficulty: bool, board: Vec<Vec<u8>>) -> u8 {
    let mut corrected_board = vec![vec![0; board.len()]; board[0].len()];
    for (i, row) in board.iter().enumerate() {
        for (j, &value) in row.iter().enumerate() {
            corrected_board[j][i] = value;
        }
    }

    corrected_board.reverse();

    let is_draw = corrected_board.iter().all(|row| row.iter().all(|&cell| cell != 0));

    if is_draw {
        return 0;
    } else {
        if difficulty {
            calculate_hard_move(&corrected_board)
        } else {
            calculate_easy_move(&corrected_board)
        }
    }
}

fn calculate_easy_move(board: &Vec<Vec<u8>>) -> u8 {
    let rows = board.len();
    let cols = board[0].len();

    for priority in [3, 2] {
        for x in 0..rows {
            for y in 0..cols {
                for dx in 0..=1 {
                    for dy in -1..=1 {
                        if dy == 0 && dx == 0 { continue; }

                        let mut count_user = 0;
                        let mut empty_spot = None;

                        for step in 0..4 {
                            let nx = x as isize + dx as isize * step;
                            let ny = y as isize + dy as isize * step;
                            if nx < 0 || ny < 0 || nx >= rows as isize || ny >= cols as isize {
                                break;
                            }
                            match board[nx as usize][ny as usize] {
                                1 => count_user += 1,
                                0 => {
                                    if empty_spot.is_none() {
                                        empty_spot = Some((nx as usize, ny as usize));
                                    }
                                },
                                _ => {}
                            }
                        }

                        if count_user == priority {
                            if let Some((ex, ey)) = empty_spot {
                                if ex == rows - 1 || board[ex + 1][ey] != 0 {
                                    return ey as u8;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    let mut available_columns = Vec::new();
    for col in 0..cols {
        if board[0][col] == 0 {
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
        random_choice_from_list(&available_columns) as u8
    } else {
        0
    }
}

// fn calculate_hard_move(board: &Vec<Vec<u8>>) -> usize {
//     0
// }

fn calculate_hard_move(board: &Vec<Vec<u8>>) -> u8 {
    let depth = 5;
    let alpha = i32::MIN;
    let beta = i32::MAX;
    let maximizing_player = true;

    let (_, best_move) = minimax(board, depth, alpha, beta, maximizing_player);
    best_move.1 as u8
}

fn minimax(board: &Vec<Vec<u8>>, depth: usize, alpha: i32, beta: i32, maximizing_player: bool) -> (i32, (usize, usize)) {
    if depth == 0 || check_winner(board) != 0 {
        return (evaluate_board(board), (0, 0));
    }

    let mut alpha = alpha;
    let mut beta = beta;

    if maximizing_player {
        let mut max_eval = i32::MIN;
        let mut best_move = (0, 0);

        for (x, y) in generate_moves(board) {
            let mut new_board = board.clone();
            new_board[x][y] = 2;
            let eval = minimax(&new_board, depth - 1, alpha, beta, false).0;
            if eval > max_eval {
                max_eval = eval;
                best_move = (x, y);
            }
            alpha = alpha.max(eval);
            if beta <= alpha {
                break;
            }
        }
        (max_eval, best_move)
    } else {
        let mut min_eval = i32::MAX;
        let mut best_move = (0, 0);

        for (x, y) in generate_moves(board) {
            let mut new_board = board.clone();
            new_board[x][y] = 1;
            let eval = minimax(&new_board, depth - 1, alpha, beta, true).0;
            if eval < min_eval {
                min_eval = eval;
                best_move = (x, y);
            }
            beta = beta.min(eval);
            if beta <= alpha {
                break;
            }
        }
        (min_eval, best_move)
    }
}

fn check_winner(board: &Vec<Vec<u8>>) -> u8 {
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

fn generate_moves(board: &Vec<Vec<u8>>) -> Vec<(usize, usize)> {
    let mut moves = Vec::new();
    let cols = board[0].len();
    let rows = board.len();

    for col in 0..cols {
        for row in (0..rows).rev() {
            if board[row][col] == 0 {
                moves.push((row, col));
                break;
            }
        }
    }
    moves
}

fn evaluate_board(board: &Vec<Vec<u8>>) -> i32 {
    let mut score = 0;
    let rows = board.len();
    let cols = board[0].len();

    let three_in_a_row = 100;
    let two_in_a_row = 10;
    let one_in_a_row = 1;

    for row in 0..rows {
        for col in 0..cols {
            if board[row][col] == 0 {
                continue;
            }

            if col <= cols - 4 {
                let mut count_user = 0;
                let mut count_comp = 0;
                for k in 0..4 {
                    if board[row][col + k] == 1 {
                        count_user += 1;
                    } else if board[row][col + k] == 2 {
                        count_comp += 1;
                    }
                }
                score += evaluate_line(count_user, count_comp, three_in_a_row, two_in_a_row, one_in_a_row);
            }

            if row <= rows - 4 {
                let mut count_user = 0;
                let mut count_comp = 0;
                for k in 0..4 {
                    if board[row + k][col] == 1 {
                        count_user += 1;
                    } else if board[row + k][col] == 2 {
                        count_comp += 1;
                    }
                }
                score += evaluate_line(count_user, count_comp, three_in_a_row, two_in_a_row, one_in_a_row);
            }

            if col <= cols - 4 && row <= rows - 4 {
                let mut count_user = 0;
                let mut count_comp = 0;
                for k in 0..4 {
                    if board[row + k][col + k] == 1 {
                        count_user += 1;
                    } else if board[row + k][col + k] == 2 {
                        count_comp += 1;
                    }
                }
                score += evaluate_line(count_user, count_comp, three_in_a_row, two_in_a_row, one_in_a_row);
            }

            if col >= 3 && row <= rows - 4 {
                let mut count_user = 0;
                let mut count_comp = 0;
                for k in 0..4 {
                    if board[row + k][col - k] == 1 {
                        count_user += 1;
                    } else if board[row + k][col - k] == 2 {
                        count_comp += 1;
                    }
                }
                score += evaluate_line(count_user, count_comp, three_in_a_row, two_in_a_row, one_in_a_row);
            }
        }
    }

    score
}

fn evaluate_line(count_user: u8, count_comp: u8, three_in_a_row: i32, two_in_a_row: i32, one_in_a_row: i32) -> i32 {
    if count_user > 0 && count_comp > 0 {
        0
    } else if count_comp == 3 {
        three_in_a_row
    } else if count_comp == 2 {
        two_in_a_row
    } else if count_comp == 1 {
        one_in_a_row
    } else if count_user == 3 {
        -three_in_a_row
    } else if count_user == 2 {
        -two_in_a_row
    } else if count_user == 1 {
        -one_in_a_row
    } else {
        0
    }
}
