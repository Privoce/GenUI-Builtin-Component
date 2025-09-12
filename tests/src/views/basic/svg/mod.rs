use gen_ui::{components::*, inherits_view_livehook};
use makepad_widgets::*;

use crate::widget_node;

live_design! {
    use link::widgets::*;
    use link::genui::*;
    use crate::views::cbox::*;

    pub SvgPage = {{SvgPage}} {
        <CBox> {
            show = {
                style: {
                    basic: {
                        height: Fit,
                        width: Fill,
                        flow: Down,
                    }
                }
                <GHLayout>{
                    style: {
                        basic: {
                            height: Fit,
                            width: Fill,
                            align: {y: 0.5}
                        }
                    },
                    <GSvg> {
                        style: {
                            basic: {
                                svg: {height: 36.0, width: 36.0},
                            }
                        }
                        src: dep("crate://self/resources/heavy.svg"),
                    }
                    <GLabel> {
                        text: "svg with out container"
                    }
                }
                <GHLayout>{
                    style: {
                        basic: {
                            height: Fit,
                            width: Fill,
                            align: {y: 0.5}
                        }
                    },
                    <GSvg> {
                        style: {
                            basic: {
                                container: {background_visible: true, theme: Primary, cursor: Hand}
                                svg: {height: 36.0, width: 36.0},
                            }
                        },
                        // disabled: true,
                        src: dep("crate://self/resources/rain.svg"),
                    }
                    <GLabel> {
                        text: "svg with container"
                    }
                }
                <GHLayout>{
                    style: {
                        basic: {
                            height: Fit,
                            width: Fill,
                            align: {y: 0.5}
                        }
                    },
                    <GSvg> {
                        style: {
                            basic: {
                                container: {
                                    background_visible: true, theme: Primary, cursor: Hand,
                                    blur_radius: 4.0,
                                    spread_radius: 4.0,
                                    clip_y: false, clip_x: false,
                                    border_radius: {left: 9.0, right: 9.0, top: 9.0, bottom: 9.0},
                                    align: {x: 0.5, y: 0.5},
                                }
                                svg: {height: 36.0, width: 36.0, theme: Success},
                            }
                        },
                        // disabled: true,
                        src: dep("crate://self/resources/heavy.svg"),
                    }
                    <GLabel> {
                        text: "more props"
                    }
                }
                <GHLayout>{
                    style: {
                        basic: {
                            height: Fit,
                            width: Fill,
                            align: {y: 0.5}
                        }
                    },
                    <IconClose> {}
                    <IconLeft> {}
                    <IconRight> {}
                    <IconMore> {}
                    <IconAll> {}
                    <IconMinus> {}
                    <IconPlus> {}
                    <IconChart> {}
                    <IconCheck> {}
                    <IconCloudDownload> {}
                    <IconCloudUpload> {}
                    <IconUp> {}
                    <IconDown> {}
                    <IconHome> {}
                    <IconLink> {}
                    <IconPowerOff> {}
                    <IconScatter> {}
                    <IconSearch> {}
                    <IconWaiting> {}
                    <IconGallery> {}
                    <IconGift> {}
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
                    }
                }
                esvg = <GSvg> {
                        style: {
                            basic: {
                                container: {background_visible: true, theme: Primary, cursor: Hand, padding: {
                                    left: 12.0, right: 12.0, top: 12.0, bottom: 12.0
                                }}
                                svg: {height: 36.0, width: 36.0},
                            }
                        },
                        // disabled: true,
                        src: dep("crate://self/resources/rain.svg"),
                    }
                elabel = <GLabel> {
                    text: "None"
                }
            }
            desc = {
                text: "Svg Events"
            }
        }
    }
}

#[derive(Live, WidgetRef, WidgetSet, LiveRegisterWidget)]
pub struct SvgPage {
    #[deref]
    pub deref_widget: GView,
}

impl LiveHook for SvgPage {
    inherits_view_livehook!();
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.deref_widget.after_apply(cx, apply, index, nodes);
    }
}

impl Widget for SvgPage {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let _ = self.deref_widget.draw_walk(cx, scope, walk);

        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.match_event(cx, event);
        self.deref_widget.handle_event(cx, event, scope)
    }
}

impl MatchEvent for SvgPage {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        let esvg = self.gsvg(id!(esvg));
        let elabel = self.glabel(id!(elabel));
        if let Some(_) = esvg.clicked(actions) {
            let _ = elabel.set_text(cx, "Clicked".to_string());
        }else if let Some(_) = esvg.hover_in(actions) {
            let _ = elabel.set_text(cx, "Hover In".to_string());
        }else if let Some(_) = esvg.hover_out(actions) {
            let _ = elabel.set_text(cx, "Hover Out".to_string());
        }else if let Some(_) = esvg.finger_down(actions) {
            let _ = elabel.set_text(cx, "Finger Down".to_string());
        }
    }
}

widget_node!(SvgPage);
