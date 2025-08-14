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

    pub ButtonPage = {{ButtonPage}} {
        <CBox> {
            show = {
                prop: {
                    basic: {
                        height: Fit,
                        width: Fill,
                    }
                }
                <GButton> {}
                <GButton> {
                    prop: {
                        basic: {
                            theme: Primary,
                        }
                    }
                }
            }
            desc = {
                text: "Basic Button Component"
            }
        }
        // --------------------- animation ---------------------------------------------------------
        
        // --------------------- event handling ---------------------------------------------------------
       
    }
}

#[derive(Live, WidgetRef, WidgetSet, LiveRegisterWidget)]
pub struct ButtonPage {
    #[deref]
    pub deref_widget: GView,
}

impl LiveHook for ButtonPage {
    inherits_view_livehook!();
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.deref_widget.after_apply(cx, apply, index, nodes);
    }
}

impl Widget for ButtonPage {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let _ = self.deref_widget.draw_walk(cx, scope, walk);

        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.match_event(cx, event);
        self.deref_widget.handle_event(cx, event, scope)
    }
}

impl MatchEvent for ButtonPage {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        // let eview = self.gview(id!(eview));
        // let etext = self.glabel(id!(etext));
        // let ebtn = self.gbutton(id!(ebtn));
        // if let Some(_) = eview.clicked(actions) {
        //     dbg!("Clicked");
        //     let _ = etext.set_text(cx, "Clicked".to_string());
        // }
        // if let Some(_) = ebtn.clicked(actions) {
        //     dbg!("Button Clicked");
        //     let _ = etext.set_text(cx, "Button Clicked".to_string());
        // }
    }
}

widget_node!(ButtonPage);
