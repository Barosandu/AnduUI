use serde::{Deserialize, Serialize};

use crate::view_traits::Frame;
use crate::views::{BoundingBox, Child, View, ViewType};
use crate::views::ViewType::VSTACK;

// #[macro_export] macro_rules! vstack {
//     ($( $children: expr ) *) => {
//         {
//             #[allow(unused_mut)]
//             let mut vs: View<_> = View::empty(ViewType::VSTACK);
//             let mut children: Vec<View<_>> = vec![];
//             $(
//               children.push($children as View<_>);
//             )*
//             vs.children = children;//.into_iter().map(|c| {c.set_height(100 / ln).set_width(100)}).collect();
//             vs
//         }
//     };
// }

impl<T: Serialize + Default + for<'a> Deserialize<'a> + 'static> Default for Child<T> {
    fn default() -> Self {
        return Child::<T>::view(View::empty(VSTACK));
    }
}
pub fn vstack<T: Default + Serialize + for<'a> Deserialize<'a> + 'static>(children: &mut [Child<T>]) -> View<T> {
    let mut v: View<T> = View::<T>::empty(ViewType::VSTACK);
    let mut c = vec![];
    for i in 0..children.len() {
        let child_value = std::mem::take(&mut children[i]);
        c.push(child_value);
    }
    v.children = c;
    return v;
}


// vstack! { bbox in
//     chestie
// }

