use std::fmt::format;

use yew::prelude::*;
use web_sys::HtmlInputElement;
mod connect_4_computer;
use web_sys::console;
use connect_4_computer::next_move;
enum Msg {
    StartGame,
    GoToHome,
    RestartGame,
    UpdateName(String),
    UpdateDifficulty(String),
    UpdateColorMode(String),
    UpdateBoardSize(String),
    UpdateCustomCols(i32),
    UpdateCustomRows(i32),
    PlacePiece(usize),

}

enum Player {
    Red,
    Yellow,
}
struct Model {
    name: String,
    difficulty: String,
    color_mode: String,
    board_size: String,
    custom_cols: i32,
    custom_rows: i32,
    board: Vec<Vec<Option<String>>>,
    game_started: bool,
    current_player: Player,
    game_over: bool,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self {
            name: "".into(),
            difficulty: "easy".into(),
            color_mode: "normal".into(),
            board_size: "standard".into(),
            custom_cols: 7,
            custom_rows: 6,
            board: vec![vec![None; 6]; 7],
            game_started: false,
            current_player: Player::Red,
            game_over: false,
        }
    }
    

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::StartGame => {
                self.game_started = true;    
                true
            }Msg::RestartGame =>{
                self.board = vec![vec![None; self.custom_rows as usize]; self.custom_cols as usize];
                self.game_started = true;
                self.game_over = false;
                self.current_player = Player::Red; // Or choose the player who starts the new game
                true
            }
            Msg::GoToHome =>{
                self.name = "".into();
                self.difficulty = "easy".into();
                self.color_mode = "normal".into();
                self.board_size = "standard".into();
                self.custom_cols = 7;
                self.custom_rows = 6;
                self.board = vec![vec![None; 6]; 7];
                self.game_started = false;
                self.current_player = Player::Red;
                self.game_over = false;
                true
            }
            Msg::PlacePiece(col_idx) => {
                self.place_piece(col_idx)
            },
            Msg::UpdateName(name) => {
                self.name = name;
                true
            }
            Msg::UpdateDifficulty(difficulty) => {
                self.difficulty = difficulty;
                true
            }
            Msg::UpdateColorMode(color_mode) => {
                self.color_mode = color_mode;
                true
            }
            Msg::UpdateBoardSize(board_size) => {
                self.board_size = board_size;
                true
            }
            Msg::UpdateCustomCols(cols) => {
                self.custom_cols = cols;
                true
            }
            Msg::UpdateCustomRows(rows) => {
                self.custom_rows = rows;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        if self.game_started {
            self.view_board(ctx);
            if self.game_over{
                html! {
                    <>
                        { self.view_board(ctx) }
                        <div class={"winner-announcement"} style={"text-align: center; margin-top: 20px; font-size: 24px; color: red;"}>
                            { self.winner_message() }
                        </div>
                        <button onclick={ctx.link().callback(|_| Msg::RestartGame)}>{"Play Again"}</button>
                        <button onclick={ctx.link().callback(|_| Msg::GoToHome)}>{"Home Screen"}</button>
                    </>
                }
            } else {
                self.view_board(ctx)
            }
        } else{
            html! {
                <div id="main-content">
                    <div id="main">
                        <div id="services">
                            <h5 class={"w3-xxxlarge w3-text-red"}><b>{"Enter Your Name"}</b></h5>
                        </div>
                        <div>
                            <input id="playerNameInput"
                                   type="text"
                                   placeholder="Your Name"
                                   value={self.name.clone()}
                                   oninput={ctx.link().callback(|e: InputEvent| {
                                       let input: HtmlInputElement = e.target_unchecked_into();
                                       Msg::UpdateName(input.value())
                                   })} />
                            <div id="difficultySelection">
                                <label>
                                    <input type="radio"
                                           name="difficulty"
                                           value="easy"
                                           checked={self.difficulty == "easy"}
                                           onchange={ctx.link().callback(|_| Msg::UpdateDifficulty("easy".to_string()))} />
                                    {" Easy"}
                                </label>
                                <label>
                                    <input type="radio"
                                           name="difficulty"
                                           value="hard"
                                           checked={self.difficulty == "hard"}
                                           onchange={ctx.link().callback(|_| Msg::UpdateDifficulty("hard".to_string()))} />
                                    {" Hard"}
                                </label>
                            </div>
                            <div id="colorBlindModeSelection">
                                <h5>{"Color Mode:"}</h5>
                                <label>
                                    <input type="radio"
                                           name="colorMode"
                                           value="normal"
                                           checked={self.color_mode == "normal"}
                                           onchange={ctx.link().callback(|_| Msg::UpdateColorMode("normal".to_string()))} />
                                    {" Normal"}
                                </label>
                                <label>
                                    <input type="radio"
                                           name="colorMode"
                                           value="colorBlind1"
                                           checked={self.color_mode == "colorBlind1"}
                                           onchange={ctx.link().callback(|_| Msg::UpdateColorMode("colorBlind1".to_string()))} />
                                    {" Color Blind Set 1"}
                                </label>
                                <label>
                                    <input type="radio"
                                           name="colorMode"
                                           value="colorBlind2"
                                           checked={self.color_mode == "colorBlind2"}
                                           onchange={ctx.link().callback(|_| Msg::UpdateColorMode("colorBlind2".to_string()))} />
                                    {" Color Blind Set 2"}
                                </label>
                            </div>
                            <div id="boardSizeSelection" style="align-items: center;">
                                <h5>{"Choose Board Size:"}</h5>
                                <label>
                                    <input type="radio"
                                        name="boardSize"
                                        value="standard"
                                        checked={self.board_size == "standard"}
                                        onchange={ctx.link().callback(|_| Msg::UpdateBoardSize("standard".to_string()))} />
                                    {" Standard (7 cols x 6 rows)"}
                                </label>
                                <label style="display: flex; align-items: center;">
                                    <input type="radio"
                                        name="boardSize"
                                        value="custom"
                                        checked={self.board_size == "custom"}
                                        onchange={ctx.link().callback(|_| Msg::UpdateBoardSize("custom".to_string()))} />
                                    {" Custom"}
                                    <div id="customSizeInputs" style={format!("display: {}; margin-left: 10px;", if self.board_size == "custom" { "flex" } else { "none" })}>
                                        <input id="customCols"
                                            type="number"
                                            placeholder="Cols"
                                            min="4"
                                            max="10"
                                            style="width: 60px; margin-right: 5px;"
                                            value={self.custom_cols.to_string()}
                                            oninput={ctx.link().callback(|e: InputEvent| {
                                                let input: HtmlInputElement = e.target_unchecked_into();
                                                Msg::UpdateCustomCols(input.value_as_number() as i32)
                                            })} />
                                        <input id="customRows"
                                            type="number"
                                            placeholder="Rows"
                                            min="4"
                                            max="10"
                                            style="width: 60px;"
                                            value={self.custom_rows.to_string()}
                                            oninput={ctx.link().callback(|e: InputEvent| {
                                                let input: HtmlInputElement = e.target_unchecked_into();
                                                Msg::UpdateCustomRows(input.value_as_number() as i32)
                                            })} />
                                    </div>
                                </label>
                            </div>
                            <button id="startGameButton" onclick={ctx.link().callback(|_| {
                                web_sys::console::log_1(&"Start Game button clicked".into());
                                Msg::StartGame
                            })}>
                                {"Start Game"}
                            </button>
                        </div>
                    </div>
                </div>
            }
        }
    }
}

impl Model {
    fn winner_message(&self) -> String {
        let mut name = "".to_string();
        if self.name.is_empty(){
            name = "Human".to_string();
        } else {
            name = self.name.to_string();
        }
        match self.current_player {
            Player::Red => "The Ai wins....".to_string(),
            Player::Yellow => format!("{:?} wins!", name),
        }
    }

    fn board_to_ai_format(&self) -> Vec<Vec<u8>> {
        self.board.iter().map(|col| {
            col.iter().map(|cell| match cell {
                Some(color) if color == "Red" => 1,
                Some(color) if color == "Yellow" => 2,
                _ => 0,
            }).collect()
        }).collect()
    }

    fn place_piece(&mut self, col_idx: usize) -> bool {
        if self.game_over {
            return false;
        }

        // Place the human player's piece
        if !self.place_piece_in_column(col_idx) {
            return false;
        }

        // Check for a winner or if the board is full
        self.check_winner();
        if self.game_over {
            return true;
        }

        // Trigger AI move if the game isn't over
        self.trigger_ai_move()
    }

    fn trigger_ai_move(&mut self) -> bool {
        let ai_col_idx = self.decide_ai_move(); 
        if self.place_piece_in_column(ai_col_idx) {
            self.check_winner();
            true
        } else {
            false
        }
    }

    fn decide_ai_move(&self) -> usize {
        let mut ai_difficulty = false;
        if (self.difficulty == "hard"){
            ai_difficulty = true;
            web_sys::console::log_1(&"hard now".into());                
        }
        let board = self.board_to_ai_format();
        let col = next_move(ai_difficulty, board) as usize;
        col
        // (0..self.custom_cols as usize).find(|&col| self.board[col].iter().any(Option::is_none)).unwrap_or(0)

    }

    fn place_piece_in_column(&mut self, col_idx: usize) -> bool {
        for row in self.board[col_idx].iter_mut() {
            if row.is_none() {
                *row = Some(match self.current_player {
                    Player::Red => "Red".to_string(),
                    Player::Yellow => "Yellow".to_string(),
                });
                self.toggle_player();
                return true;
            }
        }
        false
    }

    fn toggle_player(&mut self) {
        self.current_player = match self.current_player {
            Player::Red => Player::Yellow,
            Player::Yellow => Player::Red,
        };
    }

    fn check_winner(&mut self) {
        let mut winner_found = false;
        // Check horizontal lines
        for row in 0..self.custom_rows {
            for col in 0..=(self.custom_cols - 4) {
                if self.check_line((col, row), (1, 0), 4) {
                    winner_found = true;
                    break;
                }
            }
            if winner_found {
                break;
            }
        }
    
        // Check vertical lines
        if !winner_found {
            for col in 0..self.custom_cols {
                for row in 0..=(self.custom_rows - 4) {
                    if self.check_line((col, row), (0, 1), 4) {
                        winner_found = true;
                        break;
                    }
                }
                if winner_found {
                    break;
                }
            }
        }
    
        // Check diagonal lines (downward right and upward right)
        if !winner_found {
            for col in 0..=(self.custom_cols - 4) {
                for row in 0..=(self.custom_rows - 4) {
                    if self.check_line((col, row), (1, 1), 4) {
                        winner_found = true;
                        break;
                    }
                }
                for row in 3..self.custom_rows {
                    if self.check_line((col, row), (1, -1), 4) {
                        winner_found = true;
                        break;
                    }
                }
                if winner_found {
                    break;
                }
            }
        }
    
        if winner_found {
            self.game_over = true;
            console::log_1(&"Winner detected!".into());
        }
    }
    
    fn check_line(&self, start: (i32, i32), direction: (i32, i32), length: i32) -> bool {
        let mut x = start.0;
        let mut y = start.1;
        let mut last_color = None;
    
        for _ in 0..length {
            if x >= self.custom_cols || y >= self.custom_rows || y < 0 {
                return false;
            }
    
            let current_color = &self.board[x as usize][y as usize];
            if let Some(color) = current_color {
                if last_color == Some(color) || last_color.is_none() {
                    last_color = Some(color);
                } else {
                    return false;
                }
            } else {
                return false;
            }
    
            x += direction.0;
            y += direction.1;
        }
    
        last_color.is_some()
    }
    
    
    fn view_board(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div id="board" style="background-color: #01befe; padding: 10px; display: flex; justify-content: center; border-radius: 10px;">
                <div style="display: grid; grid-template-columns: repeat(7, 1fr); grid-gap: 10px;">
                    { for self.board.iter().enumerate().map(|(col_idx, col)| self.view_column(ctx,col_idx, col)) }
                </div>
            </div>
        }
    }

    fn view_column(&self, ctx: &Context<Self>, col_idx: usize, column: &Vec<Option<String>>) -> Html {
        html! {
            <div key={col_idx} style="display: flex; flex-direction: column-reverse; justify-content: start;">
                { for column.iter().enumerate().map(|(row_idx, cell)| self.view_cell(ctx, col_idx, row_idx, cell)) }
            </div>
        }
    }

    fn view_cell(&self, ctx: &Context<Self>, col_idx: usize, row_idx: usize, cell: &Option<String>) -> Html {
        let cell_color = cell.as_ref().map_or("#FFFFFF", |player| {
            if player == "Red" {
                "#FF0000"
            } else {
                "#FFFF00"
            }
        });
    
        html! {
            <div style={format!("width: 50px; height: 50px; border-radius: 50%; background-color: {}; border: 1px solid #cccccc;", cell_color)}
                 onclick={ctx.link().callback(move |_| Msg::PlacePiece(col_idx))}>
            </div>
        }
    }
    
}

fn main() {
    yew::Renderer::<Model>::new().render();
}