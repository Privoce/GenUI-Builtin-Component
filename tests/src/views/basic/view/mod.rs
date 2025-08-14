use luna_ui::{
    components::{
        button::GButtonWidgetExt, label::GLabelWidgetExt, view::{GView, GViewWidgetExt}
    },
    inherits_view_livehook,
};
use makepad_widgets::*;

use crate::widget_node;

live_design! {
    use link::widgets::*;
    use link::gen_ui::*;
    use crate::views::cbox::*;

    pub ViewPage = {{ViewPage}} {
        <CBox> {
            show = {
                prop: {
                    basic: {
                        height: Fit,
                        width: Fill,
                    }
                }
                <GView> {
                    prop: {
                        basic: {
                            height: 100.0,
                            width: 100.0,
                            background_visible: true,
                        }
                    }
                    <GLabel> {
                        text: "Basic"
                    }
                }
                <GView> {
                    prop: {
                        basic: {
                            height: 100.0,
                            width: 100.0,
                            background_visible: true,
                            theme: Primary,
                        }
                    }
                    <GLabel> {
                        text: "Primary"
                    }
                }
                <GView> {
                    prop: {
                        basic: {
                            height: 100.0,
                            width: 100.0,
                            background_visible: true,
                            theme: Info,
                            border_width: 2.0,
                            border_color: #ff0,
                        }
                    }
                    <GLabel> {
                        text: "Border"
                    }
                }
                <GView> {
                    prop: {
                        basic: {
                            height: 100.0,
                            width: 100.0,
                            background_visible: true,
                            theme: Error,
                            spread_radius: 4.0,
                            blur_radius: 4.0,
                        }
                    }
                    <GLabel> {
                        text: "Shadow"
                    }
                }
                <GView> {
                    prop: {
                        basic: {
                            height: 100.0,
                            width: 100.0,
                            background_visible: true,
                            theme: Success,
                        }
                    },
                    disabled: true,
                    <GLabel> {
                        text: "Disabled"
                    }
                }
            }
            desc = {
                text: "Basic View Component"
            }
        }
        // --------------------- animation ---------------------------------------------------------
        <CBox> {
            show = {
                prop: {
                    basic: {
                        height: Fit,
                        width: Fill,
                    }
                }
                <GView> {
                    prop: {
                        basic: {
                            height: 100.0,
                            width: 100.0,
                            background_visible: true,
                            theme: Warning,
                            spread_radius: 4.0,
                            blur_radius: 4.0,
                            cursor: Hand
                        }
                    }
                    <GLabel> {
                        text: "Animation"
                    }
                    animation_open: true,
                }
            }
            desc = {
                text: "Animation"
            }
        }
        // --------------------- event handling ---------------------------------------------------------
        <CBox> {
            show = {
                prop: {
                    basic: {
                        height: Fit,
                        width: Fill,
                    }
                }
                eview = <GView> {
                    prop: {
                        basic: {
                            height: 100.0,
                            width: 100.0,
                            background_visible: true,
                            theme: Warning,
                            spread_radius: 4.0,
                            blur_radius: 4.0,
                            cursor: Hand
                        }
                    }
                    etext = <GLabel> {
                        text: "None"
                    }
                    animation_open: true,
                    event_open: true,
                }
                ebtn = <GButton> {}
            }
            desc = {
                text: "Event Handling"
            }
        }
    }
}

#[derive(Live, WidgetRef, WidgetSet, LiveRegisterWidget)]
pub struct ViewPage {
    #[deref]
    pub deref_widget: GView,
}

impl LiveHook for ViewPage {
    inherits_view_livehook!();
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.deref_widget.after_apply(cx, apply, index, nodes);
    }
}

impl Widget for ViewPage {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let _ = self.deref_widget.draw_walk(cx, scope, walk);

        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.match_event(cx, event);
        self.deref_widget.handle_event(cx, event, scope)
    }
}

impl MatchEvent for ViewPage {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        let eview = self.gview(id!(eview));
        let etext = self.glabel(id!(etext));
        let ebtn = self.gbutton(id!(ebtn));
        if let Some(_) = eview.clicked(actions) {
            dbg!("Clicked");
            let _ = etext.set_text(cx, "Clicked".to_string());
        }
        if let Some(_) = ebtn.clicked(actions) {
            dbg!("Button Clicked");
            let _ = etext.set_text(cx, "Button Clicked".to_string());
        }
    }
}

widget_node!(ViewPage);
