use getrandom::getrandom;

pub fn next_move(difficulty: bool, board: Vec<Vec<u8>>) -> i32 {
    let is_draw = board.iter().all(|row| row.iter().all(|&cell| cell != 0));

    if is_draw {
        return -1;
    } else {
        if difficulty {
            calculate_hard_move(&board) as i32
        } else {
            calculate_easy_move(&board) as i32
        }
    }
}

fn calculate_easy_move(board: &Vec<Vec<u8>>) -> usize {
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
                            return move_pos;
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
        random_choice_from_list(&available_columns)
    } else {
        0
    }
}

fn calculate_hard_move(board: &Vec<Vec<u8>>) -> usize {
    0
}

// fn calculate_hard_move(board: &Vec<Vec<u8>>) -> (usize, usize) {
//     let depth = 4;
//     let alpha = i32::MIN;
//     let beta = i32::MAX;
//     let maximizing_player = true;

//     let (_, best_move) = minimax(board, depth, alpha, beta, maximizing_player);
//     best_move
// }

// fn minimax(board: &Vec<Vec<u8>>, depth: usize, alpha: i32, beta: i32, maximizing_player: bool) -> (i32, (usize, usize)) {
//     if depth == 0 || check_winner(&board) != 0 {
//         return (evaluate_board(board), (0, 0));
//     }

//     let mut alpha = alpha;
//     let mut beta = beta;

//     if maximizing_player {
//         let mut max_eval = i32::MIN;
//         let mut best_move = (0, 0);

//         for (x, y) in generate_moves(board) {
//             let mut new_board = board.clone();
//             new_board[x][y] = 2;
//             let eval = minimax(&new_board, depth - 1, alpha, beta, false).0;
//             if eval > max_eval {
//                 max_eval = eval;
//                 best_move = (x, y);
//             }
//             alpha = alpha.max(eval);
//             if beta <= alpha {
//                 break;
//             }
//         }
//         (max_eval, best_move)
//     } else {
//         let mut min_eval = i32::MAX;
//         let mut best_move = (0, 0);

//         for (x, y) in generate_moves(board) {
//             let mut new_board = board.clone();
//             new_board[x][y] = 1;
//             let eval = minimax(&new_board, depth - 1, alpha, beta, true).0;
//             if eval < min_eval {
//                 min_eval = eval;
//                 best_move = (x, y);
//             }
//             beta = beta.min(eval);
//             if beta <= alpha {
//                 break;
//             }
//         }
//         (min_eval, best_move)
//     }
// }

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

// fn generate_moves(board: &Vec<Vec<u8>>) -> Vec<(usize, usize)> {
//     let mut moves = Vec::new();
//     let cols = board[0].len();
//     let rows = board.len();

//     for col in 0..cols {
//         for row in (0..rows).rev() {
//             if board[row][col] == 0 {
//                 moves.push((row, col));
//                 break;
//             }
//         }
//     }
//     moves
// }

// fn evaluate_board