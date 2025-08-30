use luna_ui::{
    components::*,
    prop::traits::ToColor,
    themes::Theme,
};
use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::gen_ui::*;
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
                    flow: Down,
                    spacing:30,
                    align: {
                        x: 0.5,
                        y: 0.5
                    },
                    show_bg: true,
                    draw_bg: {
                        color: #140D2A,
                    }
                    
                    
                    // <GButton>{
                    //     prop: {
                    //         basic: {
                    //             theme: Success,
                    //         }
                    //     }
                    // }
                    // <GDivider>{
                    //     prop: {
                    //         basic: {
                    //             theme: Primary,
                    //         }
                    //     }
                    // }
                    // <GView>{
                    //     prop: {
                    //         basic: {
                    //             height: 100.0,
                    //             width: 100.0,
                    //             background_visible: true,
                    //             align: {x: 0.5, y: 0.5},
                    //             theme: Error,
                    //         }
                    //     }
                  
                    
                    // <GDialog> {
                    //     popup: {
                    //         popup: {
                    //             <GButton> {
                    //                 prop: {
                    //                     basic: {theme: Success}
                    //                 }
                    //             }
                    //         }
                    //     }
                    //     <GButton> {}
                    // }

                    // <GDrawer> {
                    //     popup: {
                    //         popup: {
                    //             <GButton> {
                    //                 prop: {
                    //                     basic: {theme: Success}
                    //                 }
                    //             }
                    //         }
                    //     }
                    //     <GButton> {}
                    // }

                    // <GPopover> {
                    //     position: Right
                    //     <GButton> {}
                    // }

                    // <GToolTip> {
                    //     <GButton> {}
                    // }
                    // <GTabbarItem> {
                    //     prop: {
                    //         basic: {
                    //             icon: {
                    //                 svg: {
                    //                     theme: Error,
                    //                 },
                    //                 container: {
                    //                     background_visible: true,
                    //                     theme: Info,
                    //                     margin: {
                    //                         top: 10.0,
                    //                         right: 10.0
                    //                         left: 10.0,
                    //                         bottom: 10.0,
                    //                     }
                    //                 }
                    //             },
                    //             text: {
                    //                 font_size: 24.0,
                    //             }
                    //         }
                    //     }
                    //     icon: <GSvg> {
                    //         prop: {
                    //             basic: {
                    //                 svg: {
                    //                     theme: Primary
                    //                 }
                    //             }
                    //         }
                    //         src: dep("crate://self/resources/wind.svg"),
                    //     }
                    //     text: <GLabel> {
                    //         text: "Wind"
                    //     }
                    // }
                    // <GTag> {
                    //     text: <GLabel>{
                    //         text: "tag"
                    //     }
                    // }
                    
                    // <GView> {
                    //     prop: {
                    //         basic: {
                    //             height: Fill,
                    //             width: Fill,
                    //             background_visible: true,
                    //             theme: Primary,
                    //             flow: Right
                    //         }
                    //     }
                    //     // <GView> {
                    //     //     prop: {
                    //     //         basic: {
                    //     //             height: Fill,
                    //     //             width: Fill,
                    //     //             background_visible: true,
                    //     //             theme: Info,
                    //     //         }
                    //     //     }
                    //     // }
                    //     // <GView> {
                    //     //     prop: {
                    //     //         basic: {
                    //     //             height: Fill,
                    //     //             width: Fill,
                    //     //             background_visible: true,
                    //     //             theme: Error,
                    //     //         }
                    //     //     }
                    //     // }
                    //     <GTabbarItem> {
                    //         value: "wind",
                    //         icon: <GSvg> {
                    //             prop: {
                    //                 basic: {
                    //                     svg: {
                    //                         theme: Error,
                    //                     },
                    //                 }
                    //             }
                    //             src: dep("crate://self/resources/wind.svg"),
                    //         }
                    //         text: <GLabel> {
                    //             text: "Wind"
                    //         }
                    //     }
                    //     <GTabbarItem> {
                    //         value: "wind",
                    //         icon: <GSvg> {
                    //             prop: {
                    //                 basic: {
                    //                     svg: {
                    //                         theme: Error,
                    //                     },
                    //                 }
                    //             }
                    //             src: dep("crate://self/resources/wind.svg"),
                    //         }
                    //         text: <GLabel> {
                    //             text: "Wind"
                    //         }
                    //     }
                    // }

                    // <GTabbar> {
                    //     <GTabbarItem> {
                    //         value: "wind",
                    //         icon: <GSvg> {
                    //             prop: {
                    //                 basic: {
                    //                     svg: {
                    //                         theme: Error,
                    //                     },
                    //                 }
                    //             }
                    //             src: dep("crate://self/resources/wind.svg"),
                    //         }
                    //         text: <GLabel> {
                    //             text: "Wind"
                    //         }
                    //     }
                    //     <GTabbarItem> {
                    //         value: "heavy",
                    //         icon: <GSvg> {
                    //             src: dep("crate://self/resources/heavy.svg"),
                    //         }
                    //         text: <GLabel> {
                    //             text: "Heavy"
                    //         }
                    //     }
                    // }
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

                    // <GCollapse> {
                    //     prop: {
                    //         basic: {
                    //             container: {
                    //                 background_color: #fff,
                    //                 background_visible: true
                    //             }
                    //         }
                    //     },
                    //     active: true,
                    //     position: Top,
                    //     header: <GView> {

                    //         <GLabel> {
                    //             text: "Collapse Header"
                    //         }
                    //     }
                    //     body: <GView> {
                    //         <GLabel> {
                    //             text: "Collapse Body"
                    //         }
                    //     }
                    // }
                    // <GCollapse> {
                    //     prop: {
                    //         basic: {
                    //             header: {
                    //                 width: 200.0,
                    //             },
                    //             body: {
                    //                 height: 300.0,
                    //             }
                    //         }
                    //     },
                    //     position: Right,
                    //     active: true,
                    //     header: <GView> {
                    //         <GLabel> {
                    //             text: "Collapse Header"
                    //         }
                    //     }
                    //     body: <GView> {
                    //         <GLabel> {
                    //             text: "Collapse Body"
                    //         }
                    //     }
                    // }
                    // <GMenuItem> {
                    //     text: <GLabel> {
                    //         text: "Menu item"
                    //     }
                    // }
                    // <GSubMenu>{
                    //     header: <GView> {
                    //         <GLabel>{
                    //             text: "Sub Menu",
                    //         }
                    //     }
                    //     body: <GView> {
                    //         <GMenuItem> {
                    //             text: <GLabel>{
                    //                 text: "Sub Menu Item 1",
                    //             }
                    //         }
                    //         <GMenuItem> {
                    //             text: <GLabel>{
                    //                 text: "Sub Menu Item 2",
                    //             }
                    //         }
                    //     }
                    // }
                    // <GSubMenu>{
                    //     header: <GView> {
                    //         <GLabel>{
                    //             text: "Sub Menu",
                    //         }
                    //     }
                    //     body: <GView> {
                    //         <GMenuItem> {
                    //             text: <GLabel>{
                    //                 text: "Sub Menu Item 1-1",
                    //             }
                    //         }
                    //         <GSubMenu>{
                    //             header: <GView> {
                    //                 <GLabel>{
                    //                     text: "Sub Menu",
                    //                 }
                    //             }
                    //             body: <GView> {
                    //                 <GMenuItem> {
                    //                     text: <GLabel>{
                    //                         text: "Sub Menu Item 2-1",
                    //                     }
                    //                 }
                    //                 <GMenuItem> {
                    //                     text: <GLabel>{
                    //                         text: "Sub Menu Item 2-2",
                    //                     }
                    //                 }
                    //             }
                    //         }
                    //     }
                    // }

                    // <GMenu>{
                    //     body: <GView> {
                    //         <GSubMenu>{
                    //             header: <GView> {
                    //                 <GLabel>{
                    //                     text: "Sub Menu",
                    //                 }
                    //             }
                    //             body: <GView> {
                    //                 <GMenuItem> {
                    //                     text: <GLabel>{
                    //                         text: "Sub Menu Item 1-1",
                    //                     }
                    //                 }
                    //                 <GSubMenu>{
                    //                     header: <GView> {
                    //                         <GLabel>{
                    //                             text: "Sub Menu",
                    //                         }
                    //                     }
                    //                     body: <GView> {
                    //                         <GMenuItem> {
                    //                             text: <GLabel>{
                    //                                 text: "Sub Menu Item 2-1",
                    //                             }
                    //                         }
                    //                         <GMenuItem> {
                    //                             text: <GLabel>{
                    //                                 text: "Sub Menu Item 2-2",
                    //                             }
                    //                         }
                    //                     }
                    //                 }
                    //             }
                    //         }
                    //     }
                    // }

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
                        // <GMenu>{
                        //     header: <GView> {
                        //         <GLabel>{
                        //             text: "Menu Header",
                        //         }
                        //     },
                        //     body: <GView> {
                        //         <GSubMenu>{
                        //             body: {
                        //                 <GMenuItem> {
                        //                     text: <GLabel>{
                        //                         text: "Sub Menu Item 0-0",
                        //                     }
                        //                 }
                        //                 <GSubMenu>{
                        //                     header: <GView> {
                        //                         <GLabel>{
                        //                             text: "Sub Menu",
                        //                         }
                        //                     }
                        //                     body: <GView> {
                        //                         <GMenuItem> {
                        //                             text: <GLabel>{
                        //                                 text: "Sub Menu Item 0-1-0",
                        //                             }
                        //                         }
                        //                         <GMenuItem> {
                        //                             text: <GLabel>{
                        //                                 text: "Sub Menu Item 0-1-1",
                        //                             }
                        //                         }
                        //                     }
                        //                 }
                        //             }
                        //         }
                        //         <GMenuItem> {
                        //             text: <GLabel>{
                        //                 text: "Sub Menu Item 1",
                        //             }
                        //         }
                        //     },
                        //     footer: <GView>{
                        //         prop: {
                        //             basic: {
                        //                 background_visible: true,
                        //                 theme: Primary,
                        //                 // width: Fill
                        //             }
                        //         }
                        //         <GLabel> {
                        //             text: "Menu Footer"
                        //         }
                        //     }
                        // }
                        // <GMenuItem> {
                        //             text: <GLabel>{
                        //                 text: "Sub Menu Item 1",
                        //             }
                        //         }
                        // <GMenu>{
                        //     active: "sub_active",
                        //     header: <GView> {
                        //         <GLabel>{
                        //             text: "Menu Header",
                        //         }
                        //     },
                        //     body: <GView> {
                        //         <GSubMenu>{
                        //             body: {
                        //                 <GMenuItem> {
                        //                     text: <GLabel>{
                        //                         text: "Sub Menu Item 0-0",
                        //                     }
                        //                 }
                        //                 <GSubMenu>{
                        //                     header: <GView> {
                        //                         <GLabel>{
                        //                             text: "Sub Menu",
                        //                         }
                        //                     }
                        //                     body: <GView> {
                        //                         <GMenuItem> {
                        //                             text: <GLabel>{
                        //                                 text: "Sub Menu Item 0-1-0",
                        //                             }
                        //                         }
                        //                         <GMenuItem> {
                        //                             value: "sub_active",
                        //                             text: <GLabel>{
                        //                                 text: "Sub Menu Item 0-1-1",
                        //                             }
                        //                         }
                        //                     }
                        //                 }
                        //             }
                        //         }
                                // <GMenuItem> {
                                //     prop: {
                                //         basic: {
                                //             container: {
                                //                 theme: Primary,
                                //             }
                                //         }
                                //     }
                                //     text: <GLabel>{
                                //         text: "Sub Menu Item 1",
                                //     }
                                // }
                        //     },
                        //     footer: <GView>{
                        //         prop: {
                        //             basic: {
                        //                 background_visible: true,
                        //                 theme: Primary,
                        //                 // width: Fill
                        //             }
                        //         }
                        //         <GLabel> {
                        //             text: "Menu Footer"
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