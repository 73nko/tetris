use js_sys::{Function, Reflect};
use tetris::Tetris;
use wasm_bindgen::{prelude::Closure, JsCast, JsValue, UnwrapThrowExt};
use wasm_react::{
    c, export_components, h,
    hooks::{use_effect, use_state, Deps},
    props::Style,
    Component,
};
use web_sys::window;

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

        use_effect(
            {
                let tetris = tetris.clone();
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
                            500,
                        )
                        .unwrap_throw();

                    move || {
                        drop(tick_closure);
                        window().unwrap_throw().clear_interval_with_handle(handle)
                    }
                }
            },
            Deps::none(),
        );

        h!(div)
            .style(
                &Style::new()
                    .display("inline-grid")
                    .grid_template(format!(
                        "repeat({}, 1em) / repeat({}, 1em)",
                        self.height, self.width
                    ))
                    .border("1px solid #ccc".to_string()),
            )
            .build(c![..tetris.value().iter_positions().map(|pos| {
                let typ = tetris.value().get(pos);

                h!(div).build(c![typ.unwrap_or_default()])
            })])
    }
}

export_components! { App }
