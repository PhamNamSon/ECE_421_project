use std::fmt::format;
use yew::prelude::*;
use web_sys::HtmlInputElement;
mod connect_4_computer;
use web_sys::console;
use connect_4_computer::next_move;
mod text_input;
use text_input::{TextInput, ButtonInput};
use wasm_bindgen::{JsValue};
use rand::prelude::*;






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
struct ConnectFour {
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

impl Component for ConnectFour {
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
                if self.board_size == "custom"{
                    self.board = vec![vec![None; self.custom_rows as usize]; self.custom_cols as usize];
                } else {
                    self.board = vec![vec![None; 6]; 7];
                }
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
            if self.game_over {
                html! {
                    <>
                        { self.view_board(ctx) }
                        <div class="winner-announcement">
                            { self.winner_message() }
                        </div>
                        <button class="button" onclick={ctx.link().callback(|_| Msg::RestartGame)}>{"Play Again"}</button>
                        <button class="button" onclick={ctx.link().callback(|_| Msg::GoToHome)}>{"Home Screen"}</button>
                    </>
                }
            } else {
                self.view_board(ctx)
            }
        } else {
            html! {
                <div id="main-content">
                    <div id="main">
                        <div class="input-group">
                            <h5 class="input-label">{"Enter Your Name"}</h5>
                            <input class="input-field" id="playerNameInput" type="text" placeholder="Your Name" value={self.name.clone()} oninput={ctx.link().callback(|e: InputEvent| {
                                let input: HtmlInputElement = e.target_unchecked_into();
                                Msg::UpdateName(input.value())
                            })} />
                        </div>
                        <div class="radio-group">
                            <label class="radio-label">
                                <input type="radio" name="difficulty" value="easy" checked={self.difficulty == "easy"} onchange={ctx.link().callback(|_| Msg::UpdateDifficulty("easy".to_string()))} />
                                {" Easy"}
                            </label>
                            <label class="radio-label">
                                <input type="radio" name="difficulty" value="hard" checked={self.difficulty == "hard"} onchange={ctx.link().callback(|_| Msg::UpdateDifficulty("hard".to_string()))} />
                                {" Hard"}
                            </label>
                        </div>
                        <div class="radio-group">
                            <label class="radio-label">
                                <input type="radio" name="colorMode" value="normal" checked={self.color_mode == "normal"} onchange={ctx.link().callback(|_| Msg::UpdateColorMode("normal".to_string()))} />
                                {" Normal"}
                            </label>
                            <label class="radio-label">
                                <input type="radio" name="colorMode" value="colorBlind1" checked={self.color_mode == "colorBlind1"} onchange={ctx.link().callback(|_| Msg::UpdateColorMode("colorBlind1".to_string()))} />
                                {" Color Blind Set 1"}
                            </label>
                            <label class="radio-label">
                                <input type="radio" name="colorMode" value="colorBlind2" checked={self.color_mode == "colorBlind2"} onchange={ctx.link().callback(|_| Msg::UpdateColorMode("colorBlind2".to_string()))} />
                                {" Color Blind Set 2"}
                            </label>
                        </div>
                        <div class="radio-group">
                            <label class="radio-label">
                                <input type="radio" name="boardSize" value="standard" checked={self.board_size == "standard"} onchange={ctx.link().callback(|_| Msg::UpdateBoardSize("standard".to_string()))} />
                                {" Standard (7 cols x 6 rows)"}
                            </label>
                            <label style="display: flex; align-items: center;" class="radio-label">
                                <input type="radio" name="boardSize" value="custom" checked={self.board_size == "custom"} onchange={ctx.link().callback(|_| Msg::UpdateBoardSize("custom".to_string()))} />
                                {" Custom"}
                                <div class="custom-size-inputs" style={format!("display: {};", if self.board_size == "custom" { "flex" } else { "none" })}>
                                    <input class="custom-size-input" id="customCols" type="number" placeholder="Cols" min="4" max="10" value={self.custom_cols.to_string()} oninput={ctx.link().callback(|e: InputEvent| {
                                        let input: HtmlInputElement = e.target_unchecked_into();
                                        Msg::UpdateCustomCols(input.value_as_number() as i32)
                                    })} />
                                    <input class="custom-size-input" id="customRows" type="number" placeholder="Rows" min="4" max="10" value={self.custom_rows.to_string()} oninput={ctx.link().callback(|e: InputEvent| {
                                        let input: HtmlInputElement = e.target_unchecked_into();
                                        Msg::UpdateCustomRows(input.value_as_number() as i32)
                                    })} />
                                </div>
                            </label>
                        </div>
                        <button class="button" id="startGameButton" onclick={ctx.link().callback(|_| {
                            web_sys::console::log_1(&"Start Game button clicked".into());
                            Msg::StartGame
                        })}>
                            {"Start Game"}
                        </button>
                    </div>
                </div>
            }
        }
    }
    
}

impl ConnectFour {
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
            _=> "It's a draw".to_string(),
        }
    }

    fn is_board_full(&self) -> bool {
        self.board.iter().all(|col| col.iter().all(|cell| cell.is_some()))
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
        if self.difficulty == "hard" {
            ai_difficulty = true;
            web_sys::console::log_1(&"hard now".into());                
        }
        let board = self.board_to_ai_format();
        let col = next_move(ai_difficulty, board) as usize;
        col

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
        } else if self.is_board_full() {
            self.game_over = true;
            console::log_1(&"The game is a draw!".into());
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
        let grid_style = format!(
            "display: grid; grid-template-columns: repeat({}, 1fr); grid-gap: 10px;",
            self.custom_cols
        );
        let mut board_color = "#01befe";
        if self.color_mode == "colorBlind2"{
            board_color = "#800080"
        }
    
        html! {
            <div id="board" style={format!("background-color: {}; padding: 10px; display: flex; justify-content: center; border-radius: 10px;", board_color)}>
                <div style="display: grid; grid-template-columns: repeat(7, 1fr); grid-gap: 10px;">
                    { for self.board.iter().enumerate().map(|(col_idx, col)| self.view_column(ctx, col_idx, col)) }
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
        let mut human_color = "#FF0000";
        let mut ai_color = "#FFFF00";
            if self.color_mode == "colorBlind1"{
                human_color = "#Fd6900";
                ai_color =  "#d3d3d3";
            } else if self.color_mode == "colorBlind2"{
                human_color = "#008080";
                ai_color =  "#FFFF00";
            }
            if player == "Red" {
                human_color
            } else {
                ai_color
            }
        });
    
        html! {
            <div style={format!("width: 50px; height: 50px; border-radius: 50%; background-color: {}; border: 1px solid #cccccc;", cell_color)}
                 onclick={ctx.link().callback(move |_| Msg::PlacePiece(col_idx))}>
            </div>
        }
    }
}

struct TootComputerController {
    name_input: String,
    game_started: bool,
    player_name: String, 
    disc_type: char,
    matrix: Vec<Vec<char>>,
    dummy_matrix: Vec<Vec<i32>>,
    non_full_col: Vec<usize>,
    won: bool, 
    turn: u32, 
    winner_name: Option<String>,
    ai_move_value: i32,
    difficulty: String,
    Columns: usize, 
    Rows: usize
}

impl Default for TootComputerController {
    fn default() -> Self {
        Self {
            name_input: String::new(),
            player_name: String::new(),   
            game_started: false,  
            disc_type: 'T',
            matrix: Vec::new(),
            dummy_matrix: Vec::new(),
            non_full_col: vec![0,1,2,3,4,5],
            won: false, 
            turn: 0, 
            winner_name: None,
            ai_move_value: 1,
            difficulty: String::from("Easy"),
            Columns: 6, 
            Rows: 4
        }
    }
}

enum OttOMsg {
    UpdatePlayerName(String),
    GameStart,
    PassValue(String),
    UpdateDiscType(char),
    ResetGame, 
    SetDifficulty(String),
    SetColumns(String), 
    SetRows(String)
    
}

impl Component for TootComputerController {
    type Message = OttOMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self::default()
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            OttOMsg::UpdatePlayerName(name) => {
                self.name_input = name;

                true // Update the component state
            }
            OttOMsg::GameStart => {
                self.player_name = self.name_input.clone();
                self.game_started = true;
                // Populate the matrix with empty vectors
                for _ in 0..self.Rows {
                    let row: Vec<char> = vec![' '; self.Columns as usize];
                    let dummy_row: Vec<i32> = vec![0; self.Columns as usize];
                    (self.matrix).push(row);
                    (self.dummy_matrix).push(dummy_row);
                }

                if !(self.Rows >= 4 && self.Columns >=4) {
                    return false;
                }
                true // Update the component state
            }
            OttOMsg::PassValue(id) => {
                //self.selected = id;
                let id_int = id.parse::<usize>().unwrap();
                let mut full_col_pressed = true;
                for i in (0..self.Rows).rev() {
                    if self.matrix[i][id_int] == ' ' {
                        self.matrix[i][id_int] = self.disc_type;
                        self.dummy_matrix[i][id_int] = self.disc_type as i32;
                        full_col_pressed = false;
                        break;
                    }
                }
                
                // Check if player won
                if self.win_check() {
                    web_sys::console::log_1(&JsValue::from_str("won"));
                    return true;
                }

                self.ai_move_value *= -1; 

                if self.difficulty == "Easy" {
                    self.ai_move_easy(&mut full_col_pressed);

                } else if self.difficulty == "Hard" {
                    self.ai_move_hard(&mut full_col_pressed);
                }

                

                if self.win_check() {
                    web_sys::console::log_1(&JsValue::from_str("won"));
                    return true;
                }

                true
            }
            OttOMsg::UpdateDiscType(dtype) => {
                self.disc_type = dtype;
                false
            }
            OttOMsg::ResetGame => {
                *self = Self::default();
                true
            }
            OttOMsg::SetDifficulty(difficulty) => {
                self.difficulty = difficulty;
                false
            }
            OttOMsg::SetColumns(col) => {
                self.Columns = col.parse().expect("Failed to parse string to integer");;
                false
            }

            OttOMsg::SetRows(row) => {
                self.Rows = row.parse().expect("Failed to parse string to integer");;
                false
            }
            
        }
    }

    fn changed(&mut self, _ctx: &Context<Self>, _props: &Self::Properties) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {   
        if self.won { 
            return self.render_win_screen(ctx);
        }    
        if !self.game_started {
            return self.menu(ctx);
        } else {
            return self.render_game(ctx);
        }
    }
}

impl TootComputerController {
    fn menu(&self, ctx: &Context<Self>) -> Html { 
        let on_change: Callback<_> = ctx.link().callback(OttOMsg::UpdatePlayerName);
        let onclick: Callback<_> = ctx.link().callback(|_| OttOMsg::GameStart);

        let on_change_diff: Callback<_> = ctx.link().callback(OttOMsg::SetDifficulty);
        let on_change_diff_clone: Callback<_> = on_change_diff.clone();
        let on_change_diff_easy = Callback::from(move |_| {
            on_change_diff.emit(String::from("Easy"));
        });
        let on_change_diff_hard = Callback::from(move |_| {
            on_change_diff_clone.emit(String::from("Hard"));
        });

        let on_change_col = ctx.link().callback(OttOMsg::SetColumns);
        let on_change_row = ctx.link().callback(OttOMsg::SetRows);

        html! {
            <div id="main" style="text-align: center;">
                <div class="w3-container" id="services" style="margin-top: 75px;">
                    <h2 style="color: #FF5733; font-family: Arial, sans-serif;">{"Enter Your Name"}</h2>
                    <hr style="width: 50px; border: 5px solid #FF5733; border-radius: 10px;" />
                </div>
                <div class="col-md-offset-4 col-md-8" style="margin-top: 30px;">
                    <div class="col-md-offset-3 col-md-8">
                        <TextInput {on_change} value={self.name_input.clone()}/>
                        <div style="margin-top: 10px;"></div>
                        <input {onclick} id="startbutton" class="button" type="submit" value="Start Game" style="padding: 10px 20px; background-color: #FF5733; color: white; border: none; border-radius: 5px; cursor: pointer; font-family: Arial, sans-serif; font-size: 16px;" />
                    </div>
                    <h3 style="margin-top: 20px; color: #FF5733; font-family: Arial, sans-serif;">{"Select a Difficulty:"}
                        <div style="display: inline-block; margin-right: 20px;">
                            <input type="radio" name="difficulty" value="Easy" onclick={on_change_diff_easy} checked={self.difficulty == "Easy"} id="easy"/>
                            <label for="easy" style="cursor: pointer; font-family: Arial, sans-serif; font-size: 16px; color: #FF5733; padding: 8px 20px; border: 2px solid #FF5733; border-radius: 5px;">{"Easy"}</label>
                        </div>
                        <div style="display: inline-block;">
                            <input type="radio" name="difficulty" value="Hard" onclick={on_change_diff_hard} checked={self.difficulty == "Hard"} id="hard"/>
                            <label for="hard" style="cursor: pointer; font-family: Arial, sans-serif; font-size: 16px; color: #FF5733; padding: 8px 20px; border: 2px solid #FF5733; border-radius: 5px;">{"Hard"}</label>
                        </div>
                    </h3>
                    <h4 style="margin-top: 20px; color: #FF5733; font-family: Arial, sans-serif;">{"Select a Row and Column:"}
                        <div style="display: inline-block; margin-right: 20px;">
                            <TextInput on_change = {on_change_row} value={(self.Rows.clone()).to_string()}/>
                            {"(Min 4)   "}
                            <TextInput on_change = {on_change_col} value={(self.Columns.clone()).to_string()}/>
                            {"(Min 4)   "}
                        </div>
                    </h4>
                    <canvas id="gameboard" height="480" width="640"></canvas>
                </div>
            </div>
        }
    }
    fn render_game(&self, ctx: &Context<Self>) -> Html {
        let on_change: Callback<_> = ctx.link().callback(OttOMsg::PassValue);
        let on_change_disc: Callback<_> = ctx.link().callback(OttOMsg::UpdateDiscType);
        let on_change_disc_clone = on_change_disc.clone();

        let on_change_disc_t = Callback::from(move |_| {
            on_change_disc.emit('T');
        });
        
        let on_change_disc_o = Callback::from(move |_| {
            on_change_disc_clone.emit('O');
        });
        
        html! {
            <div class="post" style="font-family: 'Arial', sans-serif; margin-top: 20px; text-align: center;">
                <br />
                <h4 style="font-weight: bold; color: #FF5733;">{"New Game: "} {self.player_name.clone()} {" Vs Computer"}</h4>
                <small style="font-size: 12px; color: #777;">{"(Winning Combination: "}{self.player_name.clone()}{" - TOOT and Computer - OTTO)"}</small>
                <br />
                <form>
                    <h4 style="font-weight: bold; margin-top: 20px; color: #FF5733;">{"Select a Disc Type:"}
                        <div style="display: inline-block; margin-right: 20px;">
                            <input type="radio" name="choice" value="T" onclick={on_change_disc_t} checked={self.disc_type == 'T'} id="disc_t"/>
                            <label for="disc_t" style="cursor: pointer; font-size: 16px; color: #FF5733; padding: 8px 20px; border: 2px solid #FF5733; border-radius: 5px; background-color: white;">{"T"}</label>
                        </div>
                        <div style="display: inline-block;">
                            <input type="radio" name="choice" value="O" onclick={on_change_disc_o} checked={self.disc_type == 'O'} id="disc_o"/>
                            <label for="disc_o" style="cursor: pointer; font-size: 16px; color: #FF5733; padding: 8px 20px; border: 2px solid #FF5733; border-radius: 5px; background-color: white;">{"O"}</label>
                        </div>
                    </h4>
                    <style>
                        {"
                            .circle {
                                width: 40px;
                                height: 40px;
                                border-radius: 50%;
                                background-color: #ccc;
                                display: flex;
                                justify-content: center;
                                align-items: center;
                                font-size: 24px;
                                color: #333;
                                border: 2px solid #fff;
                            "}
                    </style>
                    <table style="margin-top: 20px; margin-left: auto; margin-right: auto;">
                        <tbody>
                            <tr>
                                { for (0..self.Columns).map(|i| {
                                    html! {
                                        <th style="padding: 5px;"><ButtonInput id={i.to_string()} value={i.to_string()} on_click={on_change.clone()}/></th>
                                    }
                                })}
                            </tr>
                            { for (0..self.Rows).map(|row| {
                                html! {
                                    <tr>
                                        { for (0..self.Columns).map(|col| {
                                            html! {
                                                <td style="padding: 5px;"><div class="circle">{self.matrix[row][col]}</div></td>
                                            }
                                        })}
                                    </tr>
                                }
                            })}
                        </tbody>
                    </table>
                </form>
            </div>
        }
    }


    fn win_check(&mut self) -> bool {
        let mut temp_r1 = [' '; 4];
        let mut temp_b1 = [' '; 4];
        let mut temp_br1 = [' '; 4];
        let mut temp_br2 = [' '; 4];
        
        for i in 0..self.Rows {
            for j in 0..self.Columns {
                temp_r1 = [' '; 4];
                temp_b1 = [' '; 4];
                temp_br1 = [' '; 4];
                temp_br2 = [' '; 4];
                
                for k in 0..=3 {
                    // from (i,j) to right
                    if j + k < self.Columns {
                        temp_r1[k] = self.matrix[i][j + k];
                    }
                    // from (i,j) to bottom
                    if i + k < self.Rows && j < self.Columns {
                        temp_b1[k] = self.matrix[i + k][j];
                    }
                    // from (i,j) to bottom-right
                    if i + k < self.Rows && j + k < self.Columns {
                        temp_br1[k] = self.matrix[i + k][j + k];
                    }
                    // from (i,j) to top-right
                    if i as i32 - k as i32 >= 0 && j + k < self.Columns {
                        temp_br2[k] = self.matrix[i - k][j + k];
                    }
                }
    
                if temp_r1 == ['T', 'O', 'O', 'T'] {
                    self.win(1);
                    return true;
                } else if temp_r1 == ['O', 'T', 'T', 'O'] {
                    self.win(-1);
                    return true;
                } else if temp_b1 == ['T', 'O', 'O', 'T'] {
                    self.win(1);
                    return true;
                } else if temp_b1 == ['O', 'T', 'T', 'O'] {
                    self.win(-1);
                    return true;
                } else if temp_br1 == ['T', 'O', 'O', 'T'] {
                    self.win(1);
                    return true;
                } else if temp_br1 == ['O', 'T', 'T', 'O'] {
                    self.win(-1);
                    return true;
                } else if temp_br2 == ['T', 'O', 'O', 'T'] {
                    self.win(1);
                    return true;
                } else if temp_br2 == ['O', 'T', 'T', 'O'] {
                    self.win(-1);
                    return true;
                }
            }
        }
        
        // check if draw
        if self.turn == 24 && !self.won {
            self.win(0);
            return true;
        }
    
        return false;
    }

    fn render_win_screen(&self, ctx: &Context<Self>) -> Html {

        let win_statement: Html = match &self.winner_name {
            Some(winner) => {
                html! { 
                    <h1 style="text-align: center; color: #333;">{ format!("{} Wins!", (winner)) }</h1>
                }
            },
            None => {
                html! { 
                    <h1 style="text-align: center; color: #333;">{"The match has ended in a draw!"}</h1>
                }
            }
        };

        html! {
            <div style="background-color: #f0f0f0; padding: 20px; border: 2px solid #ccc; border-radius: 8px;">
                {win_statement}
                <div style="text-align: center;">
                    <button onclick={ctx.link().callback(|_| OttOMsg::ResetGame)} style="padding: 10px 20px; background-color: #007bff; color: #fff; border: none; border-radius: 4px; cursor: pointer;">
                        {"Restart"}
                    </button>
                </div>
            </div>
        }
    }

    fn win(&mut self, player: i32) { 
        //Set game state to over and select winner; 
        self.won = true;
        if player == 1 {
            self.winner_name = Some(self.player_name.clone());
        } else if player == -1 {
            self.winner_name = Some(String::from("Computer"));

        } else if player == 0 { 
            self.winner_name = None; 
        }
    }

    fn ai_move_easy(&mut self, full_col_pressed: &mut bool){
        let mut rng = rand::thread_rng();
        let mut col = self.non_full_col[rng.gen_range(0..self.non_full_col.len())];

        while self.matrix[0][col] != ' ' {
            self.non_full_col.retain(|&x| x != col);
            col = self.non_full_col[rng.gen_range(0..self.non_full_col.len())];
        }

        // Place computer disc
        for i in (0..self.Rows).rev() {
            if self.matrix[i][col] == ' ' && self.disc_type != '_' && !*full_col_pressed {
                self.matrix[i][col] = ['T', 'O'][rng.gen_range(0..=1)];
                *full_col_pressed = true;
                break;
            }
        }
    }

    fn ai_move_hard(&mut self, full_col_pressed: &mut bool){

        let ai_move = self.max_state(&self.dummy_matrix, 0, i32::MIN, i32::MAX);
        web_sys::console::log_1(&JsValue::from_str(&(ai_move.0).to_string()));
        web_sys::console::log_1(&JsValue::from_str(&(ai_move.1).to_string()));

        // Place computer disc
        for i in (0..self.Rows).rev() {
            if self.matrix[i][ai_move.1 as usize] == ' ' && self.disc_type != '_' && !*full_col_pressed {
                self.matrix[i][ai_move.1 as usize] = if self.ai_move_value > 0 {'T'} else {'O'};
                self.dummy_matrix[i][ai_move.1 as usize] = if self.ai_move_value > 0 {1} else {-1};
                *full_col_pressed = true;
                break;
            }
        }

    }

    fn check_state(state: &Vec<Vec<i32>>) -> (i32, i32) {
        let mut win_val = 0;
        let mut chain_val = 0;

        let rows = state.len();
        let columns = state.get(0).map_or(0, |row| row.len());
    
        for i in 0..rows {
            for j in 0..columns {
                let mut temp_r: i32 = 0;
                let mut temp_b: i32 = 0;
                let mut temp_br: i32 = 0;
                let mut temp_tr: i32 = 0;
    
                for k in 0..=3 {
                    // From (i,j) to right
                    if j + k < columns {
                        temp_r += state[i][j + k] as i32;
                    }
    
                    // From (i,j) to bottom
                    if i + k < rows {
                        temp_b += state[i + k][j] as i32;
                    }
    
                    // From (i,j) to bottom-right
                    if i + k < rows && j + k < columns {
                        temp_br += state[i + k][j + k] as i32;
                    }
    
                    // From (i,j) to top-right
                    if i as i32 - k as i32 >= 0 && j + k < columns {
                        temp_tr += state[i - k][j + k] as i32;
                    }
                }
    
                chain_val += temp_r * temp_r * temp_r;
                chain_val += temp_b * temp_b * temp_b;
                chain_val += temp_br * temp_br * temp_br;
                chain_val += temp_tr * temp_tr * temp_tr;
    
                if temp_r.abs() == 4 {
                    win_val = temp_r;
                } else if temp_b.abs() == 4 {
                    win_val = temp_b;
                } else if temp_br.abs() == 4 {
                    win_val = temp_br;
                } else if temp_tr.abs() == 4 {
                    win_val = temp_tr;
                }
            }
        }
    
        (win_val, chain_val)
    }

    pub fn fill_map(state: &Vec<Vec<i32>>, column: usize, value: i32) -> Result<Vec<Vec<i32>>, i32> {
        let rows = state.len();
        let col = state.get(0).map_or(0, |row| row.len());
        if state[0][column] != 0 || column > col-1 {
            return Err(-1);
        }
    
        let mut temp_map = state.clone();
    
        let mut row = 0;
        let mut done = false;
        for i in 0..rows-1 {
            if temp_map[i + 1][column] != 0 {
                done = true;
                row = i;
                break;
            }
        }
        if !done {
            row = rows-1;
        }
    
        temp_map[row][column] = value;
        Ok(temp_map)
    }

    fn choose(choice: &Vec<i32>) -> i32 {
        let index = rand::random::<usize>() % choice.len();
        choice[index]
    }

    fn max_state(&self, state: &Vec<Vec<i32>>, depth: i32, alpha: i32, beta: i32) -> (i32, i32) {
        let mut v: i32 = i32::MIN;
        let mut move_col: i32 = -1;
        let mut temp_val: (i32, i32) = (-1, -1); // Placeholder value
        let mut move_queue: Vec<i32> = vec![];
        let mut new_alpha: i32 = alpha;
    
        for j in 0..self.Columns {
            let temp_state_result = Self::fill_map(&state, j, self.ai_move_value);
            if let Ok(temp_state) = temp_state_result {
                temp_val = self.value(&temp_state, depth, new_alpha, beta);
                if temp_val.0 > v {
                    v = temp_val.0;
                    move_col = j as i32;
                    move_queue = vec![j as i32];
                } else if temp_val.0 == v {
                    move_queue.push(j as i32);
                }
    
                // Alpha-beta pruning
                if v > beta {
                    move_col = Self::choose(&move_queue);
                    return (v, move_col);
                }
                new_alpha = std::cmp::max(new_alpha, v);
            }
        }
        let move_choice = Self::choose(&move_queue);
        (v, move_choice)
    }

    fn min_state(&self, state: &Vec<Vec<i32>>, depth: i32, alpha: i32, mut beta: i32) -> (i32, i32) {
        let mut v: i32 = i32::MAX;
        let mut move_col: i32 = -1;
        let mut temp_val: (i32, i32) = (-1, -1); // Placeholder value
        let mut move_queue: Vec<i32> = vec![];
        let mut new_beta: i32 = beta;
    
        for j in 0..self.Columns {
            if let Ok(temp_state) = Self::fill_map(&state, j, self.ai_move_value * -1) {
                temp_val = self.value(&temp_state, depth, alpha, new_beta);
                if temp_val.0 < v {
                    v = temp_val.0;
                    move_col = j as i32;
                    move_queue = vec![j as i32];
                } else if temp_val.0 == v {
                    move_queue.push(j as i32);
                }
    
                // Alpha-beta pruning
                if v < alpha {
                    move_col = Self::choose(&move_queue);
                    return (v, move_col);
                }
                new_beta = new_beta.min(v);
            }
        }
        let move_choice = Self::choose(&move_queue);
        (v, move_choice)
    }

    fn value(&self, state: &Vec<Vec<i32>>, depth: i32, alpha: i32, beta: i32) -> (i32, i32) {
        let val = Self::check_state(state);
        if depth >= 4 {
            let mut ret_value = 0;
    
            let win_val = val.0;
            let chain_val = val.1 * self.ai_move_value;
            ret_value = chain_val;
    
            if win_val == 4 * self.ai_move_value {
                ret_value = 999999;
            } else if win_val == 4 * self.ai_move_value * -1 {
                ret_value = -999999;
            }
            ret_value -= depth * depth;
    
            return (ret_value, -1);
        }
    
        let win = val.0;
        if win == 4 * self.ai_move_value {
            return (999999 - depth * depth, -1);
        } else if win == 4 * self.ai_move_value * -1 {
            return (-999999 - depth * depth, -1);
        }
    
        if depth % 2 == 0 {
            return self.min_state(state, depth + 1, alpha, beta);
        }
        self.max_state(state, depth + 1, alpha, beta)
    }
}

enum ActiveGame {
    None,
    ConnectFour,
    TootComputerController,
}

struct Root {
    active_game: ActiveGame,
}

enum Msgg {
    SelectConnectFour,
    TootComputerController,
    GoToHome,
}

impl Component for Root {
    type Message = Msgg;
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self {
            active_game: ActiveGame::None,
        }
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msgg::SelectConnectFour => {
                self.active_game = ActiveGame::ConnectFour;
                true
            },
            Msgg::TootComputerController => {
                self.active_game = ActiveGame::TootComputerController;
                true
            },
            Msgg::GoToHome => {
                self.active_game = ActiveGame::None;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        match self.active_game {
            ActiveGame::ConnectFour => html! {
                <div style="display: flex; justify-content: center; align-items: center; height: 100vh; flex-direction: column;">
                    <button onclick={ctx.link().callback(|_| Msgg::GoToHome)} style="padding: 10px; background-color: #4CAF50; color: white; margin-bottom: 20px; width: 150px;">{"Back to Home"}</button>
                    <div style="padding: 20px; border: 2px solid #ddd; border-radius: 5px; text-align: center;">
                        <h2>{"Connect Four"}</h2>
                        <p>{"A classic two-player connection game where players first choose a color and then take turns dropping colored discs into a seven-column, six-row vertically suspended grid. The pieces fall straight down, occupying the lowest available space within the column. The objective of the game is to be the first to form a horizontal, vertical, or diagonal line of four of one's own discs."}</p>
                    </div>
                    <ConnectFour />
                </div>
            },
            ActiveGame::TootComputerController => html! {
                <div style="display: flex; justify-content: center; align-items: center; height: 100vh; flex-direction: column;">
                    <button onclick={ctx.link().callback(|_| Msgg::GoToHome)} style="padding: 10px; background-color: #4CAF50; color: white; margin-bottom: 20px; width: 150px;">{"Back to Home"}</button>
                    <div style="padding: 20px; border: 2px solid #ddd; border-radius: 5px; text-align: center;">
                        <h2>{"TOOT and OTTO"}</h2>
                        <p>{"TOOT and OTTO is a two-player strategic game where players choose to be either TOOT or OTTO. Each player aims to create the words TOOT or OTTO in a straight line from their letters on a grid, either horizontally, vertically, or diagonally. Players must strategically place their letters while blocking their opponent."}</p>
                    </div>
                    <TootComputerController />
                </div>
            },
            ActiveGame::None => html! {
                <div>
                    <button onclick={ctx.link().callback(|_| Msgg::SelectConnectFour)}>{"Play Connect Four"}</button>
                    <div id="main-connect-four">
                        <h5 class="w3-xxxlarge w3-text-red"><b>{"How to Play Connect 4"}</b></h5>
                        <p>{"Connect Four is a two-player connection game in which the players take turns dropping colored discs from the top into a seven-column, six-row vertically"}</p>
                        <p>{"suspended grid. The objective of the game is to be the first to form a horizontal, vertical, or diagonal line of four of one's own discs."}</p>
                        <br/>
                        <div><h5>{"To play Connect 4 follow the following steps:"}</h5></div>
                        <ul>
                            <li>{"A new game describes discs of which color belongs to which player"}</li>
                            <li>{"Click on the desired column on the game board to place your disc"}</li>
                            <li>{"Try to connect 4 of your colored discs either horizontally, vertically or diagonally"}</li>
                        </ul>
                        <br/>{" For More information on Connect 4 click "}<a href="https://en.wikipedia.org/wiki/Connect_Four">{"here"}</a>
                    </div>
                    <br/>
                    <button onclick={ctx.link().callback(|_| Msgg::TootComputerController)}>{"TOOT and OTTO"}</button>
                    <div id="main-toot-otto">
                        <h5 class="w3-xxxlarge w3-text-red"><b>{"How to Play TOOT-OTTO"}</b></h5>
                        <p>{"TOOT-OTTO is a fun strategy game for older players who like tic-tac-toe and checkers. One player is TOOT and the other player is OTTO. Both players"}</p>
                        <p>{"can place both T's and O's, based on their choice. The first player who spells his or her winning combination - horizontally, vertically or diagonally - wins!"}</p>
                        <br/>
                        <div><h5>{"To play TOOT-OTTO follow the following steps:"}</h5></div>
                        <ul>
                            <li>{"A new game describes which player is TOOT and which is OTTO"}</li>
                            <li>{"Select the disc type T or O that you want to place"}</li>
                            <li>{"Click on the desired column on the game board to place your disc"}</li>
                            <li>{"Try to spell TOOT or OTTO based on your winning combination, either horizontally, vertically or diagonally"}</li>
                        </ul>
                        <br/> {"For More information on TOOT-OTTO click "}<a href="https://boardgamegeek.com/boardgame/19530/toot-and-otto">{"here"}</a>
                    </div>
                </div>
            },
        }
    }
    
    
    
}


fn main() {
    yew::Renderer::<Root>::new().render();
}