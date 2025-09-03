use luna_ui::{components::*, prop::traits::ToColor, themes::Theme};
use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::genui::*;
    use link::shaders::*;
    use link::widgets::*;
    use crate::an::*;
    use crate::views::home::*;
    use crate::views::test_home::*;
    use crate::views::basic::view::*;

    Post = <View> {
        width: Fill, height: Fit,
        padding: { top: 10., bottom: 10.}

        body = <RoundedView> {
            width: Fill, height: Fit
            content = <View> {
                width: Fill, height: Fit
                text = <P> { text: "" }
            }
        }
    }

    App = {{App}} {
        ui: <Root>{
            main_window = <Window>{
                body = <View>{
                    flow: Right,
                    spacing:30,
                    align: {
                        x: 0.5,
                        y: 0.5
                    },
                    show_bg: true,
                    draw_bg: {
                        color: #140D2A,
                    }



                    // <GVTabbar>{
                    //     items: [
                    //         {
                    //             value: "wind",
                    //             text: "Wind",
                    //             icon: {
                    //                 src: dep("crate://self/resources/wind.svg"),
                    //             },
                    //             prop: {
                    //                 basic: {
                    //                     icon: {
                    //                         svg: {
                    //                             theme: Primary
                    //                         }
                    //                     }
                    //                 }
                    //             }
                    //         }
                    //     ]
                    // }
                    // <GNavPage> {
                    //     prop: {
                    //         basic: {
                    //             height: 300.0,
                    //             width: 300.0,
                    //             background_visible: true,
                    //             theme: Info,
                    //         }
                    //     }
                    //     <GButton>{}
                    // }
                    //

                   

                    // <GHLayout> {
                        // <GView> {
                        //     prop: {
                        //         basic: {
                        //             background_visible: true,
                        //             height: 200.0,
                        //             clip_y: true
                        //         }
                        //     }
                        //     scroll_bars: <GScrollBars> {
                        //         show_scroll_y: true
                        //     }
                        //     <GView> {
                        //         prop: {
                        //             basic: {
                        //                 background_visible: true,
                        //                 height: 300.0,
                        //                 width: 60.0,
                        //                 theme: Primary,
                        //             }
                        //         }
                        //     }
                        // }
                       
                    // }
                    body = <HomePage> {}
                    // body = <THomePage> {}
                    // body = <ViewPage>{}
                }
            }
        }
    }
}

app_main!(App);

#[derive(Live, LiveHook)]
pub struct App {
    #[live]
    ui: WidgetRef,
    #[rust]
    counter: usize,
}

impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        crate::makepad_widgets::live_design(cx);
        crate::an::live_design(cx);
        crate::luna_ui::live_design(cx);
        crate::views::register(cx);
    }
}

impl MatchEvent for App {
    fn handle_startup(&mut self, _cx: &mut Cx) {}

    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        // let v1 = self.ui.GView(id!(v1));
        // if let Some(_) = v1.lbutton(id!(btn1)).clicked(actions) {
        //     if let Some(v1) = v1.borrow() {
        //         dbg!(&v1.prop.hover.theme);
        //         dbg!(&v1.apply_state_map);
        //     }
        // }
        let rsvg = self.ui.gsvg(id!(rsvg));
        let rbtn = self.ui.gbutton(id!(rbtn));
        if let Some(_) = rbtn.clicked(actions) {
            rsvg.redraw(cx);
        }
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        if let Event::XrUpdate(_e) = event {
            //log!("{:?}", e.now.left.trigger.analog);
        }
        self.match_event(cx, event);
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}
