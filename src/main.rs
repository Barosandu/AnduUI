mod views;
pub use UI_ANDU_new_derive::Stateful;
mod vstack;
mod widgets;
mod view_traits;
mod functions;
mod to_dict;

use colored::*;
use std::mem;
use views::{ViewEvent, View, ViewPossibleEvent, ClickedEvent, ViewType};
use ViewPossibleEvent::*;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::{from_value, json, to_value, Value};
use UI_ANDU_new_derive::ToDict;
use widgets::{Widget, DrawView};
use crate::view_traits::Frame;
use crate::widgets::{_DrawView, Stateful};
use crate::to_dict::ToDict;
use crate::views::ClickedEvent::NoClick;

#[derive(Default, Deserialize, Serialize)]
struct AppState {
    x: i32,
    y: String,
}


macro_rules! using {
    (state as $typ: ty, draw view $vw: expr) => {
        {
            #[allow(unused_mut)]
            let func: DrawView<$typ> = $vw;

            convert_drawview(func)
        }
    };
}

#[derive(Default, Stateful)]
#[state_type(AppState)]
struct App {
    state: AppState,
}

impl Widget<AppState> for App {
    fn draw(&self) -> _DrawView {
        using!(state as AppState, draw view |mut state, event, rect| {
            let x = vstack![
                vstack!(
                    vstack!()
                    vstack!(
                        vstack!()
                            .on(Click, |mut s: AppState| {println!("hello"); s.x += 1; s})
                        vstack!(
                            vstack!()
                                .on(Click, |s: AppState| {println!("hello"); s})
                        )
                            .on(Click, |s: AppState| {println!("hello"); s})
                    )
                    .on(Click, |s: AppState| {println!("hello"); s})
                )
                vstack!()
            ]
                .set_width(rect.0)
                .set_height(rect.1)
                .set_bounding_boxes();
            return (state, x);
        })
    }
}


fn convert<T: Default + Serialize + for<'a> Deserialize<'a> + 'static>(function: fn(T) -> T) -> Box<dyn Fn(Value) -> Value> {
    return Box::new(move |value: Value| {
        // Deserialize the Value into the type T
        let deserialized = from_value::<T>(value).unwrap_or(T::default());
        // Apply the function to the deserialized value
        let transformed = function(deserialized);
        // Serialize the result back into a Value
        to_value(transformed).unwrap()  // In case serialization fails, return the original value
    });
}


fn convert_drawview<T: Default + Serialize + for<'a> Deserialize<'a> + 'static>(function: DrawView<T>) -> _DrawView {
    return Box::new(move |state, event, rect| {
        // Deserialize the Value into the type T
        let deserialized = from_value::<T>(state).unwrap_or(T::default());
        // Apply the function to the deserialized value
        let transformed = function(deserialized, event, rect);
        // Serialize the result back into a Value
        let state = to_value(transformed.0);
        let view: View<T> = (transformed.1);
        // In case serialization fails, return the original value
        (state.unwrap(), view.to_())
    });
}

#[derive(Serialize, Deserialize, Debug)]
struct Adi {
    adi: String,
    x: i32,
}


fn main() {
    let mut view = App::default();
    for i in 1..2 {
        let v = view.draw_view(ViewEvent { clicked: NoClick }, (100, 100));
        println!("{}", v);
    }
}
