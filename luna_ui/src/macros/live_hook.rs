#[macro_export]
macro_rules! pure_after_apply {
    () => {
        #[allow(unused_variables)]
        fn after_new_from_doc(&mut self, cx: &mut Cx) {
            #[cfg(feature = "release")]
            self.render_after_apply(cx);
        }
        
        #[allow(unused_variables)]
        fn after_apply_from_doc(&mut self, cx: &mut Cx) {
            #[cfg(feature = "dev")]
            self.render_after_apply(cx);
        }
    };
}