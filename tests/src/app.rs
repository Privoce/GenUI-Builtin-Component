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
                        color: #999,
                    }
                    // <GView> {
                    //     prop: {
                    //         basic: {
                    //             theme: Info,
                    //             height: 200.0,
                    //             width: 200.0,
                    //             background_visible: true,
                    //         }
                    //         hover: {
                    //             theme: Warning,
                    //         }
                    //     },
                    //     animation_open: true
                    // }
                    // <GButton>{
                    //         prop: {
                    //             basic: {
                    //                 theme: Error,
                    //             }
                    //             hover: {
                    //                 theme: Primary,
                    //             }
                    //             disabled: {
                    //                 theme: Info,
                    //                 padding: {
                    //                     top: 20.0,
                    //                     bottom: 20.0,
                    //                     left: 20.0,
                    //                     right: 20.0,
                    //                 }
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

                        // lb = <GLabel>{
                        //     text: "Hello World!",
                        //     mode: Bold
                        //     prop: {
                        //         basic: {
                        //             font_size: 24.0,
                        //         }
                        //     }
                        // }
                        // lb2 = <GLabel>{
                        //     text: "Hello World! hello",
                        //     mode: Bold
                        //     disabled: true,
                        //     prop: {
                        //         basic: {
                        //             font_size: 24.0,
                        //         }
                        //     }
                        // }



                    // <GCard>{
                    //     prop: {
                    //         basic: {
                    //             container: {
                    //                 theme: Primary,
                    //                 height: 200.0,
                    //                 width: 200.0,
                    //                 background_visible: true,

                    //             },
                    //             header: {
                    //                 border_width: 2.0,
                    //                 border_color: #00f,
                    //                 background_visible: true,
                    //             },
                    //             footer: {
                    //                 theme: Error,
                    //                 height: 40.0,
                    //                 border_width: 2.0,
                    //                 border_color: #f00,
                    //                 background_visible: true,
                    //             }
                    //         }
                    //     }

                    // }

                //    <GCard>{}
                    // <GRadioGroup> {
                    //     active: "2",
                    //     <GRadio> {
                    //         prop: {
                    //             basic: {
                    //                 radio: {
                    //                     theme: Warning,
                    //                     mode: Cross,
                    //                 }
                    //             }
                    //         }

                    //         extra: {
                    //             <GLabel>{
                    //                 text: "Radio Cross",
                    //                 mode: Bold,
                    //             }
                    //         }
                    //     }
                    //     <GRadio>{
                    //         active: true,
                    //     }
                    //     <GRadio> {
                    //         prop: {
                    //             basic: {
                    //                 container: {
                    //                     background_visible: true,
                    //                     theme: Info,
                    //                 },
                    //                 radio: {
                    //                     theme: Primary,
                    //                     mode: Tick,

                    //                 }
                    //             },
                    //             hover: {
                    //                 radio: {
                    //                     theme: Warning,
                    //                 }
                    //             }
                    //         },
                    //         extra: {
                    //              <GLabel> {
                    //                 text: "Radio Tick"
                    //             }
                    //         }
                    //     }
                    // }
                    // <GCheckboxGroup>{
                    //     active: ["coffee", "milk"],
                    //     <GCheckbox>{
                    //         prop: {
                    //             basic: {
                    //                 checkbox: {
                    //                     theme: Warning,
                    //                     mode: Cross,
                    //                 }
                    //             }
                    //         },
                    //         value: "coffee",
                    //         extra: {
                    //             <GLabel>{
                    //                 text: "coffee"
                    //             }
                    //         }
                    //     }
                    //     <GCheckbox>{
                    //         value: "tea",
                    //         extra: {
                    //             <GLabel>{
                    //                 text: "tea"
                    //             }
                    //         }
                    //     }
                    //     <GCheckbox>{
                    //         value: "milk",
                    //         extra: {
                    //             <GLabel>{
                    //                 text: "milk"
                    //             }
                    //         }
                    //     }
                    // }
                    // <GSwitch>{
                    //     prop: {
                    //         basic: {
                    //             size: 60.0
                    //         }
                    //     },
                    // }
                    // <GSwitch>{
                    //     prop: {
                    //         basic: {
                    //             theme: Warning,
                    //             size: 60.0,
                    //             border_radius: {
                    //                 top: 15.0,
                    //                 left: 15.0,
                    //                 bottom: 15.0,
                    //                 right: 15.0,
                    //             }
                    //         }
                    //     }
                    //     disabled: true,
                    //     value: true
                    // }
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
                    // <GSvg> {
                    //     prop: {
                    //         basic: {
                    //             container: {background_visible: true, theme: Primary}
                    //             svg: {height: 36.0, width: 36.0},
                    //         }
                    //     }
                    //     src: dep("crate://self/resources/heavy.svg"),
                    // }
                    // <GSvg> {
                    //     prop: {
                    //         basic: {
                    //             container: {background_visible: true, theme: Primary}
                    //             svg: {height: 36.0, width: 36.0},
                    //         }
                    //     },
                    //     disabled: true,
                    //     src: dep("crate://self/resources/rain.svg"),
                    // }
                    // <GImage> {
                    //     src: Live(dep("crate://self/resources/install.png")),
                    //     prop: {
                    //         basic: {
                    //             height: 300.0,
                    //             width: 500.0,
                    //         }
                    //     }
                    // }
                    // <GImage> {
                    //     // src: Url("https://genui.privoce.com/genui.png"),
                    //     src: File("/Users/shengyifei/projects/gen_ui/components/tests/resources/bg.png"),
                    //     // src: Live(dep("/Users/shengyifei/projects/gen_ui/components/tests/resources/bg.png")),

                    // }
                    // <GImage> {
                    //     src: Url("https://aisearch.bj.bcebos.com/homepage/input_panel/aisearch_online.png"),
                    //     prop: {
                    //         basic: {
                    //             height: 100.0,
                    //             width: 200.0,
                    //         }
                    //     }
                    // }
                    // <GImage> {
                    //     src: Url("https://miro.medium.com/v2/resize:fit:1200/0*fmpeXj1eUS-Nrkkv.png"),
                    //     prop: {
                    //         basic: {
                    //             height: 100.0,
                    //             width: 200.0,
                    //         }
                    //     }
                    // }
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
                    <GTabbarItem> {
                        prop: {
                            basic: {
                                icon: {
                                    svg: {
                                        theme: Error,
                                    },
                                    container: {
                                        background_visible: true,
                                        theme: Info,
                                        margin: {
                                            top: 10.0
                                        }
                                    }
                                },
                                text: {
                                    font_size: 24.0,
                                }
                            }
                        }
                        icon: <GSvg> {
                            prop: {
                                basic: {
                                    svg: {
                                        theme: Primary
                                    }
                                }
                            }
                            src: dep("crate://self/resources/wind.svg"),
                        }
                        text: <GLabel> {
                            text: "Wind"
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
