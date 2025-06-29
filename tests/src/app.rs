use luna_ui::{
    components::{
        button::{ButtonState, LButtonWidgetRefExt},
        label::LLabelWidgetRefExt,
        traits::{BasicProp, Component},
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
                    // draw_bg:{
                    //     fn pixel(self) -> vec4 {

                    //         let center = vec2(0.5, 0.5);
                    //         let uv = self.pos - center;
                    //         let radius = length(uv);
                    //         let angle = atan(uv.y, uv.x);
                    //         let color1 = mix(#f00, #00f, 0.5 + 10.5 * cos(angle + self.time));
                    //         let color2 = mix(#0f0, #ff0, 0.5 + 0.5 * sin(angle + self.time));
                    //         let color = mix(color1, color2, radius);
                    //         return depth_clip(self.world, color, self.depth_clip);
                    //     }
                    // }
                    // <Rotary>{
                    //     text:"Slide"
                    // }
                    // button_1 = <Button> {
                    //     text: "Click 福 me 😊"
                    //     draw_text:{text_style:{font_size:18}}
                    // }
                    // <TypingAnimation>{}
                    // <AButton>{
                    //     text: "asdsdasd"
                    // }
                    // <LView> {
                    //     // prop: {
                    //     //     basic: {
                    //     //         background_visible: true,
                    //     //         background_color: #000,
                    //     //         theme: Primary,
                    //     //         height: 200.0,
                    //     //         width: 200.0,
                    //     //         shadow_offset: vec2(0.0, 0.0),
                    //     //         align: {
                    //     //             x: 0.5,
                    //     //             y: 0.5
                    //     //         },
                    //     //     }
                    //     // }
                    //     <LLabel>{
                    //         text: "Hello World",
                    //         prop: {
                    //             basic: {
                    //                 color: #ff0,
                    //                 font_size: 24.0,

                    //             }
                    //         }
                    //     }


                    // }

                        // btn1 = <LButton>{
                        //     prop: {
                        //         basic: {
                        //             theme: Info,
                        //             border_width: 2.0,
                        //             border_color: #ff0,
                        //         }
                        //         hover: {
                        //             theme: Success,
                        //             border_color: #0f0,
                        //             border_width: 4.0,
                        //         },
                        //         pressed: {
                        //             theme: Error,
                        //         }
                        //     }
                        //     slot: <Label>{
                        //         text: "Click me !"
                        //     }
                        // }
                    
                    <LButton>{
                            prop: {
                                basic: {
                                    theme: Error,
                                }
                                hover: {
                                    theme: Primary,
                                }
                            }
                            disabled: true,
                            slot: <LLabel>{
                                text: "Button",
                                mode: Regular,
                            }
                        }
                        btn1 = <LButton>{
                            prop: {
                                hover: {
                                    theme: Warning,
                                }

                            }
                            slot: <LLabel>{
                                text: "Click me !",
                                mode: Bold,
                            }
                        }

                        lb = <LLabel>{
                            text: "Hello World!",
                            mode: Bold
                            prop: {
                                basic: {
                                    font_size: 24.0,
                                }
                            }
                        }
                        lb2 = <LLabel>{
                            text: "Hello World! hello",
                            mode: Bold
                            disabled: true,
                            prop: {
                                basic: {
                                    font_size: 24.0,
                                }
                            }
                        }
                    <LView>{
                        prop: {
                            basic: {
                                border_color: #f00,
                                border_width: 2.0,
                                height: 100.0,
                                width: 100.0,
                                background_visible: true,
                            }
                        }
                    }
                    <LView>{
                        prop: {
                            basic: {
                                theme: Primary,
                                height: 100.0,
                                width: 100.0,
                                background_visible: true,
                                border_radius: {left: 30.0, right: 30.0, top: 30.0, bottom: 30.0},
                                // border_radius: {left: 1.0, right: 1.0, top: 1.0, bottom: 1.0},
                                border_width: 2.0,
                                // border_color: #f00,
                                // padding: { top: 10.0, bottom: 10.0, left: 10.0, right: 10.0 },
                            }
                        }
                        animation_open: true
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
        // if self.ui.button(id!(button_1)).clicked(&actions) {
        //     self.ui.button(id!(button_1)).set_text(cx, "Clicked 😀");
        //     log!("hi");
        //     self.counter += 1;
        // }
        let lb = self.ui.llabel(id!(lb));
        let btn1 = self.ui.lbutton(id!(btn1));
        if let Some(_) = btn1.clicked(actions) {
            self.counter += 1;
            // lb.set_text(cx, format!("Clicked {} times", self.counter));
            let _ = lb.set_color(cx, "#f00".to_string());
            let _ = lb.set_font_size(cx, 18.0);

            let _ = btn1.set_background_color(cx, "#0f0".to_string());
            lb.redraw(cx);
        }

        // let lb = self.ui.llabel(id!(lb));

        // if self.ui.lbutton(id!(btn1)).cli
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
