#[macro_export]
macro_rules! pure_after_apply {
    () => {
        #[allow(unused_variables)]
        #[cfg(feature = "release")]
        fn after_new_from_doc(&mut self, cx: &mut Cx) {
            self.sync();
            self.render_after_apply(cx);
        }
        
        #[allow(unused_variables)]
        #[cfg(feature = "dev")]
        fn after_apply_from_doc(&mut self, cx: &mut Cx) {
            self.sync();
            self.render_after_apply(cx);
        }
    };
}

#[macro_export]
macro_rules! pure_after_apply_slot {
    () => {
        #[allow(unused_variables)]
        #[cfg(feature = "release")]
        fn after_new_from_doc(&mut self, cx: &mut Cx) {
            self.sync();
            self.render_after_apply(cx);
        }

        #[allow(unused_variables)]
        #[cfg(feature = "dev")]
        fn after_apply_from_doc(&mut self, cx: &mut Cx) {
            self.sync();
            self.render_after_apply(cx);
            self.merge_from_super();
        }
    };
}