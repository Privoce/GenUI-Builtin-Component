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
