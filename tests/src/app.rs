use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::luna_ui::*;
    use link::shaders::*;
    use link::widgets::*;


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
                    <LView> {
                        height: 200.0,
                        width: 200.0,
                        prop: {
                            basic: {
                                spread_radius: 2.0,
                                blur_radius: 12.0,
                                shadow_offset: vec2(6.0, 6.0),
                                shadow_color: #ff0000,
                                align: {
                                    x: 0.5,
                                    y: 0.5
                                },
                            }
                        }
                        <LLabel>{
                            text: "Hello World",
                            prop: {
                                color: #ff0,
                                font_size: 24.0,
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
        crate::luna_ui::live_design(cx);
    }
}

impl MatchEvent for App {
    fn handle_startup(&mut self, _cx: &mut Cx) {}

    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        if self.ui.button(id!(button_1)).clicked(&actions) {
            self.ui.button(id!(button_1)).set_text(cx, "Clicked 😀");
            log!("hi");
            self.counter += 1;
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
