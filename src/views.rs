

#[derive(Clone)]
pub(crate) enum ClickedEvent {
    NoClick,
    ClickedAt(i32, i32),
}

use std::cmp::PartialEq;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter, write};
use std::marker::PhantomData;
use std::mem;
use std::ops::Add;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use ClickedEvent::ClickedAt;
use crate::convert;
use crate::to_dict::ToDict;
use crate::view_traits::Frame;
use crate::views::ViewType::VSTACK;

#[derive(PartialEq, Eq, Clone)]
pub enum ViewType {
    VSTACK,
    HSTACK,
}

impl Default for ViewType {
    fn default() -> Self {
        VSTACK
    }
}

macro_rules! default_width {
    () => {
        100
    };
}

macro_rules! default_height {
    () => {
        100
    };
}


type Measurement = Vec<i32>;

//
//
// struct Function<WHType: 'static + Sized + Copy> {
//     measurement: Measurement,
//     phantom_data: PhantomData<WHType>
// }
//
// impl<T: Copy> Debug for Function<T> {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{}", "")
//     }
// }
//
// impl<WHType: 'static + Sized + Copy>  Default for Function<WHType> {
//     fn default() -> Self {
//         Self::identitity()
//     }
// }
//
// impl<T: 'static + Sized + Copy> Function<T> {
//     pub fn compose (
//         f: Measurement,
//         g: Measurement
//     ) -> Measurement
//     {
//         Box::new(move |x, w, h| g(f(x, w, h), w, h))
//     }
//
//     pub fn identitity() -> Self {
//         return Self{ measurement: Box::new(|x,_,_| {x}), phantom_data: Default::default() };
//     }
// }
//
//
// impl<T: 'static + Sized + Copy + Add<Output = T>> Function<T> {
//     pub fn add_measurements(f: Measurement, g: Measurement) -> Measurement
//     {
//         Box::new(move |x, w, h| f(x, w, h) + g(x, w, h))
//     }
//
//
//     pub fn sub_measurements(f: Measurement, g: Measurement) -> Measurement
//     {
//         Box::new(move |x, w, h| f(x, w, h) + g(x, w, h))
//     }
// }
//
//
// impl Function<i32> {
//     pub fn div_measurements(f: Measurement, term: i32) -> Measurement
//     {
//         Box::new(move |x, w, h| f(x, w, h) / term)
//     }
// }
//
//
// impl<T: 'static + Sized + Copy + Add<Output = T>> std::ops::Add for Function<T> {
//     type Output = Self;
//
//     fn add(self, rhs: Self) -> Self::Output {
//         Self { measurement: Self::add_measurements(self.measurement, rhs.measurement), phantom_data: Default::default() }
//     }
// }
//
//
// impl<T: 'static + Sized + Copy + Add<Output = T>> std::ops::Sub for Function<T> {
//     type Output = Self;
//
//     fn sub(self, rhs: Self) -> Self::Output {
//         Self { measurement: Self::sub_measurements(self.measurement, rhs.measurement), phantom_data: Default::default() }
//     }
// }




#[derive(Default, Debug)]
pub(crate) struct BoundingBox {
    pub x: i32,
    pub y: i32,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub relative: bool
}


impl BoundingBox {
    pub fn contains(&self, p: (i32, i32)) -> bool {
        // let (x, y) = p;
        // if x < self.x {
        //     return false;
        // }
        // 
        // if y < self.y {
        //     return false;
        // }
        // 
        // if x > self.x + self.width.unwrap() {
        //     return false;
        // }
        // if y > self.y + self.height.unwrap() {
        //     return false;
        // }

        true
    }
    pub fn is_valid(&self) -> bool {
        return self.width.is_some() && self.height.is_some();
    }


    pub fn is_valid2(&self, dim: ViewType) -> bool {
        if dim == VSTACK {
            return self.height.is_some();
        }

        return self.width.is_some();
    }
}
#[derive(Clone)]
pub(crate) struct ViewEvent {
    pub clicked: ClickedEvent,
}

impl ViewEvent {
    pub fn to(self) -> (fn((i32, i32), Self) -> Self){
        return |_, s| {s};
    }
}

#[derive(PartialEq, Eq, Hash)]
pub(crate) enum ViewPossibleEvent {
    Idle,
    Click,
    Appear,
    Dissapear,
}
type EventMap<S> = HashMap<ViewPossibleEvent, fn(S) -> S>;
type Func<S> = fn(S) -> S;

pub(crate) enum Child<S> {
    view(View<S>), _view(_View)
}
#[derive(Default)]
pub(crate) struct View<S> {
    pub children: Vec<View<S>>,
    pub view_type: ViewType,
    pub id: &'static str,
    pub event_hashmap: EventMap<S>,
    pub bounding_box: BoundingBox,
}

type _EventMap = HashMap<ViewPossibleEvent, Box<dyn Fn(Value) -> Value>>;
pub(crate) struct _View {
    pub children: Vec<_View>,
    pub view_type: ViewType,
    pub id: &'static str,
    pub event_hashmap: _EventMap,
    pub bounding_box: BoundingBox
}
impl<T> View<T> {
    pub fn get_dimension(&self) -> i32 {
        if self.view_type == VSTACK {
            return self.bounding_box.height.unwrap();
        } else {
            return self.bounding_box.width.unwrap();
        }
    }

    pub fn get_other_dimension(&self) -> i32 {
        if self.view_type != VSTACK {
            return self.bounding_box.height.unwrap();
        } else {
            return self.bounding_box.width.unwrap();
        }
    }

    pub fn with_dimension(mut self, d: i32, vt: ViewType, other_d: i32) -> Self {
        if vt == VSTACK {
            if self.bounding_box.height.is_none() {
                self.bounding_box.height = Some(d);
            }

            if self.bounding_box.width.is_none() {
                self.bounding_box.width = Some(other_d);
            }
        } else {
            if self.bounding_box.height.is_none() {
                self.bounding_box.height = Some(other_d);
            }

            if self.bounding_box.width.is_none() {
                self.bounding_box.width = Some(d);
            }
        }
        return self
    }

    pub fn set_bounding_boxes(mut self) -> Self {
        if self.bounding_box.is_valid() != true {
            panic!("Bounding box must be set to valid. please add .set_width() and .set_height() modifiers");
        }

        if self.children.len() == 0 {
            return self;
        }

        let total_len: i32 = self.get_dimension();
        let set_children: Vec<i32> = self.children.iter().filter(|item| {(item).bounding_box.is_valid2(self.view_type.clone())}).map(|item| {item.get_dimension()}).collect();
        let unset_count = (self.children.len() - set_children.iter().count()) as i32;
        let set_len: i32 = set_children.iter().sum();

        let unset_len = total_len - set_len;

        let one_child_len = unset_len / unset_count;
        let my_old_children = mem::take(&mut self.children);
        let unmapped: Vec<View<_>> = my_old_children.into_iter().map(|c| { c.with_dimension(one_child_len, self.view_type.clone(), self.get_other_dimension()) }).collect();
        self.children = unmapped.into_iter().map(|c| { c.set_bounding_boxes() }).collect();
        self
    }
}
impl<S: Default + Serialize + for<'a> Deserialize<'a> + 'static> View<S> {
    pub fn to<T>(self) -> View<T> {
        let new_children = self.children.into_iter().map(|c| c.to::<T>()).collect();

        let me = View {
            children: new_children,
            view_type: self.view_type,
            id: self.id,
            event_hashmap: HashMap::new(),
            bounding_box: self.bounding_box,
        };
        me
    }
    pub fn to_(self) -> _View {
        let new_children = self.children.into_iter().map(|c| c.to_()).collect();


        let mut _event_hashmap: _EventMap = _EventMap::new();
        for (key, function) in self.event_hashmap {
            _event_hashmap.insert(key, convert(function));
        }

        let me = _View {
            children: new_children,
            view_type: self.view_type,
            id: self.id,
            event_hashmap: _event_hashmap,
            bounding_box: self.bounding_box
        };
        me
    }

    pub fn on(mut self, event: ViewPossibleEvent, f: Func<S>) -> Self {
        self.event_hashmap.insert(event, f);
        self
    }

    pub fn empty(vtype: ViewType) -> Self {
        let mut s =  Self {children: vec![], view_type: vtype, id: "", event_hashmap: HashMap::new(), bounding_box: Default::default()};
        return s;
    }

    fn is_clicked(&self, event: ViewEvent, initial_state: S) -> S {
        if let ClickedAt(x, y) = event.clicked {
            if self.event_hashmap.get(&ViewPossibleEvent::Click).is_some() && self.bounding_box.contains((x, y)) {
                let new_state = (self.event_hashmap.get(&ViewPossibleEvent::Click).unwrap())(initial_state);
                return new_state;
            }
        }
        return initial_state;
    }


    fn with_(mut self, initial_state: S, event: ViewEvent) -> (S, Self) {
        if let ClickedAt(x, y) = event.clicked {
            // daca clickul nu e in mine => opreste arborele
            //
            let mut state = initial_state;
            if self.bounding_box.contains((x, y)) == false {
                return (state, self);
            } else {
                // daca clickul e in mine executa clickul
                let new_state = self.is_clicked(event.clone(), state);
                state = new_state;
            }

            let mut new_children: Vec<View<S>> = vec![];
            for child in self.children.into_iter() {
                let (new_state, new_child) = child.with_(state, event.clone());
                state = new_state;
                new_children.push(new_child);
            }

            self.children = new_children;

            return (state, self);
        }

        (initial_state, self)
    }



    pub fn with(mut self, initial_state: S, event: ViewEvent) -> (S, Self) {
        // self = self.set_defaults();
        // self = self.set_bounding_boxes();
        self.with_(initial_state, event)
    }
}
