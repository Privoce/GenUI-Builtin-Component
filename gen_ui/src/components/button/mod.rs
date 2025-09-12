mod event;
mod prop;

pub use event::*;
use makepad_widgets::*;
pub use prop::*;

use crate::{
    active_event, animation_open_then_redraw, area, area_ref,
    components::{
        lifecycle::LifeCycle,
        traits::{BasicStyle, Component, Style},
    },
    error::Error,
    event_option, event_option_ref, getter, getter_setter_ref, hit_finger_down, hit_finger_up,
    hit_hover_in, hit_hover_out, lifecycle, play_animation,
    prop::{
        manuel::{BASIC, DISABLED, HOVER, PRESSED},
        traits::{ToColor, ToFloat},
        ApplyStateMap, Radius,
    },
    pure_after_apply, set_animation, set_index, set_scope_path, setter,
    shader::draw_view::DrawView,
    sync,
    themes::{conf::Conf, Theme},
    visible, ComponentAnInit,
};

live_design! {
    link genui_basic;
    use link::genui_animation_prop::*;

    pub GButtonBase = {{GButton}} {
        animator: {
            hover = {
                default: off,

                off = {
                    from: {all: Forward {duration: (AN_DURATION)}},
                    ease: InOutQuad,
                    apply: {
                        draw_button: <AN_DRAW_VIEW> {}
                    }
                }

                on = {
                    from: {
                        all: Forward {duration: (AN_DURATION),},
                        pressed: Forward {duration: (AN_DURATION)},
                    },
                    ease: InOutQuad,
                    apply: {
                       draw_button: <AN_DRAW_VIEW> {}
                    }
                }

                pressed = {
                    from: {all: Forward {duration: (AN_DURATION)}},
                    ease: InOutQuad,
                    apply: {
                        draw_button: <AN_DRAW_VIEW> {}
                    }
                }

                disabled = {
                    from: {all: Forward {duration: (AN_DURATION)}},
                    ease: InOutQuad,
                    apply: {
                        draw_button: <AN_DRAW_VIEW> {}
                    }
                }
            }
        }
    }
}

#[derive(Live, WidgetRef, WidgetSet, LiveRegisterWidget)]
pub struct GButton {
    #[live]
    pub style: ButtonStyle,
    // --- visible -------------------
    #[live(true)]
    pub visible: bool,
    // --- others -------------------
    #[live]
    pub disabled: bool,
    #[live]
    pub grab_key_focus: bool,
    #[live(true)]
    pub event_open: bool,
    #[rust]
    pub scope_path: Option<HeapLiveIdPath>,
    #[rust]
    pub apply_state_map: ApplyStateMap<ButtonState>,
    // --- draw ----------------------
    #[live]
    pub slot: WidgetRef,
    #[live]
    pub draw_button: DrawView,
    // --- animator ----------------
    #[live(true)]
    pub animation_open: bool,
    #[animator]
    pub animator: Animator,
    #[live(true)]
    pub animation_spread: bool,
    // --- init ----------------------
    #[rust]
    pub lifecycle: LifeCycle,
    #[rust]
    index: usize,
    /// sync other state props (except related to theme) from `basic` state]
    /// means: if you set basic prop that `border_radius: 10.0`, then other state like `hover` or `pressed`
    /// will have the same `border_radius: 10.0` if you set this to true. (default is true)
    #[live(true)]
    pub sync: bool,
    #[rust]
    pub state: ButtonState,
}

impl WidgetNode for GButton {
    fn uid_to_widget(&self, uid: WidgetUid) -> WidgetRef {
        self.slot.uid_to_widget(uid)
    }

    fn find_widgets(&self, path: &[LiveId], cached: WidgetCache, results: &mut WidgetSet) {
        self.slot.find_widgets(path, cached, results);
    }

    fn walk(&mut self, _cx: &mut Cx) -> Walk {
        let style = self.style.get(self.state);
        style.walk()
    }

    fn area(&self) -> Area {
        self.draw_button.area
    }

    fn redraw(&mut self, cx: &mut Cx) {
        let _ = self.render(cx);
        self.draw_button.redraw(cx);
        if self.slot.visible() {
            self.slot.redraw(cx);
        }
    }

    fn state(&self) -> String {
        self.state.to_string()
    }

    fn animation_spread(&self) -> bool {
        self.animation_spread
    }

    visible!();
}

impl Widget for GButton {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        if !self.visible {
            return DrawStep::done();
        }

        let state = self.state;
        let style = self.style.get(state);
        let _ = self.draw_button.begin(cx, walk, style.layout());

        if self.slot.visible() {
            let slot_walk = self.slot.walk(cx);
            let _ = self.slot.draw_walk(cx, scope, slot_walk);
        }

        self.draw_button.end(cx);
        self.set_scope_path(&scope.path);
        DrawStep::done()
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, _scope: &mut Scope) {
        if !self.visible {
            return;
        }

        self.set_animation(cx);
        cx.global::<ComponentAnInit>().button = true;
        let area = self.area();
        let hit = event.hits(cx, area);
        if self.disabled {
            self.handle_when_disabled(cx, event, hit);
        } else {
            self.handle_widget_event(cx, event, hit, area);
        }
    }

    fn handle_event_with(
        &mut self,
        cx: &mut Cx,
        event: &Event,
        _scope: &mut Scope,
        sweep_area: Area,
    ) {
        if !self.visible {
            return;
        }

        self.set_animation(cx);
        cx.global::<ComponentAnInit>().button = true;
        let hit = event.hits_with_options(
            cx,
            self.area(),
            HitOptions::new().with_sweep_area(sweep_area),
        );
        if self.disabled {
            self.handle_when_disabled(cx, event, hit);
        } else {
            self.handle_widget_event(cx, event, hit, sweep_area);
        }
    }
}

impl LiveHook for GButton {
    pure_after_apply!();

    fn after_new_before_apply(&mut self, cx: &mut Cx) {
        self.merge_conf_prop(cx);
    }

    fn after_apply(&mut self, _cx: &mut Cx, _apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.set_apply_state_map(
            nodes,
            index,
            &ButtonBasicStyle::live_props(),
            [
                live_id!(basic),
                live_id!(hover),
                live_id!(pressed),
                live_id!(disabled),
            ],
            |_| {},
            |prefix, component, applys| match prefix.to_string().as_str() {
                BASIC => {
                    component.apply_state_map.insert(ButtonState::Basic, applys);
                }
                HOVER => {
                    component.apply_state_map.insert(ButtonState::Hover, applys);
                }
                PRESSED => {
                    component
                        .apply_state_map
                        .insert(ButtonState::Pressed, applys);
                }
                DISABLED => {
                    component
                        .apply_state_map
                        .insert(ButtonState::Disabled, applys);
                }
                _ => {}
            },
        );
    }
}

impl Component for GButton {
    type Error = Error;

    type State = ButtonState;

    fn merge_conf_prop(&mut self, cx: &mut Cx) -> () {
        let style = &cx.global::<Conf>().components.button;
        self.style = style.clone();
    }

    fn render(&mut self, _cx: &mut Cx) -> Result<(), Self::Error> {
        if self.disabled {
            self.switch_state(ButtonState::Disabled);
        }
        let style = self.style.get(self.state);
        self.draw_button.merge(&style.into());
        Ok(())
    }

    fn handle_when_disabled(&mut self, cx: &mut Cx, _event: &Event, hit: Hit) -> () {
        match hit {
            Hit::FingerHoverIn(_) => {
                self.switch_state_and_redraw(cx, ButtonState::Disabled);
                cx.set_cursor(self.style.get(self.state).cursor);
            }
            _ => {}
        }
    }

    fn handle_widget_event(&mut self, cx: &mut Cx, event: &Event, hit: Hit, area: Area) {
        animation_open_then_redraw!(self, cx, event);

        match hit {
            Hit::FingerDown(e) => {
                self.switch_state_with_animation(cx, ButtonState::Pressed);
                hit_finger_down!(self, cx, area, e);
            }
            Hit::FingerHoverIn(e) => {
                cx.set_cursor(self.style.get(self.state).cursor);
                self.switch_state_with_animation(cx, ButtonState::Hover);
                hit_hover_in!(self, cx, e);
            }
            Hit::FingerHoverOut(e) => {
                self.switch_state_with_animation(cx, ButtonState::Basic);
                hit_hover_out!(self, cx, e);
            }
            Hit::FingerUp(e) => {
                if e.is_over {
                    if e.has_hovers() {
                        self.switch_state_with_animation(cx, ButtonState::Hover);
                        self.play_animation(cx, id!(hover.on));
                    } else {
                        self.switch_state_with_animation(cx, ButtonState::Basic);
                        self.play_animation(cx, id!(hover.off));
                    }
                    self.active_clicked(cx, e);
                } else {
                    self.switch_state_with_animation(cx, ButtonState::Basic);
                    hit_finger_up!(self, cx, e);
                }
            }
            _ => {}
        };
    }

    fn switch_state(&mut self, state: Self::State) -> () {
        self.state = state;
    }

    fn switch_state_with_animation(&mut self, cx: &mut Cx, state: Self::State) -> () {
        if !self.animation_open {
            return;
        }
        self.switch_state(state);
        self.set_animation(cx);
        self.redraw(cx);
    }

    // sync props if not set in DSL, depend on `self.sync` is true
    fn focus_sync(&mut self) -> () {
        self.style.sync(&self.apply_state_map);
    }

    fn set_animation(&mut self, cx: &mut Cx) -> () {
        let init_global = cx.global::<ComponentAnInit>().button;

        let live_ptr = match self.animator.live_ptr {
            Some(ptr) => ptr.file_id.0,
            None => return,
        };

        let mut registry = cx.live_registry.borrow_mut();
        let live_file = match registry.live_files.get_mut(live_ptr as usize) {
            Some(lf) => lf,
            None => return,
        };

        let nodes = &mut live_file.expanded.nodes;

        if self.lifecycle.is_created() || !init_global || self.scope_path.is_none() {
            self.lifecycle.next();
            let basic_prop = self.style.get(ButtonState::Basic);
            let hover_prop = self.style.get(ButtonState::Hover);
            let pressed_prop = self.style.get(ButtonState::Pressed);
            let disabled_prop = self.style.get(ButtonState::Disabled);
            let (mut basic_index, mut hover_index, mut pressed_index, mut disabled_index) =
                (None, None, None, None);
            if let Some(index) = nodes.child_by_path(
                self.index,
                &[
                    live_id!(animator).as_field(),
                    live_id!(hover).as_instance(),
                    live_id!(off).as_instance(),
                ],
            ) {
                basic_index = Some(index);
            }

            if let Some(index) = nodes.child_by_path(
                self.index,
                &[
                    live_id!(animator).as_field(),
                    live_id!(hover).as_instance(),
                    live_id!(on).as_instance(),
                ],
            ) {
                hover_index = Some(index);
            }

            if let Some(index) = nodes.child_by_path(
                self.index,
                &[
                    live_id!(animator).as_field(),
                    live_id!(hover).as_instance(),
                    live_id!(pressed).as_instance(),
                ],
            ) {
                pressed_index = Some(index);
            }

            if let Some(index) = nodes.child_by_path(
                self.index,
                &[
                    live_id!(animator).as_field(),
                    live_id!(hover).as_instance(),
                    live_id!(disabled).as_instance(),
                ],
            ) {
                disabled_index = Some(index);
            }

            set_animation! {
                nodes: draw_button = {
                    basic_index => {
                        background_color => basic_prop.background_color,
                        border_color =>basic_prop.border_color,
                        border_radius => basic_prop.border_radius,
                        border_width =>(basic_prop.border_width as f64),
                        shadow_color => basic_prop.shadow_color,
                        spread_radius => (basic_prop.spread_radius as f64),
                        blur_radius => (basic_prop.blur_radius as f64),
                        shadow_offset => basic_prop.shadow_offset,
                        background_visible => basic_prop.background_visible.to_f64()
                    },
                    hover_index => {
                        background_color => hover_prop.background_color,
                        border_color => hover_prop.border_color,
                        border_radius => hover_prop.border_radius,
                        border_width => (hover_prop.border_width as f64),
                        shadow_color => hover_prop.shadow_color,
                        spread_radius => (hover_prop.spread_radius as f64),
                        blur_radius => (hover_prop.blur_radius as f64),
                        shadow_offset => hover_prop.shadow_offset,
                        background_visible => hover_prop.background_visible.to_f64()
                    },
                    pressed_index => {
                        background_color => pressed_prop.background_color,
                        border_color => pressed_prop.border_color,
                        border_radius => pressed_prop.border_radius,
                        border_width => (pressed_prop.border_width as f64),
                        shadow_color => pressed_prop.shadow_color,
                        spread_radius => (pressed_prop.spread_radius as f64),
                        blur_radius => (pressed_prop.blur_radius as f64),
                        shadow_offset => pressed_prop.shadow_offset,
                        background_visible => pressed_prop.background_visible.to_f64()
                    },
                    disabled_index => {
                        background_color => disabled_prop.background_color,
                        border_color => disabled_prop.border_color,
                        border_radius => disabled_prop.border_radius,
                        border_width => (disabled_prop.border_width as f64),
                        shadow_color => disabled_prop.shadow_color,
                        spread_radius => (disabled_prop.spread_radius as f64),
                        blur_radius => (disabled_prop.blur_radius as f64),
                        shadow_offset => disabled_prop.shadow_offset,
                        background_visible => disabled_prop.background_visible.to_f64()
                    }
                }
            }
        } else {
            let state = self.state;
            let style = self.style.get(state);
            let index = match state {
                ButtonState::Basic => nodes.child_by_path(
                    self.index,
                    &[
                        live_id!(animator).as_field(),
                        live_id!(hover).as_instance(),
                        live_id!(off).as_instance(),
                    ],
                ),
                ButtonState::Hover => nodes.child_by_path(
                    self.index,
                    &[
                        live_id!(animator).as_field(),
                        live_id!(hover).as_instance(),
                        live_id!(on).as_instance(),
                    ],
                ),
                ButtonState::Pressed => nodes.child_by_path(
                    self.index,
                    &[
                        live_id!(animator).as_field(),
                        live_id!(hover).as_instance(),
                        live_id!(pressed).as_instance(),
                    ],
                ),
                ButtonState::Disabled => nodes.child_by_path(
                    self.index,
                    &[
                        live_id!(animator).as_field(),
                        live_id!(hover).as_instance(),
                        live_id!(disabled).as_instance(),
                    ],
                ),
            };
            set_animation! {
                nodes: draw_button = {
                    index => {
                        background_color => style.background_color,
                        border_color => style.border_color,
                        border_radius => style.border_radius,
                        border_width => (style.border_width as f64),
                        shadow_color => style.shadow_color,
                        spread_radius => (style.spread_radius as f64),
                        blur_radius => (style.blur_radius as f64),
                        shadow_offset => style.shadow_offset,
                        background_visible => style.background_visible.to_f64()
                    }
                }
            }
        }
    }

    sync!();
    play_animation!();
    set_scope_path!();
    set_index!();
    lifecycle!();
}

impl GButton {
    active_event! {
        active_hover_in: ButtonEvent::HoverIn |meta: FingerHoverEvent| => ButtonHoverIn { meta },
        active_hover_out: ButtonEvent::HoverOut |meta: FingerHoverEvent| => ButtonHoverOut { meta },
        active_finger_up: ButtonEvent::FingerUp |meta: FingerUpEvent| => ButtonFingerUp { meta },
        active_finger_down: ButtonEvent::FingerDown |meta: FingerDownEvent| => ButtonFingerDown { meta },
        active_clicked: ButtonEvent::Clicked |meta: FingerUpEvent| => ButtonClicked { meta }
    }
    event_option! {
        hover_in: ButtonEvent::HoverIn => ButtonHoverIn,
        hover_out: ButtonEvent::HoverOut => ButtonHoverOut,
        finger_up: ButtonEvent::FingerUp => ButtonFingerUp,
        finger_down: ButtonEvent::FingerDown => ButtonFingerDown,
        clicked: ButtonEvent::Clicked => ButtonClicked
    }
    area! {
        area_slot, slot
    }
    getter! {
        GButton {
            get_theme(Theme) {|c| {c.style.basic.get_theme()}},
            get_background_color(String) {|c| {c.style.basic.get_background_color().to_hex_string()}},
            get_background_visible(bool) {|c| {c.style.basic.get_background_visible()}},
            get_shadow_color(String) {|c| {c.style.basic.get_shadow_color().to_hex_string()}},
            get_border_color(String) {|c| {c.style.basic.get_border_color().to_hex_string()}},
            get_border_radius(Radius) {|c| {c.style.basic.get_border_radius()}},
            get_border_width(f32) {|c| {c.style.basic.get_border_width()}},
            get_spread_radius(f32) {|c| {c.style.basic.get_spread_radius()}},
            get_blur_radius(f32) {|c| {c.style.basic.get_blur_radius()}},
            get_shadow_offset(Vec2) {|c| {c.style.basic.get_shadow_offset()}},
            get_margin(Margin) {|c| {c.style.basic.get_margin()}},
            get_padding(Padding) {|c| {c.style.basic.get_padding()}},
            get_width(Size) {|c| {c.style.basic.get_width()}},
            get_height(Size) {|c| {c.style.basic.get_height()}},
            get_cursor(MouseCursor) {|c| {c.style.basic.get_cursor()}},
            get_flow(Flow) {|c| {c.style.basic.get_flow()}},
            get_align(Align) {|c| {c.style.basic.get_align()}},
            get_spacing(f64) {|c| {c.style.basic.get_spacing()}},
            get_disabled(bool) {|c| {c.disabled}},
            get_visible(bool) {|c| {c.visible}},
            get_grab_key_focus(bool) {|c| {c.grab_key_focus}},
            get_sync(bool) {|c| {c.sync}},
            get_event_open(bool) {|c| {c.event_open}},
            get_abs_pos(Option<DVec2>) {|c| {c.style.basic.get_abs_pos()}}
        }
    }
    setter! {
        GButton {
            set_theme(theme: Theme) {|c, _cx| {c.style.basic.set_theme(theme); c.style.basic.sync(ButtonState::Basic); Ok(())}},
            set_background_color(color: String) {|c, _cx| {let color = Vec4::from_hex(&color)?; c.style.basic.set_background_color(color); Ok(())}},
            set_background_visible(visible: bool) {|c, _cx| {c.style.basic.set_background_visible(visible); Ok(())}},
            set_shadow_color(color: String) {|c, _cx| {let color = Vec4::from_hex(&color)?; c.style.basic.set_shadow_color(color); Ok(())}},
            set_border_color(color: String) {|c, _cx| {let color = Vec4::from_hex(&color)?; c.style.basic.set_border_color(color); Ok(())}},
            set_border_radius(radius: Radius) {|c, _cx| {c.style.basic.set_border_radius(radius); Ok(())}},
            set_border_width(width: f32) {|c, _cx| {c.style.basic.set_border_width(width); Ok(())}},
            set_spread_radius(radius: f32) {|c, _cx| {c.style.basic.set_spread_radius(radius); Ok(())}},
            set_blur_radius(radius: f32) {|c, _cx| {c.style.basic.set_blur_radius(radius); Ok(())}},
            set_shadow_offset(offset: Vec2) {|c, _cx| {c.style.basic.set_shadow_offset(offset); Ok(())}},
            set_margin(margin: Margin) {|c, _cx| {c.style.basic.set_margin(margin); Ok(())}},
            set_padding(padding: Padding) {|c, _cx| {c.style.basic.set_padding(padding); Ok(())}},
            set_width(width: Size) {|c, _cx| {c.style.basic.set_width(width); Ok(())}},
            set_height(height: Size) {|c, _cx| {c.style.basic.set_height(height); Ok(())}},
            set_cursor(cursor: MouseCursor) {|c, _cx| {c.style.basic.set_cursor(cursor); Ok(())}},
            set_flow(flow: Flow) {|c, _cx| {c.style.basic.set_flow(flow); Ok(())}},
            set_align(align: Align) {|c, _cx| {c.style.basic.set_align(align); Ok(())}},
            set_spacing(spacing: f64) {|c, _cx| {c.style.basic.set_spacing(spacing); Ok(())}},
            set_disabled(disabled: bool) {|c, _cx| {c.disabled = disabled; Ok(())}},
            set_visible(visible: bool) {|c, _cx| {c.visible = visible; c.redraw(_cx); Ok(())}},
            set_grab_key_focus(grab: bool) {|c, _cx| {c.grab_key_focus = grab; Ok(())}},
            set_sync(sync: bool) {|c, _cx| {c.sync = sync; c.style.basic.sync(ButtonState::Basic); Ok(())}},
            set_event_open(open: bool) {|c, _cx| {c.event_open = open; Ok(())}},
            set_abs_pos(abs_pos: Option<DVec2>) {|c, _cx| {c.style.basic.set_abs_pos(abs_pos); Ok(())}}
        }
    }
}

impl GButtonRef {
    event_option_ref! {
        hover_in => ButtonHoverIn,
        hover_out => ButtonHoverOut,
        finger_up => ButtonFingerUp,
        finger_down => ButtonFingerDown,
        clicked => ButtonClicked
    }
    area_ref! {
        area_slot
    }
    getter_setter_ref! {
        get_theme, set_theme -> Theme,
        get_background_color, set_background_color -> String,
        get_background_visible, set_background_visible -> bool,
        get_shadow_color, set_shadow_color -> String,
        get_border_color, set_border_color -> String,
        get_border_radius, set_border_radius -> Radius,
        get_border_width, set_border_width -> f32,
        get_spread_radius, set_spread_radius -> f32,
        get_blur_radius, set_blur_radius -> f32,
        get_shadow_offset, set_shadow_offset -> Vec2,
        get_margin, set_margin -> Margin,
        get_padding, set_padding -> Padding,
        get_width, set_width -> Size,
        get_height, set_height -> Size,
        get_cursor, set_cursor -> MouseCursor,
        get_flow, set_flow -> Flow,
        get_align, set_align -> Align,
        get_spacing, set_spacing -> f64,
        get_disabled, set_disabled -> bool,
        get_visible, set_visible -> bool,
        get_grab_key_focus, set_grab_key_focus -> bool,
        get_sync, set_sync -> bool,
        get_event_open, set_event_open -> bool,
        get_abs_pos, set_abs_pos -> Option<DVec2>
    }
}
