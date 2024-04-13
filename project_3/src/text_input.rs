use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::{Event, HtmlInputElement, InputEvent};
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct TextProps {
    pub value: String,
    pub on_change: Callback<String>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct ButtonProps {
    pub value: String,
    pub id: String,
    pub on_click: Callback<String>,
}

fn get_value_from_input_event(e: InputEvent) -> String {
    let event: Event = e.dyn_into().unwrap_throw();
    let event_target = event.target().unwrap_throw();
    let target: HtmlInputElement = event_target.dyn_into().unwrap_throw();
    web_sys::console::log_1(&target.value().into());
    target.value()
}

/// Controlled Text Input Component
#[function_component(TextInput)]
pub fn text_input(props: &TextProps) -> Html {
    let TextProps { value, on_change } = props.clone();

    let oninput = Callback::from(move |input_event: InputEvent| {
        on_change.emit(get_value_from_input_event(input_event));
    });

    html! {
        <input type="text" {value} {oninput} />
    }
}

#[function_component(ButtonInput)]
pub fn button_input(props: &ButtonProps) -> Html {
    let ButtonProps { value, id, on_click } = props.clone();

    let oninput = Callback::from(move |_| {
        on_click.emit(id.clone());
    });

    html! {
        <input type="button" {value} onclick={oninput} />
    }
}
