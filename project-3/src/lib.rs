mod utils;
mod connect_4_computer;

use wasm_bindgen::prelude::*;
use connect_4_computer::{check_winner, calculate_easy_move, calculate_hard_move};
use js_sys::Array;
use wasm_bindgen::JsValue;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, project-3!");
}

#[wasm_bindgen]
pub fn next_move(difficulty: bool, board_js: Array, rand: u8) -> Array {
    let board: Vec<Vec<u8>> = board_js.iter().map(|row| {
        let row_array: Array = row.into();
        row_array.iter().map(|cell| cell.as_f64().unwrap() as u8).collect()
    }).collect();

    let winner = check_winner(&board);

    let result = Array::new();

    let is_draw = board.iter().all(|row| row.iter().all(|&cell| cell != 0));

    match winner {
        0 if !is_draw => {
            let next_move = if difficulty {
                calculate_hard_move(&board)
            } else {
                calculate_easy_move(&board, rand)
            };
            result.push(&JsValue::from(0));
            result.push(&JsValue::from(next_move.0 as i32));
            result.push(&JsValue::from(next_move.1 as i32));
        },
        _ if is_draw => {
            result.push(&JsValue::from(4));
            result.push(&JsValue::from(-1));
            result.push(&JsValue::from(-1));
        },
        _ => {
            result.push(&JsValue::from(winner));
            result.push(&JsValue::from(-1));
            result.push(&JsValue::from(-1));
        },
    }

    result
}
