use js_sys::{Function, Reflect};
use tetris::{Direction, Tetris};
use wasm_bindgen::{prelude::Closure, JsCast, JsValue, UnwrapThrowExt};
use wasm_react::{
    c, export_components, h,
    hooks::{use_callback, use_effect, use_js_ref, use_state, Deps},
    props::Style,
    Component,
};
use web_sys::{window, Element, HtmlElement, KeyboardEvent};

mod shape;
mod tetris;

pub struct App {
    width: i32,
    height: i32,
}

impl TryFrom<JsValue> for App {
    type Error = JsValue;
    fn try_from(val: JsValue) -> Result<Self, Self::Error> {
        Ok(Self {
            width: Reflect::get(&val, &"width".into())?
                .as_f64()
                .unwrap_or(10.0) as i32,
            height: Reflect::get(&val, &"height".into())?
                .as_f64()
                .unwrap_or(30.0) as i32,
        })
    }
}

impl Component for App {
    fn render(&self) -> wasm_react::VNode {
        let tetris = use_state(|| Tetris::new(self.width, self.height));
        let speed = use_state(|| 500);
        let element_container = use_js_ref::<Element>(None);

        use_effect(
            {
                let element_container = element_container.clone();
                move || {
                    element_container
                        .current()
                        .and_then(|element| element.dyn_into::<HtmlElement>().ok())
                        .map(|el| el.focus());

                    || ()
                }
            },
            Deps::none(),
        );

        use_effect(
            {
                let tetris = tetris.clone();
                let speed = *speed.value();
                move || {
                    let tick_closure = Closure::new({
                        let mut tetris = tetris;

                        move || {
                            tetris.set(|mut tetris| {
                                tetris.tick();
                                tetris
                            })
                        }
                    });

                    let handle = window()
                        .unwrap_throw()
                        .set_interval_with_callback_and_timeout_and_arguments_0(
                            tick_closure.as_ref().dyn_ref::<Function>().unwrap_throw(),
                            speed,
                        )
                        .unwrap_throw();

                    move || {
                        drop(tick_closure);
                        window().unwrap_throw().clear_interval_with_handle(handle)
                    }
                }
            },
            Deps::some(*speed.value()),
        );

        let handle_key_down = use_callback(
            {
                let mut tetris = tetris.clone();
                let mut speed = speed.clone();
                move |evt: KeyboardEvent| {
                    let code = evt.code();
                    let direction = match &*code {
                        "ArrowLeft" => Some(Direction::Left),
                        "ArrowRight" => Some(Direction::Right),
                        _ => None,
                    };

                    if let Some(direction) = direction {
                        tetris.set(|mut tetris| {
                            tetris.shift(direction);
                            tetris
                        });
                    }

                    if code == "ArrowUp" {
                        tetris.set(|mut tetris| {
                            tetris.rotate();
                            tetris
                        });
                    } else if code == "ArrowDown" {
                        speed.set(|_| 50);
                    }
                }
            },
            Deps::none(),
        );

        let handle_key_up = use_callback(
            {
                let mut speed = speed;
                move |evt: KeyboardEvent| {
                    let code = evt.code();
                    if code == "ArrowDown" {
                        speed.set(|_| 500)
                    }
                }
            },
            Deps::none(),
        );

        h!(div)
            .tabindex(0)
            .ref_container(&element_container)
            .on_keydown(&handle_key_down)
            .on_keyup(&handle_key_up)
            .style(
                &Style::new()
                    .display("inline-grid")
                    .grid_template(format!(
                        "repeat({}, 1em) / repeat({}, 1em)",
                        self.height, self.width
                    ))
                    .border("1px solid #ccc".to_string())
                    .outline("none"),
            )
            .build(c![..tetris.value().iter_positions().map(|pos| {
                let typ = tetris.value().get(pos);

                h!(div).build(c![typ.unwrap_or_default()])
            })])
    }
}

export_components! { App }
