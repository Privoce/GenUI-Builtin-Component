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
        fn setter<F>(&mut self, cx: &mut Cx, f: F) -> Result<(), crate::error::Error>
        where
            F: FnOnce(&mut $T, &mut Cx) -> Result<(), crate::error::Error>
        {
            f(self, cx)
        }

        $(
            pub fn $fn_name(&mut self, cx: &mut Cx, $arg: $arg_ty) -> Result<(), crate::error::Error> {
                return self.setter(cx, $code);
            }
        )*
    };
}

#[macro_export]
macro_rules! getter_setter_prop {
    ($(
        $fn_getter: ident, $fn_setter: ident : $prop: ident -> $v_ty: ty
    ),*) => {
        $(
            pub fn $fn_setter(&mut self, v: $v_ty) -> () {
                self.$prop = v;
            }
            pub fn $fn_getter(&self) -> $v_ty {
                self.$prop
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

#[macro_export]
macro_rules! set_index {
    () => {
        fn set_index(&mut self, index: usize) {
            self.index = index;
        }
    };
}

#[macro_export]
macro_rules! lifecycle {
    () => {
        fn lifecycle(&self) -> LifeCycle {
            self.lifecycle
        }
    };
}

#[macro_export]
macro_rules! visible {
    () => {
        fn visible(&self) -> bool {
            self.visible
        }
    };
}

/// ## generate state_colors function
/// ### usage
/// ```
/// state_colors!{
///     (bg_level, stroke_level, border_level),
///     RadioState::Basic => (200, 200, 400),
///     RadioState::Hover => (200, 200, 400),
///     RadioState::Active => (500, 200, 500),
///     RadioState::Disabled => (100, 100, 300)
/// }
/// ```
/// ### generate code
/// ```
/// fn state_colors(theme: Theme, state: Self::State) -> Self::Colors {
///     let (bg_level, stroke_level, border_level) = match state {
///         RadioState::Basic => (200, 200, 400),
///         RadioState::Hover => (200, 200, 400),
///         RadioState::Active => (500, 200, 500),
///         RadioState::Disabled => (100, 100, 300),
///     };
///
///     match theme {
///         Theme::Dark => (
///             Theme::Dark.color(bg_level),
///             Theme::Dark.color(stroke_level),
///             Theme::Dark.color(border_level),
///         ),
///         Theme::Primary => (
///             Theme::Primary.color(bg_level),
///             Theme::Primary.color(stroke_level),
///             Theme::Primary.color(border_level),
///         ),
///         ...
///     }
/// }
/// ```
#[macro_export]
macro_rules! state_colors {
    (
        ($($level: ident),*),
        $($state: path => ($($level_number: expr),*)),*
    ) => {
        fn state_colors(theme: Theme, state: Self::State) -> Self::Colors {
            let ($($level),*) = match state {
                $(
                    $state => (
                        $($level_number),*
                    ),
                )*

            };

            match theme {
                Theme::Dark => (
                    $(Theme::Dark.color($level),)*
                ),
                Theme::Primary => (
                    $(Theme::Primary.color($level),)*
                ),
                Theme::Error => (
                    $(Theme::Error.color($level),)*
                ),
                Theme::Warning => (
                    $(Theme::Warning.color($level),)*
                ),
                Theme::Success => (
                    $(Theme::Success.color($level),)*
                ),
                Theme::Info => (
                    $(Theme::Info.color($level),)*
                ),
            }.into()
        }
    };
}

#[macro_export]
macro_rules! state_color {
    (
        ($level: ident),
        $($state: path => ($level_number: expr)),*
    ) => {
        fn state_colors(theme: Theme, state: Self::State) -> Self::Colors {
            let $level = match state {
                $(
                    $state => (
                        $level_number
                    ),
                )*

            };

            match theme {
                Theme::Dark => Theme::Dark.color($level),
                Theme::Primary => Theme::Primary.color($level),
                Theme::Error => Theme::Error.color($level),
                Theme::Warning => Theme::Warning.color($level),
                Theme::Success => Theme::Success.color($level),
                Theme::Info => Theme::Info.color($level),
            }.into()
        }
    };
}

/// ## generate `get` and `get_mut` fn in `Style` trait
/// ### usage
/// ```
/// get_get_mut!{
///     RadioState::Basic => basic,
///     RadioState::Hover => hover,
///     RadioState::Active => active,
///     RadioState::Disabled => disabled
/// }
/// ```
/// ### generate code
/// ```
/// fn get(&self, state: Self::State) -> &Self::Basic {
///     match state {
///         RadioState::Basic => &self.basic,
///         RadioState::Hover => &self.hover,
///         RadioState::Active => &self.active,
///         RadioState::Disabled => &self.disabled,
///     }
/// }
///
/// fn get_mut(&mut self, state: Self::State) -> &mut Self::Basic {
///     match state {
///         RadioState::Basic => &mut self.basic,
///         RadioState::Hover => &mut self.hover,
///         RadioState::Active => &mut self.active,
///         RadioState::Disabled => &mut self.disabled,
///     }
/// }
/// ```
#[macro_export]
macro_rules! get_get_mut {
    ($(
        $state_path: path => $state_prop: tt
    ),*) => {
        fn get(&self, state: Self::State) -> &Self::Basic {
            match state {
                $(
                    $state_path => &self.$state_prop,
                )*
            }
        }

        fn get_mut(&mut self, state: Self::State) -> &mut Self::Basic {
            match state {
                $(
                    $state_path => &mut self.$state_prop,
                )*
            }
        }
    };
}

#[macro_export]
macro_rules! sync {
    () => {
        fn sync(&mut self) -> () {
            if !self.sync {
                return;
            }

            self.focus_sync();
        }
    };
}

/// ## Inherit View Basic Properties
/// This macro generates a struct that inherits the basic properties of a view.
/// ### When to use
/// You may find, sometimes the component prop which use `ViewBasicStyle` will change to `ViewBasicStyle::default()`
/// instead of the right prop value apply from doc after Live reloading application
/// ### Example
/// ```
/// inherits_view_basic_prop!{
///     ContainerPartProp {
///         border_width: 0.0,
///         border_radius: Radius::new(4.0),
///         spread_radius: 0.0,
///         blur_radius: 0.0,
///         shadow_offset: vec2(0.0, 0.0),
///         background_visible: false,
///         rotation: 0.0,
///         scale: 1.0,
///         padding: Padding::from_f64(0.0),
///         margin: Margin::from_f64(0.0),
///         clip_x: false,
///         clip_y: false,
///         align: Align::from_f64(0.5),
///         cursor: MouseCursor::default(),
///         flow: Flow::Down,
///         spacing: 0.0,
///         height: Size::Fit,
///         width: Size::Fit,
///         abs_pos: None,
///     }, SvgState, "svg.container",
///     {
///         SvgState::Basic => (500, 500, 400),
///         SvgState::Hover => (400, 400, 300),
///         SvgState::Pressed => (600, 600, 500),
///         SvgState::Disabled => (300, 300, 200)
///     }
/// }
/// ```
#[macro_export]
macro_rules! inherits_view_basic_prop {
    ($struct_name: ident {
        border_width: $border_width_value: expr,
        border_radius: $border_radius_value: expr,
        spread_radius: $spread_radius_value: expr,
        blur_radius: $blur_radius_value: expr,
        shadow_offset: $shadow_offset_value: expr,
        background_visible: $background_visible_value: expr,
        rotation: $rotation_value: expr,
        scale: $scale_value: expr,
        padding: $padding_value: expr,
        margin: $margin_value: expr,
        clip_x: $clip_x_value: expr,
        clip_y: $clip_y_value: expr,
        align: $align_value: expr,
        cursor: $cursor_value: expr,
        flow: $flow_value: expr,
        spacing: $spacing_value: expr,
        height: $height_value: expr,
        width: $width_value: expr,
        abs_pos: $abs_pos_value: expr,
    }, $state: ident, $name: expr, {$($state_path: path => ($($level_number: expr),*)),*}) => {
        crate::basic_prop_interconvert! {
            $struct_name {
                state = $state;
                {
                    background_color => BACKGROUND_COLOR, |v| v.try_into(),
                    border_color => BORDER_COLOR, |v| v.try_into(),
                    shadow_color => SHADOW_COLOR, |v| v.try_into()
                };
                {
                    border_width: f32 => BORDER_WIDTH, $border_width_value, |v| v.to_f32(),
                    border_radius: Radius => BORDER_RADIUS, $border_radius_value, |v| v.try_into(),
                    spread_radius: f32 => SPREAD_RADIUS, $spread_radius_value, |v| v.to_f32(),
                    blur_radius: f32 => BLUR_RADIUS, $blur_radius_value, |v| v.to_f32(),
                    shadow_offset: Vec2 => SHADOW_OFFSET, $shadow_offset_value, |v| v.to_vec2(shadow_offset),
                    background_visible: bool => BACKGROUND_VISIBLE, $background_visible_value, |v| v.to_bool(),
                    rotation: f32 => ROTATION, $rotation_value, |v| v.to_f32(),
                    scale: f32 => SCALE, $scale_value, |v| v.to_f32(),
                    padding: Padding => PADDING, $padding_value, |v| v.to_padding(padding),
                    margin: Margin => MARGIN, $margin_value, |v| v.to_margin(margin),
                    clip_x: bool => CLIP_X, $clip_x_value, |v| v.to_bool(),
                    clip_y: bool => CLIP_Y, $clip_y_value, |v| v.to_bool(),
                    align: Align => ALIGN, $align_value, |v| v.to_align(align),
                    cursor: MouseCursor => CURSOR, $cursor_value, |v| v.to_cursor(),
                    flow: Flow => FLOW, $flow_value, |v| v.to_flow(),
                    spacing: f64 => SPACING, $spacing_value, |v| v.to_f64(),
                    height: Size => HEIGHT, $height_value, |v| v.to_size(),
                    width: Size => WIDTH, $width_value, |v| v.to_size(),
                    abs_pos: AbsPos => ABS_POS, $abs_pos_value, |v| Ok(v.to_dvec2().map_or(None, |v| Some(v)))

                }
            }, format!("[components.{}.$state] should be an inline table", $name)
        }

        impl BasicStyle for $struct_name {
            type State = $state;

            type Colors = crate::components::ViewColors;

            fn len() -> usize {
                22
            }

            fn set_from_str(&mut self, key: &str, value: &LiveValue, state: Self::State) -> () {
                match key {
                    THEME => {
                        self.theme = Theme::from_live_value(value).unwrap_or(Theme::default());
                        self.sync(state);
                    }
                    BACKGROUND_COLOR => {
                        let colors = Self::state_colors(self.theme, state);
                        self.background_color =
                            Vec4::from_live_color(value).unwrap_or(colors.background_color.into());
                    }
                    BORDER_COLOR => {
                        let colors = Self::state_colors(self.theme, state);
                        self.border_color =
                            Vec4::from_live_color(value).unwrap_or(colors.border_color.into());
                    }
                    BORDER_WIDTH => {
                        self.border_width =
                            f32::from_live_value(value).unwrap_or($border_width_value);
                    }
                    BORDER_RADIUS => {
                        self.border_radius =
                            Radius::from_live_value(value).unwrap_or($border_radius_value);
                    }
                    SHADOW_COLOR => {
                        let colors = Self::state_colors(self.theme, state);
                        self.shadow_color =
                            Vec4::from_live_color(value).unwrap_or(colors.shadow_color.into());
                    }
                    SPREAD_RADIUS => {
                        self.spread_radius =
                            f32::from_live_value(value).unwrap_or($spread_radius_value);
                    }
                    BLUR_RADIUS => {
                        self.blur_radius =
                            f32::from_live_value(value).unwrap_or($blur_radius_value);
                    }
                    SHADOW_OFFSET => {
                        self.shadow_offset =
                            Vec2::from_live_value(value).unwrap_or($shadow_offset_value);
                    }
                    BACKGROUND_VISIBLE => {
                        self.background_visible =
                            bool::from_live_value(value).unwrap_or($background_visible_value);
                    }
                    ROTATION => {
                        self.rotation = f32::from_live_value(value).unwrap_or($rotation_value);
                    }
                    SCALE => {
                        self.scale = f32::from_live_value(value).unwrap_or($scale_value);
                    }
                    PADDING => {
                        self.padding = Padding::from_live_value(value).unwrap_or($padding_value);
                    }
                    MARGIN => {
                        self.margin = Margin::from_live_value(value).unwrap_or($margin_value);
                    }
                    CLIP_X => {
                        self.clip_x = bool::from_live_value(value).unwrap_or($clip_x_value);
                    }
                    CLIP_Y => {
                        self.clip_y = bool::from_live_value(value).unwrap_or($clip_y_value);
                    }
                    ALIGN => {
                        self.align = Align::from_live_value(value).unwrap_or($align_value);
                    }
                    CURSOR => {
                        let cursor = if state.is_disabled() {
                            MouseCursor::NotAllowed
                        } else {
                            $cursor_value
                        };
                        self.cursor = MouseCursor::from_live_value(value).unwrap_or(cursor);
                    }
                    FLOW => {
                        self.flow = Flow::from_live_value(value).unwrap_or($flow_value);
                    }
                    SPACING => {
                        self.spacing = f64::from_live_value(value).unwrap_or($spacing_value);
                    }
                    HEIGHT => {
                        self.height = Size::from_live_value(value).unwrap_or($height_value);
                    }
                    WIDTH => {
                        self.width = Size::from_live_value(value).unwrap_or($width_value);
                    }
                    ABS_POS => {
                        self.abs_pos = DVec2::from_live_value(value);
                    }
                    _ => {}
                }
            }

            fn sync(&mut self, state: Self::State) -> () {
                let colors = Self::state_colors(self.theme, state);
                self.background_color = colors.background_color.into();
                self.border_color = colors.border_color.into();
                self.shadow_color = colors.shadow_color.into();
            }

            fn from_state(theme: Theme, state: Self::State) -> Self {
                let colors =
                    Self::state_colors(theme, state);

                let cursor = if state.is_disabled() {
                    MouseCursor::NotAllowed
                } else {
                    $cursor_value
                };

                Self {
                    theme,
                    background_color: colors.background_color.into(),
                    border_color: colors.border_color.into(),
                    border_width: $border_width_value,
                    border_radius: $border_radius_value,
                    shadow_color: colors.shadow_color.into(),
                    spread_radius: $spread_radius_value,
                    blur_radius: $blur_radius_value,
                    shadow_offset: $shadow_offset_value,
                    background_visible: $background_visible_value,
                    rotation: $rotation_value,
                    scale: $scale_value,
                    padding: $padding_value,
                    margin: $margin_value,
                    clip_x: $clip_x_value,
                    clip_y: $clip_y_value,
                    align: $align_value,
                    cursor,
                    flow: $flow_value,
                    spacing: $spacing_value,
                    height: $height_value,
                    width: $width_value,
                    abs_pos: $abs_pos_value,
                }
            }

            state_colors! {
                (bg_level, border_level, shadow_level),
                $($state_path => ($($level_number),*)),*
            }

            fn live_props() -> LiveProps {
                vec![
                    (live_id!(theme), None.into()),
                    (live_id!(background_color), None.into()),
                    (live_id!(border_color), None.into()),
                    (live_id!(border_width), None.into()),
                    (
                        live_id!(border_radius),
                        Some(vec![
                            live_id!(top),
                            live_id!(bottom),
                            live_id!(left),
                            live_id!(right),
                        ])
                        .into(),
                    ),
                    (live_id!(shadow_color), None.into()),
                    (live_id!(spread_radius), None.into()),
                    (live_id!(blur_radius), None.into()),
                    (live_id!(shadow_offset), None.into()),
                    (live_id!(background_visible), None.into()),
                    (live_id!(rotation), None.into()),
                    (live_id!(scale), None.into()),
                    (
                        live_id!(padding),
                        Some(vec![
                            live_id!(top),
                            live_id!(bottom),
                            live_id!(left),
                            live_id!(right),
                        ])
                        .into(),
                    ),
                    (
                        live_id!(margin),
                        Some(vec![
                            live_id!(top),
                            live_id!(bottom),
                            live_id!(left),
                            live_id!(right),
                        ])
                        .into(),
                    ),
                    (live_id!(clip_x), None.into()),
                    (live_id!(clip_y), None.into()),
                    (live_id!(align), Some(vec![live_id!(x), live_id!(y)]).into()),
                    (live_id!(cursor), None.into()),
                    (live_id!(flow), None.into()),
                    (live_id!(spacing), None.into()),
                    (live_id!(height), None.into()),
                    (live_id!(width), None.into()),
                    (live_id!(abs_pos), None.into()),
                ]
            }

            fn walk(&self) -> Walk {
                Walk {
                    abs_pos: self.abs_pos,
                    margin: self.margin,
                    width: self.width,
                    height: self.height,
                }
            }

            fn layout(&self) -> Layout {
                Layout {
                    clip_x: self.clip_x,
                    clip_y: self.clip_y,
                    padding: self.padding,
                    align: self.align,
                    flow: self.flow,
                    spacing: self.spacing,
                    ..Default::default()
                }
            }
        }
    };
}

#[macro_export]
macro_rules! from_inherit_to_view_basic_prop {
    ($struct_name: ident) => {
        impl From<$struct_name> for ViewBasicStyle {
            fn from(value: $struct_name) -> Self {
                Self {
                    theme: value.theme,
                    background_color: value.background_color,
                    border_color: value.border_color,
                    border_width: value.border_width,
                    border_radius: value.border_radius,
                    shadow_color: value.shadow_color,
                    spread_radius: value.spread_radius,
                    blur_radius: value.blur_radius,
                    shadow_offset: value.shadow_offset,
                    background_visible: value.background_visible,
                    rotation: value.rotation,
                    scale: value.scale,
                    padding: value.padding,
                    margin: value.margin,
                    clip_x: value.clip_x,
                    clip_y: value.clip_y,
                    align: value.align,
                    cursor: value.cursor,
                    flow: value.flow,
                    spacing: value.spacing,
                    height: value.height,
                    width: value.width,
                    abs_pos: value.abs_pos,
                }
            }
        }
    };
}

#[macro_export]
macro_rules! impl_view_trait_widget_node {
    ($struct_name: ident, $draw: ident) => {
        impl WidgetNode for $struct_name {
            fn walk(&mut self, _cx: &mut Cx) -> Walk {
                let style = self.style.get(self.state);
                style.walk()
            }

            fn area(&self) -> Area {
                self.area
            }

            fn uid_to_widget(&self, uid: WidgetUid) -> WidgetRef {
                for (_, child) in &self.children {
                    let x = child.uid_to_widget(uid);
                    if !x.is_empty() {
                        return x;
                    }
                }
                WidgetRef::empty()
            }

            fn redraw(&mut self, cx: &mut Cx) {
                let _ = self.render(cx);
                self.area.redraw(cx);
                self.$draw.redraw(cx);
                for (_, child) in &mut self.children {
                    child.redraw(cx);
                }
            }
            fn find_widgets(&self, path: &[LiveId], cached: WidgetCache, results: &mut WidgetSet) {
                match cached {
                    WidgetCache::Yes | WidgetCache::Clear => {
                        if let WidgetCache::Clear = cached {
                            self.find_cache.borrow_mut().clear();
                            if path.len() == 0 {
                                return;
                            }
                        }
                        let mut hash = 0u64;
                        for i in 0..path.len() {
                            hash ^= path[i].0
                        }
                        if let Some((_, widget_set)) =
                            self.find_cache.borrow().iter().find(|(h, _v)| h == &hash)
                        {
                            results.extend_from_set(widget_set);
                            return;
                        }
                        let mut local_results = WidgetSet::empty();
                        if let Some((_, child)) = self.children.iter().find(|(id, _)| *id == path[0]) {
                            if path.len() > 1 {
                                child.find_widgets(&path[1..], WidgetCache::No, &mut local_results);
                            } else {
                                local_results.push(child.clone());
                            }
                        }
                        for (_, child) in &self.children {
                            child.find_widgets(path, WidgetCache::No, &mut local_results);
                        }
                        if !local_results.is_empty() {
                            results.extend_from_set(&local_results);
                        }
                        self.find_cache.borrow_mut().push((hash, local_results));
                    }
                    WidgetCache::No => {
                        if let Some((_, child)) = self.children.iter().find(|(id, _)| *id == path[0]) {
                            if path.len() > 1 {
                                child.find_widgets(&path[1..], WidgetCache::No, results);
                            } else {
                                results.push(child.clone());
                            }
                        }
                        for (_, child) in &self.children {
                            child.find_widgets(path, WidgetCache::No, results);
                        }
                    }
                }
            }

            fn animation_spread(&self) -> bool {
                self.animation_spread
            }

            fn state(&self) -> String {
                self.state.to_string()
            }

            crate::visible!();
        }
    };
}

#[macro_export]
macro_rules! impl_view_trait_live_hook {
    () => {
        fn before_apply(
            &mut self,
            _cx: &mut Cx,
            apply: &mut Apply,
            _index: usize,
            _nodes: &[LiveNode],
        ) {
            if let ApplyFrom::UpdateFromDoc { .. } = apply.from {
                //self.draw_order.clear();
                self.live_update_order.clear();
                self.find_cache.get_mut().clear();
            }
        }
        fn apply_value_instance(
            &mut self,
            cx: &mut Cx,
            apply: &mut Apply,
            index: usize,
            nodes: &[LiveNode],
        ) -> usize {
            let id = nodes[index].id;
            match apply.from {
                ApplyFrom::Animate | ApplyFrom::Over => {
                    let node_id = nodes[index].id;
                    if let Some((_, component)) =
                        self.children.iter_mut().find(|(id, _)| *id == node_id)
                    {
                        component.apply(cx, apply, index, nodes)
                    } else {
                        nodes.skip_node(index)
                    }
                }
                ApplyFrom::NewFromDoc { .. } | ApplyFrom::UpdateFromDoc { .. } => {
                    if nodes[index].is_instance_prop() {
                        if apply.from.is_update_from_doc() {
                            //livecoding
                            self.live_update_order.push(id);
                        }
                        //self.draw_order.push(id);
                        if let Some((_, node)) = self.children.iter_mut().find(|(id2, _)| *id2 == id) {
                            node.apply(cx, apply, index, nodes)
                        } else {
                            self.children.push((id, WidgetRef::new(cx)));
                            self.children
                                .last_mut()
                                .unwrap()
                                .1
                                .apply(cx, apply, index, nodes)
                        }
                    } else {
                        cx.apply_error_no_matching_field(live_error_origin!(), index, nodes);
                        nodes.skip_node(index)
                    }
                }
                _ => nodes.skip_node(index),
            }
        }
    };
}