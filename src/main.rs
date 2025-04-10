mod views;
mod _views;
pub use UI_ANDU_new_derive::Stateful;
use vstack::vstack;
mod functions;
mod to_dict;
mod view_traits;
mod vstack;
mod widgets;
use crate::to_dict::ToDict;
use crate::view_traits::Frame;
use crate::views::ClickedEvent::NoClick;
use crate::widgets::{_DrawView, Stateful};
use UI_ANDU_new_derive::ToDict;
use ViewPossibleEvent::*;
use colored::*;
use serde::{Deserialize, Serialize};
use serde_json::{Value, from_value, json, to_value};
use std::collections::HashMap;
use std::mem;
use views::{Child, ClickedEvent, View, ViewEvent, ViewPossibleEvent, ViewType};
use widgets::{DrawView, Widget};

#[derive(Default, Deserialize, Serialize)]
struct AppState {
    x: i32,
    y: String
}

macro_rules! using {
    (state as $typ: ty, draw view $vw: expr) => {{
        #[allow(unused_mut)]
        let func: DrawView<$typ> = $vw;

        convert_drawview(func)
    }};
}

#[derive(Default, Stateful)]
#[state_type(AppState)]
struct App {
    state: AppState
}

impl Widget<AppState> for App {
    fn draw(&self) -> _DrawView {
        let func: DrawView<AppState> = |mut state, event, rect| {
            let x = vstack(&mut [
                Child::view(vstack(&mut [])
                    .on(Click, |mut s: AppState| {
                        s.x = 1;
                        s
                    })
                    .on(Click, |s: AppState| s)),
                Child::view(vstack(&mut [])),
            ]);
            return (state, x);
        };
        let converted = convert_drawview(func);
        return converted;
    }
}
#[derive(Default, Deserialize, Serialize)]
struct OtherState {
    x: i32,
    y: String,
    app: AppState
}


#[derive(Default, Stateful)]
#[state_type(OtherState)]
struct Other {
    state: OtherState
}

impl Widget<OtherState> for Other {
    fn draw(&self) -> _DrawView {
        let func: DrawView<OtherState> = |mut state, event, rect| {
            let x = vstack(&mut [
                Child::view(vstack(&mut [])
                    .on(Click, |mut s: OtherState| {
                        s.x = 1;
                        s
                    })
                    .on(Click, |s: OtherState| s)),
                Child::view(vstack(&mut [])),
                Child::_view(App::default().draw_view(event, rect))
            ])
            .set_width(rect.0)
            .set_height(rect.1)
            .set_bounding_boxes();
            return (state, x);
        };
        let converted = convert_drawview(func);
        return converted;
    }
}


fn convert<T: Default + Serialize + for<'a> Deserialize<'a> + 'static>(
    function: fn(T) -> T,
) -> Box<dyn Fn(Value) -> Value> {
    return Box::new(move |value: Value| {
        // Deserialize the Value into the type T
        let deserialized = from_value::<T>(value).unwrap_or(T::default());
        // Apply the function to the deserialized value
        let transformed = function(deserialized);
        // Serialize the result back into a Value
        to_value(transformed).unwrap() // In case serialization fails, return the original value
    });
}

fn convert_drawview<T: Default + Serialize + for<'a> Deserialize<'a> + 'static>(
    function: DrawView<T>,
) -> _DrawView {
    return Box::new(move |state, event, rect| {
        // Deserialize the Value into the type T
        let deserialized = from_value::<T>(state).unwrap_or(T::default());
        // Apply the function to the deserialized value
        let transformed = function(deserialized, event, rect);
        // Serialize the result back into a Value
        let state = to_value(transformed.0);
        let view: View<T> = transformed.1;
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
    let mut view = Other::default();
    for i in 1..2 {
        let v = view.draw_view(ViewEvent { clicked: NoClick }, (100, 100));
        println!("{}", v);
    }
}
