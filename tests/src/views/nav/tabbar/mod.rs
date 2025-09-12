use gen_ui::{components::*, inherits_view_livehook};
use makepad_widgets::*;

use crate::widget_node;

live_design! {
    use link::widgets::*;
    use link::genui::*;
    use crate::views::cbox::*;

    pub TabbarPage = {{TabbarPage}} {
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
                    <GTabbarItem> {
                        style: {basic: {container: {height: Fit, width: Fit}}}
                        icon: <GSvg> {
                            src: dep("crate://self/resources/wind.svg"),
                        }
                        text: <GLabel> {
                            text: "Wind"
                        }
                    }
                    <GTabbar> {
                        <GTabbarItem> {
                            value: "news",
                            icon: <GSvg> {
                                src: dep("crate://self/resources/news.svg"),
                            }
                            text: <GLabel> {
                                text: "News"
                            }
                        }
                        <GTabbarItem> {
                            value: "global",
                            icon: <GSvg> {
                                src: dep("crate://self/resources/global.svg"),
                            }
                            text: <GLabel> {
                                text: "Global"
                            }
                        }
                        <GTabbarItem> {
                            value: "for_you",
                            icon: <GSvg> {
                                src: dep("crate://self/resources/star.svg"),
                            }
                            text: <GLabel> {
                                text: "For You"
                            }
                        }
                        <GTabbarItem> {
                            value: "trending",
                            icon: <GSvg> {
                                src: dep("crate://self/resources/trending.svg"),
                            }
                            text: <GLabel> {
                                text: "Trending"
                            }
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

            }
            desc = {
                text: ""
            }
        }

    }
}

#[derive(Live, WidgetRef, WidgetSet, LiveRegisterWidget)]
pub struct TabbarPage {
    #[deref]
    pub deref_widget: GView,
}

impl LiveHook for TabbarPage {
    inherits_view_livehook!();
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.deref_widget.after_apply(cx, apply, index, nodes);
    }
}

impl Widget for TabbarPage {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let _ = self.deref_widget.draw_walk(cx, scope, walk);

        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.match_event(cx, event);
        self.deref_widget.handle_event(cx, event, scope)
    }
}

impl MatchEvent for TabbarPage {
    
}

widget_node!(TabbarPage);
