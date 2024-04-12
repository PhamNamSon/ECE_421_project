use yew::prelude::*;
mod text_input;
use text_input::{TextInput, ButtonInput};
use wasm_bindgen::{JsValue};
use rand::prelude::*;

struct TootComputerController {
    name_input: String,
    game_started: bool,
    player_name: String, 
    disc_type: char,
    matrix: [[char; 6]; 4],
    dummy_matrix: [[i32; 6]; 4],
    non_full_col: Vec<usize>,
    won: bool, 
    turn: u32, 
    winner_name: Option<String>,
    ai_move_value: i32,
}

impl Default for TootComputerController {
    fn default() -> Self {
        Self {
            name_input: String::new(),
            player_name: String::new(),   
            game_started: false,  
            disc_type: '_',
            matrix: [[' '; 6]; 4],
            dummy_matrix: [[0; 6]; 4],
            non_full_col: vec![0,1,2,3,4,5],
            won: false, 
            turn: 0, 
            winner_name: None,
            ai_move_value: 1
        }
    }
}

enum Msg {
    UpdatePlayerName(String),
    GameStart,
    PassValue(String),
    UpdateDiscType(char),
}

impl Component for TootComputerController {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self::default()
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdatePlayerName(name) => {
                self.name_input = name;

                true // Update the component state
            }
            Msg::GameStart => {
                self.player_name = self.name_input.clone();
                self.game_started = true;
                true // Update the component state
            }
            Msg::PassValue(id) => {
                //self.selected = id;
                let id_int = id.parse::<usize>().unwrap();
                let mut full_col_pressed = true;
                for i in (0..=3).rev() {
                    if self.matrix[i][id_int] == ' ' && self.disc_type != '_' {
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

                self.ai_move_hard(&mut full_col_pressed);

                if self.win_check() {
                    web_sys::console::log_1(&JsValue::from_str("won"));
                    return true;
                }

                true
            }
            Msg::UpdateDiscType(dtype) => {
                self.disc_type = dtype;
                true
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
        let on_change: Callback<_> = ctx.link().callback(Msg::UpdatePlayerName);
        let onclick: Callback<_> = ctx.link().callback(|_| Msg::GameStart);
        html! {
            <div id="main">
                <div class="w3-container" id="services" style="margin-top:75px">
                    <h5 class="w3-xxxlarge w3-text-red">{"Enter Your Name"}</h5>
                    <hr style="width:50px;border:5px solid red" class="w3-round" />
                </div>
                <div class="col-md-offset-4 col-md-8">
                    <div class="col-md-offset-3 col-md-8">
                        <TextInput {on_change} value={self.name_input.clone()} />
                        <input {onclick} id="startbutton" class="button" type="submit" value="Start Game" />
                    </div>
                    <canvas id="gameboard" height="480" width="640"></canvas>
                </div>
            </div>
        }
    }
    fn render_game(&self, ctx: &Context<Self>) -> Html {
        let on_change: Callback<_> = ctx.link().callback(Msg::PassValue);
        let on_change_disc: Callback<_> = ctx.link().callback(Msg::UpdateDiscType);
        let on_change_disc_clone = on_change_disc.clone();

        let on_change_disc_t = Callback::from(move |_| {
            on_change_disc.emit('T');
        });
        
        let on_change_disc_o = Callback::from(move |_| {
            on_change_disc_clone.emit('O');
        });
        
        html! {
            <div class="post">
                <br />
                <h4>{"New Game: "} {self.player_name.clone()} {" Vs Player2Name"}</h4>
                <small>{"(Winning Combination: "}{self.player_name.clone()}{" - TOOT and Player2Name - OTTO)"}</small>
                <br />
                <form>
                    <h4>{"Select a Disc Type:"}
                        <input type="radio" name="choice" value="T" onclick={on_change_disc_t}/>
                        <label>{"T"}</label>
                        <input type="radio" name="choice" value="O" onclick={on_change_disc_o}/>
                        <label>{"O"}</label>
                    </h4>
                    <style>
                        {"
                            .circle {
                                width: 40px;
                                height: 40px;
                                border-radius: 50%;
                                background-color: #ccc;
                            "}
                    </style>
                    <table>
                        <tbody>
                            <tr>
                                { for (0..6).map(|i| {
                                    html! {
                                        <th><ButtonInput id={i.to_string()} value={i.to_string()} on_click={on_change.clone()}/></th>
                                    }
                                })}
                            </tr>
                            { for (0..4).map(|row| {
                                html! {
                                    <tr>
                                        { for (0..6).map(|col| {
                                            html! {
                                                <td><div class="circle">{self.matrix[row][col]}</div></td>
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
        
        for i in 0..4 {
            for j in 0..6 {
                temp_r1 = [' '; 4];
                temp_b1 = [' '; 4];
                temp_br1 = [' '; 4];
                temp_br2 = [' '; 4];
                
                for k in 0..=3 {
                    // from (i,j) to right
                    if j + k < 6 {
                        temp_r1[k] = self.matrix[i][j + k];
                    }
                    // from (i,j) to bottom
                    if i + k < 4 && j < 6 {
                        temp_b1[k] = self.matrix[i + k][j];
                    }
                    // from (i,j) to bottom-right
                    if i + k < 4 && j + k < 6 {
                        temp_br1[k] = self.matrix[i + k][j + k];
                    }
                    // from (i,j) to top-right
                    if i as i32 - k as i32 >= 0 && j + k < 6 {
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
                    <button style="padding: 10px 20px; background-color: #007bff; color: #fff; border: none; border-radius: 4px; cursor: pointer;">
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
        for i in (0..=3).rev() {
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
        for i in (0..=3).rev() {
            if self.matrix[i][ai_move.1 as usize] == ' ' && self.disc_type != '_' && !*full_col_pressed {
                self.matrix[i][ai_move.1 as usize] = if self.ai_move_value > 0 {'T'} else {'O'};
                self.dummy_matrix[i][ai_move.1 as usize] = if self.ai_move_value > 0 {1} else {-1};
                *full_col_pressed = true;
                break;
            }
        }

    }

    fn check_state(state: &[[i32; 6]; 4]) -> (i32, i32) {
        let mut win_val = 0;
        let mut chain_val = 0;
    
        for i in 0..4 {
            for j in 0..6 {
                let mut temp_r: i32 = 0;
                let mut temp_b: i32 = 0;
                let mut temp_br: i32 = 0;
                let mut temp_tr: i32 = 0;
    
                for k in 0..=3 {
                    // From (i,j) to right
                    if j + k < 6 {
                        temp_r += state[i][j + k] as i32;
                    }
    
                    // From (i,j) to bottom
                    if i + k < 4 {
                        temp_b += state[i + k][j] as i32;
                    }
    
                    // From (i,j) to bottom-right
                    if i + k < 4 && j + k < 6 {
                        temp_br += state[i + k][j + k] as i32;
                    }
    
                    // From (i,j) to top-right
                    if i as i32 - k as i32 >= 0 && j + k < 5 {
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

    pub fn fill_map(state: &[[i32; 6]; 4], column: usize, value: i32) -> Result<[[i32; 6]; 4], i32> {
        if state[0][column] != 0 || column > 5 {
            return Err(-1);
        }
    
        let mut temp_map = *state;
    
        let mut row = 0;
        let mut done = false;
        for i in 0..3 {
            if temp_map[i + 1][column] != 0 {
                done = true;
                row = i;
                break;
            }
        }
        if !done {
            row = 3;
        }
    
        temp_map[row][column] = value;
        Ok(temp_map)
    }

    fn choose(choice: &Vec<i32>) -> i32 {
        let index = rand::random::<usize>() % choice.len();
        choice[index]
    }

    fn max_state(&self, state: &[[i32; 6]; 4], depth: i32, alpha: i32, beta: i32) -> (i32, i32) {
        let mut v: i32 = i32::MIN;
        let mut move_col: i32 = -1;
        let mut temp_val: (i32, i32) = (-1, -1); // Placeholder value
        let mut move_queue: Vec<i32> = vec![];
        let mut new_alpha: i32 = alpha;
    
        for j in 0..6 {
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

    fn min_state(&self, state: &[[i32; 6]; 4], depth: i32, alpha: i32, mut beta: i32) -> (i32, i32) {
        let mut v: i32 = i32::MAX;
        let mut move_col: i32 = -1;
        let mut temp_val: (i32, i32) = (-1, -1); // Placeholder value
        let mut move_queue: Vec<i32> = vec![];
        let mut new_beta: i32 = beta;
    
        for j in 0..6 {
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

    fn value(&self, state: &[[i32; 6]; 4], depth: i32, alpha: i32, beta: i32) -> (i32, i32) {
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


fn main() {
    yew::Renderer::<TootComputerController>::new().render();
}
