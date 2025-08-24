use makepad_widgets::{LiveId, SmallVec, WidgetRef};

// use crate::components::menu::sub::GSubMenuWidgetRefExt;

#[derive(Debug, Clone)]
pub enum MenuItemMode {
    /// sub menu which has a title and items, items can be sub menu or menu item
    SubMenu{
        active: bool,
        value: String,
        items: Vec<MenuItemMode>
    },
    /// menu item as a leaf node, `bool` is selected or not
    MenuItem{
        value: String,
        active: bool
    },
}

// impl MenuItemMode {
//     pub fn is_menu_item(&self) -> bool {
//         matches!(self, MenuItemMode::MenuItem(_))
//     }
//     pub fn is_sub_menu(&self) -> bool {
//         matches!(self, MenuItemMode::SubMenu(_))
//     }
//     /// ## find node mode by levels
//     /// `[1, 2]`: 0 => x |1 =>SubMenu(0 =>x, 1=> x, 2=> MenuItem) ..., means index 1 is sub menu, sub menu's index 2 is menu item
//     pub fn find(items: &Vec<MenuItemMode>, levels: &Vec<usize>) -> Option<MenuItemMode> {
//         if levels.is_empty() || items.is_empty() {
//             return None;
//         }

//         let len = levels.len();
//         // do zip to find the node
//         for (index, level) in levels.iter().enumerate() {
//             let item = items.get(*level)?;
//             if index == len - 1 {
//                 return Some(item.clone());
//             } else {
//                 // continue do find
//                 if let MenuItemMode::SubMenu(subs) = item {
//                     return MenuItemMode::find(subs, &levels[index + 1..].to_vec());
//                 }
//             }
//         }

//         None
//     }
//     pub fn find_node<F>(items: &mut SmallVec<[(LiveId, WidgetRef); 2]>, levels: &Vec<usize>, f: &mut F)
//     where
//         F: FnMut(&mut WidgetRef) -> (),
//     {
//         if levels.is_empty() || items.is_empty() {
//             return;
//         }

//         let len = levels.len();
//         for (index, level) in levels.iter().enumerate() {
//             items.get_mut(*level).map(|(_, item)| {
//                 if index == len - 1 {
//                     f(item);
//                 } else {
//                     item.as_gsub_menu().borrow_mut().map(|mut sub| {
//                         MenuItemMode::find_node(
//                             &mut sub.items.children,
//                             &levels[index + 1..].to_vec(),
//                             f,
//                         );
//                     });
//                 }
//             });
//         }
//     }
//     /// get the selected index of the menu item
//     /// try to find the item which is selected in the menu item
//     pub fn selected(items: &Vec<MenuItemMode>) -> Option<Vec<usize>> {
//         fn handle_nested(items: &Vec<MenuItemMode>, levels: &mut Vec<usize>) -> bool {
//             let mut flag = false;
//             for (index, item) in items.iter().enumerate() {
//                 match item {
//                     MenuItemMode::SubMenu(subs) => {
//                         if handle_nested(subs, levels) {
//                             levels.splice(0..0, vec![index]);
//                             flag = true;
//                             break;
//                         }
//                     }
//                     MenuItemMode::MenuItem(selected) => {
//                         if *selected {
//                             levels.push(index);
//                             return true;
//                         }
//                     }
//                 }
//             }
//             flag
//         }
//         if items.is_empty() {
//             return None;
//         }

//         let mut levels = vec![];
//         if handle_nested(items, &mut levels) {
//             Some(levels)
//         } else {
//             None
//         }
//     }
// }
