use luna_ui::{
    components::{
        button::{ButtonState, GButtonWidgetRefExt},
        label::GLabelWidgetRefExt,
        traits::{BasicProp, Component},
        view::GViewWidgetRefExt,
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
                    draw_bg: {
                        color: #444,
                    }
                    // <GButton>{
                    //         prop: {
                    //             basic: {
                    //                 theme: Error,
                    //             }
                    //             hover: {
                    //                 theme: Primary,
                    //             }
                    //         }
                    //         disabled: true,
                    //         slot: <GLabel>{
                    //             text: "Button",
                    //             mode: Regular,
                    //         }
                    //     }
                    //     btn1 = <GButton>{
                    //         prop: {
                    //             hover: {
                    //                 theme: Warning,
                    //             }

                    //         }
                    //         slot: <GLabel>{
                    //             text: "Click me !",
                    //             mode: Bold,
                    //         }
                    //     }

                    //     lb = <GLabel>{
                    //         text: "Hello World!",
                    //         mode: Bold
                    //         prop: {
                    //             basic: {
                    //                 font_size: 24.0,
                    //             }
                    //         }
                    //     }
                    //     lb2 = <GLabel>{
                    //         text: "Hello World! hello",
                    //         mode: Bold
                    //         disabled: true,
                    //         prop: {
                    //             basic: {
                    //                 font_size: 24.0,
                    //             }
                    //         }
                    //     }



                //     <GCard>{
                //         prop: {
                //             basic: {
                //                 container: {
                //                     theme: Primary,
                //                     height: 200.0,
                //                     width: 200.0,
                //                 },
                //                 header: {
                //                     border_width: 2.0,
                //                     border_color: #00f,
                //                 }
                //                 footer: {
                //                     theme: Error,
                //                     height: 40.0,
                //                     border_width: 2.0,
                //                     border_color: #f00,
                //                 }
                //             }
                //         }

                //     }
                //    <GCard>{}
                    <GRadioGroup> {
                        <GRadio> {
                            prop: {
                                basic: {
                                    radio: {
                                        theme: Warning,
                                        mode: Cross,
                                    }
                                }
                            }

                            extra: {
                                <GLabel>{
                                    text: "Radio Cross",
                                    mode: Bold,
                                }
                            }
                        }
                        <GRadio>{
                            active: true,
                        }
                        <GRadio> {
                            prop: {
                                basic: {
                                    container: {
                                        background_visible: true,
                                    },
                                    radio: {
                                        theme: Primary,
                                        mode: Tick,

                                    }
                                }
                            }
                            extra: {
                                 <GLabel> {
                                    text: "Radio Tick"
                                }
                            }
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
        // let v1 = self.ui.GView(id!(v1));
        // if let Some(_) = v1.lbutton(id!(btn1)).clicked(actions) {
        //     if let Some(v1) = v1.borrow() {
        //         dbg!(&v1.prop.hover.theme);
        //         dbg!(&v1.apply_state_map);
        //     }
        // }
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
