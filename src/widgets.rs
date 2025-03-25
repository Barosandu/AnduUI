use serde::{Deserialize, Serialize};
use serde_json::{to_value, Value};
use crate::to_dict::ToDict;
use crate::view_traits::Frame;
use crate::views::{_View, View, ViewEvent};

pub(crate) type DrawView<State> = fn(State, ViewEvent, (i32, i32)) -> (State, View<State>);
pub(crate) type _DrawView = Box<dyn Fn(Value, ViewEvent, (i32, i32)) -> (Value, _View)>;

pub(crate) trait Stateful<State: Default> {
    fn get_state(&mut self) -> State;
    fn set_state(&mut self, state: State);
}


pub(crate) trait Widget<State: Default + Serialize + for<'a> Deserialize<'a> + 'static>: Stateful<State> {
    fn draw(&self) -> _DrawView;
    fn draw_view(&mut self, event: ViewEvent, rect: (i32, i32)) -> _View {
        let (new_state, view) = (self.draw())(to_value(self.get_state()).unwrap(), event.clone(), rect);
        // let (new_new_state, view) = view.with(new_state, event);
        // self.set_state(new_new_state);
        let new_view = view;
        return new_view;
    }

}