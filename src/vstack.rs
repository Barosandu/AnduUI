use crate::view_traits::Frame;
use crate::views::{BoundingBox, View, ViewType};
use crate::views::ViewType::VSTACK;

#[macro_export] macro_rules! vstack {
    ($( $children: expr ) *) => {
        {

            #[allow(unused_mut)]
            let mut vs: View<_> = View::empty(ViewType::VSTACK);
            let mut children: Vec<View<_>> = vec![];
            $(
              children.push($children as View<_>);
            )*
            //
            // let mut ln = children.len() as i32;
            //
            vs.children = children;//.into_iter().map(|c| {c.set_height(100 / ln).set_width(100)}).collect();

            vs

        }
    };
}


// vstack! { bbox in
//     chestie
// }

