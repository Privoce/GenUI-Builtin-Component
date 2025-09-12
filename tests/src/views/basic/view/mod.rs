use gen_ui::{
    components::*,
    inherits_view_livehook,
};
use makepad_widgets::*;

use crate::widget_node;

live_design! {
    use link::widgets::*;
    use link::genui::*;
    use crate::views::cbox::*;

    pub ViewPage = {{ViewPage}} {
        <CBox> {
            show = {
                style: {
                    basic: {
                        height: Fit,
                        width: Fill,
                    }
                }
                <GView> {
                    style: {
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
                    style: {
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
                    style: {
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
                    style: {
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
                    style: {
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
                <GView> {
                    style: {
                        basic: {
                            height: 100.0,
                            width: 100.0,
                            background_visible: true,
                            theme: Warning,
                            clip_y: true
                        }
                    },
                    <GLabel> {
                        text: "Scroll"
                    }
                    <GView> {
                        style: {
                            basic: {
                                height: 200.0,
                                width: 20.0,
                                background_visible: true,
                                theme: Primary,
                            }
                        },
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
                style: {
                    basic: {
                        height: Fit,
                        width: Fill,
                    }
                }
                <GView> {
                    style: {
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
                focus_v = <GView> {
                    style: {
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
                    draw_view: {
                        instance center: vec2(0.96, 0.96)
                        fn get_background_color(self) -> vec4 {
                            let center = self.center;
                            let distance = distance (self.pos , center) ;
                            let factor = clamp (distance , 0.0 , 1.0) ;
                            let color0 = #82440F;
                            let stop0 = 0.0 ;
                            let color1 = #52241C;
                            let stop1 = 0.3;
                            let color2 = #1F1616;
                            let stop2 = 1.0;
                            return mix (color0 , mix (color1 , color2 , smoothstep (stop1 , stop2 , factor)) , smoothstep (stop0 , stop1 , factor));
                        }
                    }
                    <GLabel> {
                        text: "Focus Mouse"
                    }
                    animation_open: true,
                    event_open: true,
                }
            }
            desc = {
                text: "Animation"
            }
        }
        // --------------------- event handling ---------------------------------------------------------
        <CBox> {
            show = {
                style: {
                    basic: {
                        height: Fit,
                        width: Fill,
                    }
                }
                eview = <GView> {
                    style: {
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
        self.deref_widget.handle_event(cx, event, scope);
    }
}

impl MatchEvent for ViewPage {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        let eview = self.gview(id!(eview));
        let etext = self.glabel(id!(etext));
        let ebtn = self.gbutton(id!(ebtn));
        let focus_v = self.gview(id!(focus_v));
        if let Some(_) = eview.clicked(actions) {
            let _ = etext.set_text(cx, "Clicked".to_string());
        }
        if let Some(_) = ebtn.clicked(actions) {
            let _ = etext.set_text(cx, "Button Clicked".to_string());
        }
        if let Some(e) = focus_v.hover_over(&actions) {
            let rect = focus_v.area().rect(cx);
            let pos = rect.pos;
            let size = rect.size;
            let center = e.meta.abs;
            let x = (center.x - pos.x) / size.x;
            let y = (center.y - pos.y) / size.y;
            let center = vec2(x as f32, y as f32);
            focus_v.borrow_mut().unwrap().draw_view.apply_over(
                cx,
                live! {
                    center: (center)
                },
            );
        }
    }
}

widget_node!(ViewPage);
