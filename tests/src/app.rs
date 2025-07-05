use luna_ui::{
    components::{
        button::{ButtonState, LButtonWidgetRefExt},
        label::LLabelWidgetRefExt,
        traits::{BasicProp, Component}, view::LViewWidgetRefExt,
    },
    prop::traits::ToColor,
    themes::Theme,
};
use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::luna_ui::*;
    use link::shaders::*;
    use link::widgets::*;
    use crate::an::*;

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
                    // <LButton>{
                    //         prop: {
                    //             basic: {
                    //                 theme: Error,
                    //             }
                    //             hover: {
                    //                 theme: Primary,
                    //             }
                    //         }
                    //         disabled: true,
                    //         slot: <LLabel>{
                    //             text: "Button",
                    //             mode: Regular,
                    //         }
                    //     }
                        // btn1 = <LButton>{
                        //     prop: {
                        //         hover: {
                        //             theme: Warning,
                        //         }

                        //     }
                        //     slot: <LLabel>{
                        //         text: "Click me !",
                        //         mode: Bold,
                        //     }
                        // }

                    //     lb = <LLabel>{
                    //         text: "Hello World!",
                    //         mode: Bold
                    //         prop: {
                    //             basic: {
                    //                 font_size: 24.0,
                    //             }
                    //         }
                    //     }
                    //     lb2 = <LLabel>{
                    //         text: "Hello World! hello",
                    //         mode: Bold
                    //         disabled: true,
                    //         prop: {
                    //             basic: {
                    //                 font_size: 24.0,
                    //             }
                    //         }
                    //     }
                    // <LView>{
                    //     prop: {
                    //         basic: {
                    //             border_color: #f00,
                    //             border_width: 2.0,
                    //             height: 100.0,
                    //             width: 100.0,
                    //             background_visible: true,
                    //         }
                    //     }
                    // }
                    // <LView>{
                    //     prop: {
                    //         basic: {
                    //             theme: Primary,
                    //             height: 100.0,
                    //             width: 100.0,
                    //             background_visible: true,
                    //             border_radius: {left: 30.0, right: 30.0, top: 30.0, bottom: 30.0},
                    //             // border_radius: {left: 1.0, right: 1.0, top: 1.0, bottom: 1.0},
                    //             border_width: 2.0,
                    //             // border_color: #f00,
                    //             // padding: { top: 10.0, bottom: 10.0, left: 10.0, right: 10.0 },
                    //         }
                    //     }
                    //     animation_open: true
                    // }
                    // <View> {
                    //     flow: Down,
                    //     spacing: 10.0,
                    //     height: 200.0,
                    //     width: 200.0,
                    //     show_bg: true,
                    //     draw_bg: {
                    //         color: #f0f
                    //     }
                    //     padding: { top: 10.0, bottom: 10.0, left: 10.0, right: 10.0 },
                    //     <View> {
                    //         <LLabel>{
                    //             text: "Card Header",
                    //             mode: Bold
                    //         }
                    //         show_bg: true,
                    //         draw_bg: {
                    //             color: #f00
                    //         }
                    //         height: 40.0,
                    //         width: Fill,
                    //     }
                    //     <View> {
                    //         <LLabel>{
                    //             text: "Card Body",
                    //             mode: Bold
                    //         }
                    //         show_bg: true,
                    //         draw_bg: {
                    //             color: #000
                    //         }
                    //         height: Fill,
                    //         width: Fill,
                    //     }
                    //     <View> {
                    //         <LLabel>{
                    //             text: "Card Footer",
                    //             mode: Bold
                    //         }
                    //         show_bg: true,
                    //         draw_bg: {
                    //             color: #00f
                    //         }
                    //         height: 40.0,
                    //         width: Fill,
                    //     }
                    // }
                    // <LView> {
                    //     prop: {
                    //         basic: {
                    //             height: 200.0,
                    //             width: 200.0,
                    //         }
                    //     }
                    //     <LView> {
                    //         <LLabel>{
                    //             text: "Card Header",
                    //             mode: Bold
                    //         }
                    //         prop: {
                    //             basic: {
                    //                 theme: Primary,
                    //                 height: 40.0,
                    //                 width: Fill,
                    //             }
                    //         }
                    //     }
                    //     <LView> {
                    //         <LLabel>{
                    //             text: "Card Body",
                    //             mode: Bold
                    //         }
                    //         prop: {
                    //             basic: {
                    //                 theme: Error,
                    //                 height: Fill,
                    //             }
                    //         }
                    //     }
                    //     <LView> {
                    //         <LLabel>{
                    //             text: "Card Footer",
                    //             mode: Bold
                    //         }
                    //         prop: {
                    //             basic: {
                    //                 theme: Warning,
                    //                 height: 40.0,
                    //             }
                    //         }
                    //     }
                    // }
                    
                    // <LCard>{
                    //     prop: {
                    //         basic: {
                    //             outer: {
                    //                 theme: Primary,
                    //                 height: 200.0,
                    //                 width: 200.0,
                    //             },
                    //             header: {
                    //                 border_width: 2.0,
                    //                 border_color: #00f,
                    //             }
                    //             footer: {
                    //                 theme: Error,
                    //                 height: 40.0,
                    //                 border_width: 2.0,
                    //                 border_color: #f00,
                    //             }
                    //         }
                    //     }
                    //     header: {
                    //         prop: {
                    //             basic: {
                    //                 theme: Warning,
                    //                 height: 30.0
                    //             }
                    //         }
                    //         <LLabel>{
                    //             text: "Card Header",
                    //             mode: Bold
                    //         }
                    //     }
                    //     body: {
                    //         <LLabel>{
                    //             text: "Card Body",
                    //             mode: Bold
                    //         }
                    //         <LButton>{
                    //             slot: {
                    //                 text: "Click Me!",
                    //             }
                    //         }
                    //     }
                    //     footer: {
                    //         prop: {
                    //             basic: {
                    //                 theme: Success,
                    //                 height: Fill
                    //             }
                    //         }
                    //         <LButton>{
                    //             slot: {
                    //                 text: "Click Me!",
                    //             }
                    //         }
                    //     }
                        
                    //     // body: <LView>{
                    //     //     prop: {
                    //     //         basic: {
                    //     //             background_visible: true,
                    //     //             background_color: #f0f,
                    //     //             border_radius: {left: 10.0, right: 10.0, top: 10.0, bottom: 10.0},
                    //     //             border_width: 2.0,
                    //     //             border_color: #00f,
                    //     //         }
                    //     //     }
                    //     //     <LLabel>{
                    //     //         text: "Card Body",
                    //     //         mode: Bold
                    //     //     }
                    //     // }
                    // }
                    // v1 = <LView>{
                    //     prop: {
                    //         basic: {
                    //             theme: Primary,
                    //             height: 200.0,
                    //             width: 200.0,
                    //         }
                    //         hover: {
                    //             theme: Warning,
                    //         }
                    //     }
                    //     animation_open: true
                    //     event_open: true,
                    //     btn1 = <LButton>{
                    //         slot: {
                    //             text: "Click Me!",
                    //         }
                    //     }
                    // }
                    <LButton>{
                        prop: {
                            basic: {
                                theme: Primary,
                                height: 50.0,
                                width: 200.0,
                            }
                            hover: {
                                theme: Warning,
                            }
                        }
                        slot: <LLabel>{
                            text: "Click Me!",
                            mode: Bold
                        }
                    }
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
    }
}

impl MatchEvent for App {
    fn handle_startup(&mut self, _cx: &mut Cx) {}

    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        let v1 = self.ui.lview(id!(v1));
        if let Some(_) = v1.lbutton(id!(btn1)).clicked(actions) {
            if let Some(v1) = v1.borrow() {
                dbg!(&v1.prop.hover.theme);
                dbg!(&v1.apply_state_map);
            }
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
