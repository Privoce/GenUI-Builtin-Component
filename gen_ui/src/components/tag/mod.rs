mod event;
mod prop;

pub use event::*;
use makepad_widgets::*;
pub use prop::*;

use crate::{
    active_event, animation_open_then_redraw,
    components::{
        label::{GLabel, LabelBasicStyle},
        lifecycle::LifeCycle,
        svg::{GSvg, SvgBasicStyle},
        traits::{BasicStyle, Component, Style, SlotComponent, SlotStyle},
        view::ViewBasicStyle,
    },
    error::Error,
    event_option, event_option_ref, hit_finger_down, hit_hover_in, hit_hover_out, lifecycle,
    play_animation,
    prop::{
        manuel::{BASIC, DISABLED, HOVER, PRESSED},
        traits::{RectExp, ToFloat},
        ApplyMapImpl, ApplySlotMap, ApplySlotMapImpl, ApplySlotMergeImpl, DeferWalks, SlotDrawer,
        ToSlotMap, ToStateMap,
    },
    pure_after_apply, set_animation, set_index, set_scope_path,
    shader::draw_view::DrawView,
    sync,
    themes::conf::Conf,
    visible, ComponentAnInit,
};

live_design! {
    link genui_basic;
    use link::genui_animation_prop::*;

    pub GTagBase = {{GTag}}{
        animator: {
            hover = {
                default: off,
                off = {
                    from: {all: Forward {duration: (AN_DURATION)}}
                    apply: {
                        draw_tag: <AN_DRAW_VIEW> {}
                    }
                }

                on = {
                    from: {
                        all: Forward {duration: (AN_DURATION),},
                        pressed: Forward {duration: (AN_DURATION)},
                    },
                    ease: InOutQuad,
                    apply: {
                        draw_tag: <AN_DRAW_VIEW> {}
                    }
                }

                pressed = {
                    from: {all: Forward {duration: (AN_DURATION)}},
                    ease: InOutQuad,
                    apply: {
                        draw_tag: <AN_DRAW_VIEW> {}
                    }
                }

                disabled = {
                    from: {all: Forward {duration: (AN_DURATION)}},
                    ease: InOutQuad,
                    apply: {
                        draw_tag: <AN_DRAW_VIEW> {}
                    }
                }
            }
        }
    }
}

#[derive(Live, WidgetRef, WidgetSet, LiveRegisterWidget)]
pub struct GTag {
    #[live]
    pub style: TagProp,
    // --- draw ----------------------
    #[live]
    pub draw_tag: DrawView,
    // --- slots ----------------------
    #[live]
    pub icon: GSvg,
    #[live]
    pub text: GLabel,
    #[live]
    pub close: GSvg,
    // --- other ----------------------
    #[live(false)]
    pub disabled: bool,
    #[live(false)]
    pub grab_key_focus: bool,
    #[rust]
    pub apply_slot_map: ApplySlotMap<TagState, TagPart>,
    // visible -------------------
    #[live(true)]
    pub visible: bool,
    // animator -----------------
    #[live(true)]
    pub animation_open: bool,
    #[animator]
    animator: Animator,
    #[live(true)]
    pub event_open: bool,
    #[rust]
    pub scope_path: Option<HeapLiveIdPath>,
    // --- init ----------------------
    #[rust]
    pub lifecycle: LifeCycle,
    #[rust]
    index: usize,
    #[live(true)]
    pub sync: bool,
    #[rust]
    defer_walks: DeferWalks,
    #[rust]
    pub state: TagState,
}

impl WidgetNode for GTag {
    fn uid_to_widget(&self, uid: WidgetUid) -> WidgetRef {
        let icon_ref = self.icon.uid_to_widget(uid);
        let text_ref = self.text.uid_to_widget(uid);
        let close_ref = self.close.uid_to_widget(uid);

        if !icon_ref.is_empty() {
            return icon_ref;
        }
        if !text_ref.is_empty() {
            return text_ref;
        }
        if !close_ref.is_empty() {
            return close_ref;
        }
        WidgetRef::empty()
    }

    fn find_widgets(&self, _path: &[LiveId], _cached: WidgetCache, _results: &mut WidgetSet) {
        ()
    }

    fn walk(&mut self, _cx: &mut Cx) -> Walk {
        let style = self.style.get(self.state);
        style.container.walk()
    }

    fn area(&self) -> Area {
        self.draw_tag.area
    }

    fn redraw(&mut self, cx: &mut Cx) {
        let _ = self.render(cx);
        if self.icon.visible {
            self.icon.redraw(cx);
        }
        if self.text.visible {
            self.text.redraw(cx);
        }
        if self.close.visible {
            self.close.redraw(cx);
        }
        self.draw_tag.redraw(cx);
    }

    fn state(&self) -> String {
        self.state.to_string()
    }

    visible!();
}

impl LiveHook for GTag {
    pure_after_apply!();
    fn after_new_before_apply(&mut self, cx: &mut Cx) {
        self.merge_conf_prop(cx);
    }
    fn after_apply(&mut self, _cx: &mut Cx, _apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.set_apply_slot_map(
            nodes,
            index,
            [
                live_id!(basic),
                live_id!(hover),
                live_id!(pressed),
                live_id!(disabled),
            ],
            [
                (TagPart::Icon, &SvgBasicStyle::live_props()),
                (TagPart::Text, &LabelBasicStyle::live_props()),
                (TagPart::Close, &SvgBasicStyle::live_props()),
                (TagPart::Container, &ViewBasicStyle::live_props()),
            ],
            |_| {},
            |prefix, component, applys| match prefix.to_string().as_str() {
                BASIC => {
                    component.apply_slot_map.insert(TagState::Basic, applys);
                }
                HOVER => {
                    component.apply_slot_map.insert(TagState::Hover, applys);
                }
                PRESSED => {
                    component.apply_slot_map.insert(TagState::Pressed, applys);
                }
                DISABLED => {
                    component.apply_slot_map.insert(TagState::Disabled, applys);
                }
                _ => {}
            },
        );
    }
}

impl SlotComponent<TagState> for GTag {
    type Part = TagPart;
}

impl Component for GTag {
    type Error = Error;

    type State = TagState;

    fn merge_conf_prop(&mut self, cx: &mut Cx) -> () {
        let style = &cx.global::<Conf>().components.tag;
        self.style = style.clone();
        self.icon.style.basic = self.style.basic.icon;
        self.icon.style.hover = self.style.hover.icon;
        self.icon.style.pressed = self.style.pressed.icon;
        self.icon.style.disabled = self.style.disabled.icon;
        self.text.style.basic = self.style.basic.text;
        self.text.style.disabled = self.style.disabled.text;
        self.close.style.basic = self.style.basic.close;
        self.close.style.hover = self.style.hover.close;
        self.close.style.pressed = self.style.pressed.close;
        self.close.style.disabled = self.style.disabled.close;
    }

    fn render(&mut self, cx: &mut Cx) -> Result<(), Self::Error> {
        let state = if self.disabled {
            TagState::Disabled
        } else {
            TagState::Basic
        };
        self.switch_state(state);
        let style = self.style.get(self.state);
        self.draw_tag.merge(&style.container);
        let _ = self.icon.render(cx)?;
        let _ = self.text.render(cx)?;
        let _ = self.close.render(cx)?;
        Ok(())
    }

    fn handle_widget_event(&mut self, cx: &mut Cx, event: &Event, hit: Hit, area: Area) {
        animation_open_then_redraw!(self, cx, event);
        match hit {
            Hit::FingerHoverIn(e) => {
                self.switch_state_with_animation(cx, TagState::Hover);
                cx.set_cursor(self.style.get(self.state).container.cursor);
                hit_hover_in!(self, cx, e);
            }
            Hit::FingerHoverOut(e) => {
                self.switch_state_with_animation(cx, TagState::Basic);
                hit_hover_out!(self, cx, e);
            }
            Hit::FingerDown(e) => {
                self.switch_state_with_animation(cx, TagState::Pressed);
                hit_finger_down!(self, cx, area, e);
            }
            Hit::FingerUp(e) => {
                if e.is_over {
                    if e.has_hovers() {
                        self.switch_state_with_animation(cx, TagState::Hover);
                        self.play_animation(cx, id!(hover.on));
                    } else {
                        self.switch_state_with_animation(cx, TagState::Basic);
                        self.play_animation(cx, id!(hover.off));
                    }
                    if self.area_close().rect(cx).is_in_pos(&e.abs) {
                        self.visible = false;
                        self.active_close(cx, e);
                        return;
                    }
                    self.active_clicked(cx, e);
                } else {
                    self.switch_state_with_animation(cx, TagState::Basic);
                }
            }
            _ => {}
        }
    }

    fn handle_when_disabled(&mut self, cx: &mut Cx, _event: &Event, hit: Hit) -> () {
        match hit {
            Hit::FingerHoverIn(_) => {
                self.switch_state_and_redraw(cx, TagState::Disabled);
                cx.set_cursor(self.style.get(self.state).container.cursor);
            }
            _ => {}
        }
    }

    fn switch_state(&mut self, state: Self::State) -> () {
        self.state = state;
        self.icon.switch_state(state.into());
        self.text.switch_state(state.into());
        self.close.switch_state(state.into());
    }

    fn switch_state_with_animation(&mut self, cx: &mut Cx, state: Self::State) -> () {
        if !self.animation_open {
            return;
        }
        self.switch_state(state);
        self.set_animation(cx);
    }

    fn focus_sync(&mut self) -> () {
        let mut crossed_map = self.apply_slot_map.cross();

        crossed_map.remove(&TagPart::Icon).map(|map| {
            self.icon.apply_slot_map.merge_slot(map.to_slot());
            self.icon.focus_sync();
        });

        crossed_map.remove(&TagPart::Text).map(|map| {
            self.text.apply_state_map.merge(map.to_state());
            self.text.focus_sync();
        });

        crossed_map.remove(&TagPart::Close).map(|map| {
            self.close.apply_slot_map.merge_slot(map.to_slot());
            self.close.focus_sync();
        });

        self.style.sync_slot(&self.apply_slot_map);
    }

    fn set_animation(&mut self, cx: &mut Cx) -> () {
        let init_global = cx.global::<ComponentAnInit>().tag;
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
            let basic_prop = self.style.get(TagState::Basic);
            let hover_prop = self.style.get(TagState::Hover);
            let pressed_prop = self.style.get(TagState::Pressed);
            let disabled_prop = self.style.get(TagState::Disabled);
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
                nodes: draw_tag = {
                    basic_index => {
                        background_color => basic_prop.container.background_color,
                        border_color => basic_prop.container.border_color,
                        border_radius => basic_prop.container.border_radius,
                        border_width => (basic_prop.container.border_width as f64),
                        shadow_color => basic_prop.container.shadow_color,
                        spread_radius => (basic_prop.container.spread_radius as f64),
                        blur_radius => (basic_prop.container.blur_radius as f64),
                        shadow_offset => basic_prop.container.shadow_offset,
                        background_visible => basic_prop.container.background_visible.to_f64()
                    },
                    hover_index => {
                        background_color => hover_prop.container.background_color,
                        border_color => hover_prop.container.border_color,
                        border_radius => hover_prop.container.border_radius,
                        border_width => (hover_prop.container.border_width as f64),
                        shadow_color => hover_prop.container.shadow_color,
                        spread_radius => (hover_prop.container.spread_radius as f64),
                        blur_radius => (hover_prop.container.blur_radius as f64),
                        shadow_offset => hover_prop.container.shadow_offset,
                        background_visible => hover_prop.container.background_visible.to_f64()
                    },
                    pressed_index => {
                        background_color => pressed_prop.container.background_color,
                        border_color => pressed_prop.container.border_color,
                        border_radius => pressed_prop.container.border_radius,
                        border_width => (pressed_prop.container.border_width as f64),
                        shadow_color => pressed_prop.container.shadow_color,
                        spread_radius => (pressed_prop.container.spread_radius as f64),
                        blur_radius => (pressed_prop.container.blur_radius as f64),
                        shadow_offset => pressed_prop.container.shadow_offset,
                        background_visible => pressed_prop.container.background_visible.to_f64()
                    },
                    disabled_index => {
                        background_color => disabled_prop.container.background_color,
                        border_color => disabled_prop.container.border_color,
                        border_radius => disabled_prop.container.border_radius,
                        border_width => (disabled_prop.container.border_width as f64),
                        shadow_color => disabled_prop.container.shadow_color,
                        spread_radius => (disabled_prop.container.spread_radius as f64),
                        blur_radius => (disabled_prop.container.blur_radius as f64),
                        shadow_offset => disabled_prop.container.shadow_offset,
                        background_visible => disabled_prop.container.background_visible.to_f64()
                    }
                }
            }
        } else {
            let state = self.state;
            let style = self.style.get(state);
            let index = match state {
                TagState::Basic => nodes.child_by_path(
                    self.index,
                    &[
                        live_id!(animator).as_field(),
                        live_id!(hover).as_instance(),
                        live_id!(off).as_instance(),
                    ],
                ),
                TagState::Hover => nodes.child_by_path(
                    self.index,
                    &[
                        live_id!(animator).as_field(),
                        live_id!(hover).as_instance(),
                        live_id!(on).as_instance(),
                    ],
                ),
                TagState::Pressed => nodes.child_by_path(
                    self.index,
                    &[
                        live_id!(animator).as_field(),
                        live_id!(hover).as_instance(),
                        live_id!(pressed).as_instance(),
                    ],
                ),
                TagState::Disabled => nodes.child_by_path(
                    self.index,
                    &[
                        live_id!(animator).as_field(),
                        live_id!(hover).as_instance(),
                        live_id!(disabled).as_instance(),
                    ],
                ),
            };
            set_animation! {
                nodes: draw_tag = {
                    index => {
                        background_color => style.container.background_color,
                        border_color => style.container.border_color,
                        border_radius => style.container.border_radius,
                        border_width => (style.container.border_width as f64),
                        shadow_color => style.container.shadow_color,
                        spread_radius => (style.container.spread_radius as f64),
                        blur_radius => (style.container.blur_radius as f64),
                        shadow_offset => style.container.shadow_offset,
                        background_visible => style.container.background_visible.to_f64()
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

impl Widget for GTag {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        if !self.visible() {
            return DrawStep::done();
        }
        let style = self.style.get(self.state);
        let _ = self.draw_tag.begin(cx, walk, style.container.layout());
        let _ = SlotDrawer::new(
            [
                (live_id!(icon), (&mut self.icon).into()),
                (live_id!(text), (&mut self.text).into()),
                (live_id!(close), (&mut self.close).into()),
            ],
            &mut self.defer_walks,
        )
        .draw_walk(cx, scope);

        let _ = self.draw_tag.end(cx);
        self.set_scope_path(&scope.path);
        DrawStep::done()
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, _scope: &mut Scope) {
        if !self.visible() {
            return;
        }

        self.set_animation(cx);
        cx.global::<ComponentAnInit>().tag = true;
        let area = self.area();
        let hit = event.hits(cx, area);
        if self.disabled {
            self.handle_when_disabled(cx, event, hit);
        } else {
            self.handle_widget_event(cx, event, hit, area);
        }
    }
}

impl GTag {
    pub fn area_close(&self) -> Area {
        self.close.area()
    }
    pub fn area_text(&self) -> Area {
        self.text.area()
    }
    pub fn area_icon(&self) -> Area {
        self.icon.area()
    }
    active_event! {
        active_hover_in: TagEvent::HoverIn |meta: FingerHoverEvent| => TagHoverIn { meta },
        active_hover_out: TagEvent::HoverOut |meta: FingerHoverEvent| => TagHoverOut { meta },
        active_finger_down: TagEvent::FingerDown |meta: FingerDownEvent| => TagFingerDown { meta },
        active_clicked: TagEvent::Clicked |meta: FingerUpEvent| => TagClicked { meta },
        active_close: TagEvent::Close |meta: FingerUpEvent| => TagClose { meta }
    }
    event_option! {
        hover_in: TagEvent::HoverIn => TagHoverIn,
        hover_out: TagEvent::HoverOut => TagHoverOut,
        finger_down: TagEvent::FingerDown => TagFingerDown,
        clicked: TagEvent::Clicked => TagClicked,
        close: TagEvent::Close => TagClose
    }
    pub fn slot_text(&self) -> &GLabel {
        &self.text
    }
    pub fn slot_text_mut(&mut self) -> &mut GLabel {
        &mut self.text
    }
}

impl GTagRef {
    event_option_ref! {
        hover_in => TagHoverIn,
        hover_out => TagHoverOut,
        finger_down => TagFingerDown,
        clicked => TagClicked,
        close => TagClose
    }
    pub fn slot_text_mut<F>(&mut self, cx: &mut Cx, f: F) -> ()
    where
        F: FnOnce(&mut Cx, &mut GLabel),
    {
        if let Some(mut c_ref) = self.borrow_mut() {
            f(cx, c_ref.slot_text_mut());
        }
    }
}
