use std::fmt::{Display, Formatter};
use colored::Colorize;
use crate::views::{Child, View, ViewType, _View};
use crate::views::ViewType::VSTACK;

pub trait Frame {
    fn set_height(self, height: i32) -> Self;
    fn set_width(self, width: i32) -> Self;
    // fn set_defaults(self) -> Self;
    fn set_relative(self, r: bool) -> Self;
}


impl<T> Frame for View<T> {
    fn set_height(mut self, height: i32) -> Self {
    //     if self.bounding_box.bbox_default {
    //         self.bounding_box.width = None;
    //     }
        self.bounding_box.height = Some(height);
    //     self.bounding_box.bbox_default = false;
        self
    }
    //
    fn set_width(mut self, width: i32) -> Self {
    //     if self.bounding_box.bbox_default {
    //         self.bounding_box.height = None;
    //     }
        self.bounding_box.width = Some(width);
    //     self.bounding_box.bbox_default = false;
        self
    }


    fn set_relative(mut self, r: bool) -> Self {
        //     if self.bounding_box.bbox_default {
        //         self.bounding_box.height = None;
        //     }
        self.bounding_box.relative = r;
        //     self.bounding_box.bbox_default = false;
        self
    }
    //
    // fn set_defaults(mut self) -> Self {
    //     self.bounding_box.width = Some(100);
    //     self.bounding_box.height = Some(100);
    //     self.bounding_box.bbox_default = true;
    //     self
    // }
}

impl<T> Display for Child<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
           Child::view(x) => write!(f, "{}", x),
           Child::_view(x) => write!(f, "{}", x)
        }
    }
}

impl<T> Display for View<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut string: String = String::new();
        string += format!("{}", (if self.view_type == ViewType::VSTACK { "VSTACK" } else { "HSTACK" }).bright_blue().bold()).as_str();

        let formats_width = format!("{:?}", self.bounding_box.width).as_str().red();
        let formats_height = format!("{:?}", self.bounding_box.height).as_str().red();

        string += format!("[w: {}, h: {}]\n", formats_width, formats_height).as_str();
        for child in self.children.iter() {
            for s in format!("{}", child).as_str().split("\n") {
                string += format!("\t{}\n", s).as_str()
            }
        }
        write!(f, "{}", string.as_str())
    }
}

impl Display for _View {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut string: String = String::new();
        string += format!("{}", (if self.view_type == ViewType::VSTACK { "VSTACK" } else { "HSTACK" }).bright_blue().bold()).as_str();

        let formats_width = format!("{:?}", self.bounding_box.width).as_str().red();
        let formats_height = format!("{:?}", self.bounding_box.height).as_str().red();

        string += format!("[w: {}, h: {}]\n", formats_width, formats_height).as_str();
        for child in self.children.iter() {
            for s in format!("{}", child).as_str().split("\n") {
                string += format!("\t{}\n", s).as_str()
            }
        }
        write!(f, "{}", string.as_str())
    }
}

#[macro_export]
macro_rules! anyview {
    ($event: expr => $v: expr) => {
        {
            $v.draw_view($event.clone()).to()
        }
    };

    //
    // (width: $width: expr, height: $height: expr, $event: expr => $v: expr) => {
    //     {
    //         $v.draw_view_in($event.clone(), ($width, $height)).to()
    //     }
    // };
}
