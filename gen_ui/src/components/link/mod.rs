mod event;
mod prop;

pub use event::*;
use makepad_widgets::{shader::draw_text::TextStyle, *};
pub use prop::*;

use crate::{
    active_event, animation_open_then_redraw,
    components::{
        label::FontMode,
        lifecycle::LifeCycle,
        traits::{BasicStyle, Component, Style},
    },
    error::Error,
    event_option, hit_finger_down, hit_finger_up, hit_hover_in, hit_hover_out, lifecycle,
    play_animation,
    prop::{
        manuel::{BASIC, DISABLED, HOVER, PRESSED},
        traits::ToFloat,
        ApplyStateMap,
    },
    pure_after_apply, set_animation, set_index, set_scope_path,
    shader::draw_link::DrawLink,
    sync,
    themes::conf::Conf,
    visible, ComponentAnInit,
};

live_design! {
    link genui_basic;
    use link::theme::*;
    use link::genui_animation_prop::*;

    pub GLinkBase = {{GLink}}{
        animator: {
            hover = {
                default: off,

                off = {
                    from: {all: Forward {duration: (AN_DURATION)}},
                    ease: InOutQuad,
                    apply: {
                        draw_text: <AN_DRAW_LINK_TEXT> {}
                        draw_link: <AN_DRAW_LINK> {}
                    }
                }

                on = {
                    from: {
                        all: Forward {duration: (AN_DURATION),},
                        pressed: Forward {duration: (AN_DURATION)},
                    },
                    ease: InOutQuad,
                    apply: {
                       draw_text: <AN_DRAW_LINK_TEXT> {}
                       draw_link: <AN_DRAW_LINK> {}
                    }
                }

                pressed = {
                    from: {all: Forward {duration: (AN_DURATION)}},
                    ease: InOutQuad,
                    apply: {
                        draw_text: <AN_DRAW_LINK_TEXT> {}
                        draw_link: <AN_DRAW_LINK> {}
                    }
                }

                disabled = {
                    from: {all: Forward {duration: (AN_DURATION)}},
                    ease: InOutQuad,
                    apply: {
                        draw_text: <AN_DRAW_LINK_TEXT> {}
                        draw_link: <AN_DRAW_LINK> {}
                    }
                }
            }
        }
        font_regular: <THEME_FONT_REGULAR>{}
        font_bold: <THEME_FONT_BOLD>{}
        font_italic: <THEME_FONT_ITALIC>{}
        font_bold_italic: <THEME_FONT_BOLD_ITALIC>{}
    }
}

#[derive(Live, LiveRegisterWidget, WidgetRef, WidgetSet)]
pub struct GLink {
    #[live]
    pub style: LinkProp,
    #[live(true)]
    pub visible: bool,
    #[live(false)]
    pub disabled: bool,
    #[live(FontMode::Regular)]
    pub mode: FontMode,
    // --- others ----------------
    #[live]
    pub text: ArcStringMut,
    #[live]
    pub href: Option<ArcStringMut>,
    #[rust]
    index: usize,
    #[rust]
    pub apply_state_map: ApplyStateMap<LinkState>,
    // --- fonts ----------------
    #[live]
    font_regular: TextStyle,
    #[live]
    font_bold: TextStyle,
    #[live]
    font_italic: TextStyle,
    #[live]
    font_bold_italic: TextStyle,
    // --- draw ------------------
    #[live]
    pub draw_text: DrawText,
    #[live]
    pub draw_link: DrawLink,
    #[rust]
    pub scope_path: Option<HeapLiveIdPath>,
    // --- init -----------------
    #[live(true)]
    pub sync: bool,
    #[rust]
    pub lifecycle: LifeCycle,
    #[rust]
    pub state: LinkState,
    // --- animator ----------------
    #[live(true)]
    pub animation_open: bool,
    #[animator]
    pub animator: Animator,
    #[live(true)]
    pub animation_spread: bool,
    #[live(true)]
    pub event_open: bool,
    #[live]
    pub grab_key_focus: bool,
}

impl WidgetNode for GLink {
    fn uid_to_widget(&self, _uid: WidgetUid) -> WidgetRef {
        WidgetRef::empty()
    }

    fn find_widgets(&self, _path: &[LiveId], _cached: WidgetCache, _results: &mut WidgetSet) {
        ()
    }

    fn walk(&mut self, _cx: &mut Cx) -> Walk {
        let style = self.style.get(self.state);
        style.walk()
    }

    fn area(&self) -> Area {
        self.draw_link.area
    }

    fn redraw(&mut self, cx: &mut Cx) {
        let _ = self.render(cx);
        self.draw_link.redraw(cx);
        self.draw_text.redraw(cx);
    }

    fn state(&self) -> String {
        self.state.to_string()
    }
    fn animation_spread(&self) -> bool {
        self.animation_spread
    }

    visible!();
}

impl Widget for GLink {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        if !self.visible {
            return DrawStep::done();
        }

        let style = self.style.get(self.state);
        self.draw_link.begin(cx, walk, style.layout());
        // // here we need to check if the text is empty, if so we need to set it to a space
        // // or the text draw will not work(seems like lazy drawtext bug)
        // let _ = self.text.as_ref().is_empty().then(|| {
        //     let _ = self.set_text(cx, " ");
        // });
        self.draw_text
            .draw_walk(cx, style.walk(), Align::default(), self.text.as_ref());
        // cx.end_turtle_with_area(&mut self.area);
        self.draw_link.end(cx);
        self.set_scope_path(&scope.path);
        DrawStep::done()
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, _scope: &mut Scope) {
        if !self.visible {
            return;
        }

        self.set_animation(cx);
        cx.global::<ComponentAnInit>().link = true;
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
        cx.global::<ComponentAnInit>().link = true;
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

impl LiveHook for GLink {
    pure_after_apply!();

    fn after_new_before_apply(&mut self, cx: &mut Cx) {
        self.merge_conf_prop(cx);
    }

    fn after_apply(&mut self, _cx: &mut Cx, _apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.set_apply_state_map(
            nodes,
            index,
            &LinkBasicStyle::live_props(),
            [
                live_id!(basic),
                live_id!(hover),
                live_id!(pressed),
                live_id!(disabled),
            ],
            |_| {},
            |prefix, component, applys| match prefix.to_string().as_str() {
                BASIC => {
                    component.apply_state_map.insert(LinkState::Basic, applys);
                }
                DISABLED => {
                    component
                        .apply_state_map
                        .insert(LinkState::Disabled, applys);
                }
                HOVER => {
                    component.apply_state_map.insert(LinkState::Hover, applys);
                }
                PRESSED => {
                    component.apply_state_map.insert(LinkState::Pressed, applys);
                }
                _ => {}
            },
        );
    }
}

impl Component for GLink {
    type Error = Error;
    type State = LinkState;

    fn merge_conf_prop(&mut self, cx: &mut Cx) -> () {
        let link_prop = &cx.global::<Conf>().components.link;
        // [sync from conf prop] -----------------------------------------------------
        self.style = link_prop.clone();
    }

    fn render(&mut self, _cx: &mut Cx) -> Result<(), Self::Error> {
        if self.disabled {
            self.switch_state(LinkState::Disabled);
        }
        let style = self.style.get(self.state);
        // [sync to draw_link] -------------------------------------------------------
        self.draw_text.color = style.color;
        self.draw_text.text_style.font_size = style.font_size;
        self.draw_text.text_style.line_spacing = style.line_spacing;
        self.draw_text.text_style.font_family = match self.mode {
            FontMode::Regular => self.font_regular.font_family.clone(),
            FontMode::Bold => self.font_bold.font_family.clone(),
            FontMode::Italic => self.font_italic.font_family.clone(),
            FontMode::BoldItalic => self.font_bold_italic.font_family.clone(),
        };
        self.draw_link.merge(style);
        Ok(())
    }

    fn handle_when_disabled(&mut self, cx: &mut Cx, _event: &Event, hit: Hit) -> () {
        match hit {
            Hit::FingerHoverIn(_) => {
                self.switch_state_and_redraw(cx, LinkState::Disabled);
                cx.set_cursor(self.style.get(self.state).cursor);
            }
            _ => {}
        }
    }

    fn handle_widget_event(&mut self, cx: &mut Cx, event: &Event, hit: Hit, area: Area) {
        animation_open_then_redraw!(self, cx, event);
        match hit {
            Hit::FingerDown(e) => {
                self.switch_state_with_animation(cx, LinkState::Pressed);
                hit_finger_down!(self, cx, area, e);
            }
            Hit::FingerHoverIn(e) => {
                cx.set_cursor(self.style.get(self.state).cursor);
                self.switch_state_with_animation(cx, LinkState::Hover);
                hit_hover_in!(self, cx, e);
            }
            Hit::FingerHoverOut(e) => {
                self.switch_state_with_animation(cx, LinkState::Basic);
                hit_hover_out!(self, cx, e);
            }
            Hit::FingerUp(e) => {
                if e.is_over {
                    if e.has_hovers() {
                        self.switch_state_with_animation(cx, LinkState::Hover);
                        self.play_animation(cx, id!(hover.on));
                    } else {
                        self.switch_state_with_animation(cx, LinkState::Basic);
                        self.play_animation(cx, id!(hover.off));
                    }
                    self.active_clicked(cx, e);
                } else {
                    self.switch_state_with_animation(cx, LinkState::Basic);
                    hit_finger_up!(self, cx, e);
                }
            }
            _ => {}
        };
    }

    fn switch_state(&mut self, state: Self::State) -> () {
        self.state = state
    }
    fn switch_state_with_animation(&mut self, cx: &mut Cx, state: Self::State) -> () {
        if !self.animation_open {
            return;
        }
        self.switch_state(state);
        self.set_animation(cx);
        self.redraw(cx);
    }
    fn set_animation(&mut self, cx: &mut Cx) -> () {
        let init_global = cx.global::<ComponentAnInit>().link;

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
            let basic_prop = self.style.get(LinkState::Basic);
            let hover_prop = self.style.get(LinkState::Hover);
            let pressed_prop = self.style.get(LinkState::Pressed);
            let disabled_prop = self.style.get(LinkState::Disabled);
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
                nodes: draw_link = {
                    basic_index => {
                        background_color => basic_prop.background_color,
                        border_color => basic_prop.border_color,
                        border_radius => basic_prop.border_radius,
                        border_width => (basic_prop.border_width as f64),
                        shadow_color => basic_prop.shadow_color,
                        spread_radius => (basic_prop.spread_radius as f64),
                        blur_radius => (basic_prop.blur_radius as f64),
                        shadow_offset => basic_prop.shadow_offset,
                        background_visible => basic_prop.background_visible.to_f64(),
                        underline_color => basic_prop.underline_color,
                        underline_visible => basic_prop.underline_visible.to_f64(),
                        underline_width => (basic_prop.underline_width as f64)
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
                        background_visible => hover_prop.background_visible.to_f64(),
                        underline_color => hover_prop.underline_color,
                        underline_visible => hover_prop.underline_visible.to_f64(),
                        underline_width => (hover_prop.underline_width as f64)
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
                        background_visible => pressed_prop.background_visible.to_f64(),
                        underline_color => pressed_prop.underline_color,
                        underline_visible => pressed_prop.underline_visible.to_f64(),
                        underline_width => (pressed_prop.underline_width as f64)
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
                        background_visible => disabled_prop.background_visible.to_f64(),
                        underline_color => disabled_prop.underline_color,
                        underline_visible => disabled_prop.underline_visible.to_f64(),
                        underline_width => (disabled_prop.underline_width as f64)
                    }
                }
            }

            set_animation! {
                nodes: draw_text = {
                    basic_index => {
                        color => basic_prop.color
                    },
                    hover_index => {
                        color => hover_prop.color
                    },
                    pressed_index => {
                        color => pressed_prop.color
                    },
                    disabled_index => {
                        color => disabled_prop.color
                    }
                }
            }

            for index in [basic_index, hover_index, pressed_index, disabled_index] {
                for (field, target_prop) in [
                    (
                        live_id!(font_size),
                        (basic_prop.font_size as f64).to_live_value(),
                    ),
                    (
                        live_id!(line_spacing),
                        (basic_prop.line_spacing as f64).to_live_value(),
                    ),
                ] {
                    if let Some(index) = index {
                        if let Some(v_index) = nodes.child_by_path(
                            index,
                            &[
                                live_id!(apply).as_field(),
                                live_id!(draw_text).as_field(),
                                live_id!(text_style).as_field(),
                                field.as_field(),
                            ],
                        ) {
                            nodes[v_index].value = target_prop;
                        }
                    }
                }
            }
        } else {
            let state = self.state;
            let style = self.style.get(state);
            let index = match state {
                LinkState::Basic => nodes.child_by_path(
                    self.index,
                    &[
                        live_id!(animator).as_field(),
                        live_id!(hover).as_instance(),
                        live_id!(off).as_instance(),
                    ],
                ),
                LinkState::Hover => nodes.child_by_path(
                    self.index,
                    &[
                        live_id!(animator).as_field(),
                        live_id!(hover).as_instance(),
                        live_id!(on).as_instance(),
                    ],
                ),
                LinkState::Pressed => nodes.child_by_path(
                    self.index,
                    &[
                        live_id!(animator).as_field(),
                        live_id!(hover).as_instance(),
                        live_id!(pressed).as_instance(),
                    ],
                ),
                LinkState::Disabled => nodes.child_by_path(
                    self.index,
                    &[
                        live_id!(animator).as_field(),
                        live_id!(hover).as_instance(),
                        live_id!(disabled).as_instance(),
                    ],
                ),
            };

            set_animation! {
                nodes: draw_link = {
                    index => {
                        background_color => style.background_color,
                        border_color => style.border_color,
                        border_radius => style.border_radius,
                        border_width => (style.border_width as f64),
                        shadow_color => style.shadow_color,
                        spread_radius => (style.spread_radius as f64),
                        blur_radius => (style.blur_radius as f64),
                        shadow_offset => style.shadow_offset,
                        background_visible => style.background_visible.to_f64(),
                        underline_color => style.underline_color,
                        underline_visible => style.underline_visible.to_f64(),
                        underline_width => (style.underline_width as f64)
                    }
                }
            }

            set_animation! {
                nodes: draw_text = {
                    index => {
                        color => style.color
                    }
                }
            }

            for (field, target_prop) in [
                (live_id!(font_size), (style.font_size as f64).to_live_value()),
                (
                    live_id!(line_spacing),
                    (style.line_spacing as f64).to_live_value(),
                ),
            ] {
                if let Some(index) = index {
                    if let Some(v_index) = nodes.child_by_path(
                        index,
                        &[
                            live_id!(apply).as_field(),
                            live_id!(draw_text).as_field(),
                            live_id!(text_style).as_field(),
                            field.as_field(),
                        ],
                    ) {
                        nodes[v_index].value = target_prop;
                    }
                }
            }
        }
    }
    fn focus_sync(&mut self) -> () {
        self.style.sync(&self.apply_state_map);
    }

    sync!();
    set_index!();
    lifecycle!();
    set_scope_path!();
    play_animation!();
}

impl GLink {
    active_event! {
        active_hover_in: LinkEvent::HoverIn |meta: FingerHoverEvent| => LinkHoverIn { meta },
        active_hover_out: LinkEvent::HoverOut |meta: FingerHoverEvent| => LinkHoverOut { meta },
        active_finger_up: LinkEvent::FingerUp |meta: FingerUpEvent| => LinkFingerUp { meta },
        active_finger_down: LinkEvent::FingerDown |meta: FingerDownEvent| => LinkFingerDown { meta },
        active_clicked: LinkEvent::Clicked |meta: FingerUpEvent| => LinkClicked { meta }
    }
    event_option! {
        hover_in: LinkEvent::HoverIn => LinkHoverIn,
        hover_out: LinkEvent::HoverOut => LinkHoverOut,
        finger_up: LinkEvent::FingerUp => LinkFingerUp,
        finger_down: LinkEvent::FingerDown => LinkFingerDown,
        clicked: LinkEvent::Clicked => LinkClicked
    }
}
