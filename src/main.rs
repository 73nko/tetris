use gloo::{
    console::{self, log},
    timers::callback::Interval,
};
use web_sys::{HtmlDivElement, KeyboardEvent};
use yew::{
    function_component, html, use_effect, use_node_ref, use_state_eq, Callback, Html, Properties,
};

use crate::shape::{Pos, Shape};
use tetris::{Direction, Tetris};

mod shape;
mod tetris;

#[derive(Properties, PartialEq)]
pub struct BoardProps {
    positions: Vec<String>,
    width: i32,
    height: i32,
    handle_key_down: Callback<KeyboardEvent>,
    handle_key_up: Callback<KeyboardEvent>,
}

#[function_component(Board)]
pub fn board(props: &BoardProps) -> Html {
    let board_ref = use_node_ref();
    let positions = props.positions.clone();

    use_effect({
        let board_ref = board_ref.clone();
        move || {
            if let Some(div) = board_ref.cast::<HtmlDivElement>() {
                div.focus();
            } else {
                console::error!("No div element");
            }

            || ()
        }
    });

    let style = format!(
        "grid-template: repeat({}, 1em) / repeat({}, 1em)",
        props.height, props.width
    );

    html! {
    <div
        tabindex="0"
        class="board"
        {style}
        onkeydown={&props.handle_key_down}
        onkeyup={&props.handle_key_up}
        ref={board_ref}
    >
    { positions.iter().map(move |pos| {
        html! {
            <div class="field">{ pos }</div>
        }
    }).collect::<Html>() }
    </div>
    }
}

#[function_component(TetrisGame)]
fn tetris_game() -> Html {
    let game = use_state_eq(|| Tetris::new(10, 30));
    let speed = use_state_eq(|| 500);

    let handle_key_down = {
        let speed = speed.clone();
        let game = game.clone();
        Callback::from(move |event: KeyboardEvent| {
            let code = event.code();
            let mut tetris = (*game).clone();

            let direction = match &*code {
                "ArrowLeft" => Some(Direction::Left),
                "ArrowRight" => Some(Direction::Right),
                _ => None,
            };

            if let Some(direction) = direction {
                tetris.shift(direction);
            }

            if code == "ArrowUp" {
                tetris.rotate();
            } else if code == "ArrowDown" {
                speed.set(50_i32);
            }
            game.set(tetris.clone())
        })
    };

    let handle_key_up = {
        let speed = speed.clone();
        Callback::from(move |event: KeyboardEvent| {
            let code = event.code();
            if code == "ArrowDown" {
                speed.set(500_i32)
            }
        })
    };

    let positions = {
        let game = game.clone();

        game.iter_positions()
            .map(move |pos| {
                let typ = game.get(pos).unwrap_or("");

                typ.to_string()
            })
            .collect::<Vec<_>>()
    };

    let (width, height) = {
        let game_cloned = game.clone();
        (game_cloned.width, game_cloned.height)
    };

    use_effect({
        let game = game.clone();
        move || {
            let mut tetris = (*game).clone();
            let timer = Interval::new(*speed as u32, move || {
                let n_game = tetris.tick();
                game.set(n_game);
            });

            || drop(timer)
        }
    });

    html! {
        <div class="container">
            <h1>{"Tetris"}</h1>
            <Board
                {positions}
                {width}
                {height}
                {handle_key_down}
                {handle_key_up}
            />
        </div>
    }
}

fn main() {
    yew::start_app::<TetrisGame>();
}
