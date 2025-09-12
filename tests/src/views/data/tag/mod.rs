use gen_ui::{components::*, inherits_view_livehook};
use makepad_widgets::*;

use crate::widget_node;

live_design! {
    use link::widgets::*;
    use link::genui::*;
    use crate::views::cbox::*;

    pub TagPage = {{TagPage}} {
        <CBox> {
            show = {
                style: {
                    basic: {
                        height: Fit,
                        width: Fill,
                        flow: Down,
                        spacing: 20.0,
                    }
                }
                <GTag>{
                    text: <GLabel> {
                        text: "Basic Tag"
                    }
                }
                <GTag>{
                    style: {basic: {
                        container: {theme: Primary},
                        icon: { svg: {width: 12.0}},
                        close: { svg: {width: 12.0}},
                    }},
                    icon: <IconCheck> {}
                    text: <GLabel> {
                        text: "with icon: check"
                    }
                }
                <GTag>{
                    style: {basic: {
                        container: {theme: Error},
                        icon: { svg: {width: 12.0}},
                        close: { svg: {width: 12.0}},
                    }}
                    text: <GLabel> {
                        text: "with close"
                    }
                    close: <IconClose> {}
                }
                <GTag>{
                    style: {basic: {
                        container: {theme: Success},
                        icon: { svg: {width: 12.0}},
                        close: { svg: {width: 12.0}},
                    }}
                    icon: <IconPlus> {}
                    text: <GLabel> {
                        text: "icon + close"
                    }
                    close: <IconClose> {}
                }
                <GTag>{
                    style: {basic: {
                        container: {theme: Info, border_radius: {left: 6.0, right: 6.0, top: 6.0, bottom: 6.0}},
                        icon: { svg: {width: 12.0}},
                        close: { svg: {width: 12.0}},
                    }}
                    text: <GLabel> {
                        text: "rounded tag"
                    }
                }
            }
            desc = {
                text: ""
            }
        }
        <CBox> {
            show = {
                style: {
                    basic: {
                        height: Fit,
                        width: Fill,
                        flow: Down,
                        spacing: 20.0,
                    }
                }
                etag = <GTag>{
                    style: {basic: {
                        container: {theme: Success},
                        icon: { svg: {width: 12.0}},
                        close: { svg: {width: 12.0}},
                    }}
                    icon: <IconPlus> {}
                    text: <GLabel> {
                        text: "icon + close"
                    }
                    close: <IconClose> {}
                }
            }
            desc = {
                text: ""
            }
        }

    }
}

#[derive(Live, WidgetRef, WidgetSet, LiveRegisterWidget)]
pub struct TagPage {
    #[deref]
    pub deref_widget: GView,
}

impl LiveHook for TagPage {
    inherits_view_livehook!();
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.deref_widget.after_apply(cx, apply, index, nodes);
    }
}

impl Widget for TagPage {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let _ = self.deref_widget.draw_walk(cx, scope, walk);

        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.match_event(cx, event);
        self.deref_widget.handle_event(cx, event, scope)
    }
}

impl MatchEvent for TagPage {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        let mut etag = self.gtag(id!(etag));
        if let Some(_) = etag.clicked(actions) {
            etag.slot_text_mut(cx, |cx, lb| {
                let _ = lb.set_text(cx, "Clicked".to_string());
            });
        } else if let Some(_) = etag.hover_in(actions) {
            etag.slot_text_mut(cx, |cx, lb| {
                let _ = lb.set_text(cx, "Hover In".to_string());
            });
        } else if let Some(_) = etag.hover_out(actions) {
            etag.slot_text_mut(cx, |cx, lb| {
                let _ = lb.set_text(cx, "Hover Out".to_string());
            });
        } else if let Some(_) = etag.finger_down(actions) {
            etag.slot_text_mut(cx, |cx, lb| {
                let _ = lb.set_text(cx, "Finger Down".to_string());
            });
        } else if let Some(_) = etag.close(actions) {
            etag.slot_text_mut(cx, |cx, lb| {
                let _ = lb.set_text(cx, "Close".to_string());
            });
        }
    }
}

widget_node!(TagPage);
