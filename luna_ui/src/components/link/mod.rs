mod event;
mod prop;

pub use event::*;
use makepad_widgets::*;
pub use prop::*;

use crate::{
    components::{
        lifecycle::LifeCycle,
        traits::{BasicProp, Component, Prop},
    },
    error::Error,
    lifecycle, play_animation,
    prop::{
        manuel::{BASIC, DISABLED, HOVER, PRESSED},
        traits::ToFloat,
        ApplyMapImpl, ApplyStateMap, ApplyStateMapImpl, DeferWalks,
        ToStateMap,
    },
    pure_after_apply, set_animation, set_index, set_scope_path,
    shader::{draw_link::DrawLink},
    sync,
    themes::Conf,
    visible, ComponentAnInit,
};

live_design! {
    link genui_basic;
    use link::genui_animation_prop::*;

    pub GLinkBase = {{GLink}}{
        animator: {
            hover = {
                default: off,
                off = {
                    from: {all: Forward {duration: 0.1}}
                    apply: {
                        draw_link: {underline_visible: 1.0}
                        draw_text: {color: #0080ff}
                    }
                }

                on = {
                    from: {all: Snap}
                    apply: {
                        draw_link: {underline_visible: 1.0}
                        draw_text: {color: #0066cc}
                    }
                }

                pressed = {
                    from: {all: Snap}
                    apply: {
                        draw_link: {underline_visible: 1.0}
                        draw_text: {color: #004499}
                    }
                }

                disabled = {
                    from: {all: Forward {duration: 0.1}}
                    apply: {
                        draw_link: {underline_visible: 0.0}
                        draw_text: {color: #888888}
                    }
                }
            }
        }
    }
}

#[derive(Live, LiveRegisterWidget, WidgetRef, WidgetSet)]
pub struct GLink {
    #[live]
    pub prop: LinkProp,
    #[live(true)]
    pub visible: bool,
    #[live]
    pub disabled: bool,
    // --- link specific properties ----------------
    #[live]
    pub href: ArcStringMut,
    // --- rendering components ----------------
    #[live]
    pub draw_link: DrawLink,
    #[live]
    pub draw_text: DrawText,
    // --- others ----------------
    #[rust]
    pub area: Area,
    #[live]
    pub text: ArcStringMut,
    #[rust]
    index: usize,
    #[rust]
    pub apply_state_map: ApplyStateMap<LinkState>,
    #[rust]
    pub animator: Animator,
    #[rust]
    pub scope_path: Option<HeapLiveIdPath>,
    #[rust]
    pub lifecycle: LifeCycle,
    #[rust]
    pub state: LinkState,
    #[rust]
    pub event: LinkEvent,
}

impl Widget for GLink {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.animator_handle_event(cx, event);
        self.event.handle_event(cx, event, scope.widget_actions());

        match self.event.get_event_type() {
            LinkEventType::HoverIn => {
                if !self.disabled {
                    self.state = LinkState::Hover;
                    self.set_animation(cx);
                }
                cx.widget_action(scope.widget_uid(), &scope.path, LinkAction::HoverIn);
            }
            LinkEventType::HoverOut => {
                if !self.disabled {
                    self.state = LinkState::Basic;
                    self.set_animation(cx);
                }
                cx.widget_action(scope.widget_uid(), &scope.path, LinkAction::HoverOut);
            }
            LinkEventType::Pressed => {
                if !self.disabled {
                    self.state = LinkState::Pressed;
                    self.set_animation(cx);
                }
                cx.widget_action(scope.widget_uid(), &scope.path, LinkAction::Pressed);
            }
            LinkEventType::Clicked => {
                if !self.disabled {
                    self.state = LinkState::Basic;
                    self.set_animation(cx);
                }
                cx.widget_action(scope.widget_uid(), &scope.path, LinkAction::Clicked {
                    href: self.href.as_ref().clone(),
                });
            }
            _ => {}
        }

        self.event.clear();
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        if !self.visible {
            return DrawStep::done();
        }

        let _ = self.draw_link.begin(cx, walk, self.layout);
        let _ = self.draw_text.draw_walk(cx, walk, self.align, &self.text);
        let _ = self.draw_link.end(cx);

        DrawStep::done()
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
            &LinkBasicProp::live_props(),
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
                HOVER => {
                    component.apply_state_map.insert(LinkState::Hover, applys);
                }
                PRESSED => {
                    component.apply_state_map.insert(LinkState::Pressed, applys);
                }
                DISABLED => {
                    component.apply_state_map.insert(LinkState::Disabled, applys);
                }
                _ => {}
            },
        );

        if self.disabled {
            self.state = LinkState::Disabled;
        }

        self.sync_state_to_prop();
        self.prop.sync(&self.apply_state_map);
    }
}

impl Component for GLink {
    type Error = Error;
    type State = LinkState;

    fn merge_conf_prop(&mut self, cx: &mut Cx) -> () {
        let prop = &cx.global::<Conf>().components.link;
        self.prop = prop.clone();
    }

    fn render(&mut self, _cx: &mut Cx) -> Result<(), Self::Error> {
        if self.disabled {
            self.switch_state(LinkState::Disabled);
        }
        let prop = self.prop.get(self.state);
        // Sync properties to draw components
        self.sync_state_to_prop();
        Ok(())
    }

    fn handle_widget_event(&mut self, cx: &mut Cx, event: &Event, hit: Hit, area: Area) {
        // Handle events as needed
    }

    fn switch_state(&mut self, state: Self::State) -> () {
        self.state = state;
        self.sync_state_to_prop();
    }

    fn switch_state_with_animation(&mut self, cx: &mut Cx, state: Self::State) -> () {
        self.state = state;
        self.set_animation(cx);
    }

    fn focus_sync(&mut self) -> () {
        // Focus handling if needed
    }
}

impl GLink {
    fn sync_state_to_prop(&mut self) {
        let prop = self.prop.get(self.state);
        
        // Sync container properties to draw_link
        self.draw_link.background_color = prop.background_color;
        self.draw_link.background_visible = prop.background_visible.to_f64() as f32;
        self.draw_link.border_color = prop.border_color;
        self.draw_link.border_width = prop.border_width;
        self.draw_link.border_radius = prop.border_radius;
        self.draw_link.shadow_color = prop.shadow_color;
        self.draw_link.shadow_offset = prop.shadow_offset;
        self.draw_link.spread_radius = prop.spread_radius;
        self.draw_link.blur_radius = prop.blur_radius;
        
        // Sync underline properties to draw_link
        self.draw_link.underline_visible = prop.underline_visible.to_f64() as f32;
        self.draw_link.underline_color = prop.underline_color;
        self.draw_link.underline_width = prop.underline_width;
        
        // Sync text properties to draw_text
        self.draw_text.color = prop.color;
        self.draw_text.text_style.font_size = prop.font_size;
        self.draw_text.text_style.line_spacing = prop.line_spacing;
    }

    fn set_animation(&mut self, cx: &mut Cx) -> () {
        // Simplified animation for now
        let state = self.state;
        match state {
            LinkState::Basic => {
                self.animator_play(cx, id!(hover.off));
            }
            LinkState::Hover => {
                self.animator_play(cx, id!(hover.on));
            }
            LinkState::Pressed => {
                self.animator_play(cx, id!(hover.pressed));
            }
            LinkState::Disabled => {
                self.animator_play(cx, id!(hover.disabled));
            }
        }
    }

    sync!();
    play_animation!();
    set_scope_path!();
    set_index!();
    lifecycle!();
}

#[derive(Clone, Debug, DefaultNone)]
pub enum LinkAction {
    None,
    HoverIn,
    HoverOut,
    Pressed,
    Clicked { href: String },
}
