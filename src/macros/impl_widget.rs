/// This macro generates the following functions: 
/// - `text`
/// - `set_text`
/// - `is_visible` in Widget trait
#[macro_export]
macro_rules! set_text_and_visible_fn {
    () => {
        fn text(&self) -> String {
            self.text.as_ref().to_string()
        }
        fn set_text(&mut self, cx: &mut Cx, v: &str) {
            self.text.as_mut_empty().push_str(v);
            self.redraw(cx)
        }
        fn is_visible(&self) -> bool {
            self.visible
        }
    };
}

