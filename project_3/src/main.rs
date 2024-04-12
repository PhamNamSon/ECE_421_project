use yew::prelude::*;
use web_sys::HtmlInputElement;

enum Msg {
    StartGame,
    UpdateName(String),
    UpdateDifficulty(String),
    UpdateColorMode(String),
    UpdateBoardSize(String),
    UpdateCustomCols(i32),
    UpdateCustomRows(i32),
}

struct Model {
    name: String,
    difficulty: String,
    color_mode: String,
    board_size: String,
    custom_cols: i32,
    custom_rows: i32,
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
        }
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::StartGame => {
                // Here, you would start the game with the selected options.
                // This could involve setting up the game state and navigating to the game view.
                true
            }
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
                                <div id="customSizeInputs" style="display: none; margin-left: 10px;">
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
                        <button id="startGameButton" onclick={ctx.link().callback(|_| Msg::StartGame)}>{"Start Game"}</button>
                    </div>
                </div>
            </div>
        }
    }
}

fn main() {
    yew::Renderer::<Model>::new().render();
}