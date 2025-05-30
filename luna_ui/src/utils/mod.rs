use toml_edit::{DocumentMut, InlineTable, Item, Value};

pub fn get_from_doc<U, D, F>(doc: &DocumentMut, key: &str, default: D, f: F) -> U
where
    D: FnOnce() -> U,
    F: FnOnce(&Item) -> U,
{
    doc.get(key).map_or_else(default, f)
}

pub fn get_from_itable<U, D, F>(v: &InlineTable, key: &str, default: D, f: F) -> U
where
    D: FnOnce() -> U,
    F: FnOnce(&Value) -> U,
{
    v.get(key).map_or_else(default, f)
}
