#[macro_export]
macro_rules! set_scope_path {
    () => {
        fn set_scope_path(&mut self, path: &HeapLiveIdPath) {
            if self.scope_path.is_none() {
                self.scope_path.replace(path.clone());
            }
        }
    };
}


#[macro_export]
macro_rules! setter {
    ($T:ty {$(
        $fn_name: ident ($arg: ident: $arg_ty: ty) {$code: expr }
    ),*}) => {
        // crate::setter!($T);
        #[allow(unused_variables)]
        fn setter<F>(&mut self, cx: &mut Cx, f: F) -> Result<(), Box<dyn std::error::Error>>
        where
            F: FnOnce(&mut $T, &mut Cx) -> Result<(), Box<dyn std::error::Error>>
        {
            f(self, cx)
        }

        $(
            pub fn $fn_name(&mut self, cx: &mut Cx, $arg: $arg_ty) -> Result<(), Box<dyn std::error::Error>> {
                return self.setter(cx, $code);
            }
        )*
    };
}

#[macro_export]
macro_rules! getter {
    ($T:ty {$(
        $fn_name: ident ($return_ty: ty) {$code: expr}
    ),*}) => {
        fn getter<T, F>(&self, f: F) -> T
        where
            F: Fn(&$T) -> T,
        {
            f(self)
        }

        $(
            pub fn $fn_name(&self) -> $return_ty{
                self.getter($code)
            }
        )*
    };
}

// #[macro_export]
// macro_rules! apply_map {
//     ($nodes: ident : $map: expr => {
        
//     },*) => {
//         $(
//             if let Some(index) = $nodes.child_by_path(
//                 index,
//                 &[
//                     live_id!(prop).as_field(),
//                     live_id!($state).as_field(),
//                     live_id!($field).as_field(),
//                 ],
//             ) {
//                 let node = $nodes[index];
//                 $map
//                     .insert(node.id.to_string(), node.value.clone());
//             }
//         )*
//     };
// }