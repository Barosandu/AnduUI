use std::{collections::HashMap, env::VarError};

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::views::{BoundingBox, Child, ClickedEvent::{self, ClickedAt}, ViewEvent, ViewPossibleEvent, ViewType::{self, VSTACK}, _View};
use std::mem;

impl<S: Serialize + for<'a> Deserialize<'a> + Default + 'static> Child<S> {
    pub fn to_(self) -> _View {
        match self {
            Child::view(v) => v.to_(),
            Child::_view(v) => v,
        }
    }
}


impl<S> Child<S> {
    pub fn get_dimension(&self) -> i32 {
        match self {
            Child::view(v) => v.get_dimension(),
            Child::_view(v) => v.get_dimension(),
        }
    }

    pub fn get_other_dimension(&self) -> i32 {
        match self {
            Child::view(v) => v.get_other_dimension(),
            Child::_view(v) => v.get_other_dimension(),
        }
    }

    pub fn with_dimension(self, d: i32, vt: ViewType, other_d: i32) -> Self {
        match self {
            Child::view(v) => Child::view(v.with_dimension(d, vt, other_d)),
            Child::_view(v) => Child::_view(v.with_dimension(d, vt, other_d)),
        }
    }

    pub fn get_bbox(&self) -> BoundingBox {
        match self {
            Child::view(v) => v.bounding_box.clone(),
            Child::_view(v) => v.bounding_box.clone(),
        }
    }

    pub fn set_bounding_boxes(self) -> Self {
        match self {
            Child::view(v) => Child::view(v.set_bounding_boxes()),
            Child::_view(v) => Child::_view(v.set_bounding_boxes()),
        }
    }
}

impl _View {
    pub fn get_dimension(&self) -> i32 {
        if self.view_type ==  VSTACK {
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
        let unmapped: Vec<_View> = my_old_children.into_iter().map(|c| { c.with_dimension(one_child_len, self.view_type.clone(), self.get_other_dimension()) }).collect();
        self.children = unmapped.into_iter().map(|c| { c.set_bounding_boxes() }).collect();
        self
    }
}

impl _View {


    
    pub fn empty(vtype: ViewType) -> Self {
        let mut s =  Self {children: vec![], view_type: vtype, id: "", event_hashmap: HashMap::new(), bounding_box: Default::default()};
        return s;
    }

    fn is_clicked(&self, event: ViewEvent, initial_state: serde_json::Value) -> serde_json::Value {
        if let ClickedAt(x, y) = event.clicked {
            if self.event_hashmap.get(&ViewPossibleEvent::Click).is_some() && self.bounding_box.contains((x, y)) {
                let new_state = (self.event_hashmap.get(&ViewPossibleEvent::Click).unwrap())(initial_state);
                return new_state;
            }
        }
        return initial_state;
    }


    fn with_(mut self, initial_state: Value, event: ViewEvent) -> (Value, Self) {
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

            let mut new_children: Vec<_View> = vec![];
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



    pub fn with(mut self, initial_state: Value, event: ViewEvent) -> (Value, Self) {
        // self = self.set_defaults();
        // self = self.set_bounding_boxes();
        self.with_(initial_state, event)
    }
}
