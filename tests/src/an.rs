use makepad_widgets::*;
live_design! {
    use link::theme::*;
    use link::shaders::*;

    pub AButtonBase = {{AButton}} {}
    pub AButton = <AButtonBase> {
        text: ""
        width: Fit, height: Fit,
        spacing: (THEME_SPACE_2),
        align: {x: 0.5, y: 0.5},
        padding: <THEME_MSPACE_1> { left: (THEME_SPACE_2), right: (THEME_SPACE_2) }
        margin: <THEME_MSPACE_V_1> {}
        label_walk: { width: Fit, height: Fit },

        draw_text: {
            instance hover: 0.0,
            instance down: 0.0,
            instance focus: 0.0,
            instance disabled: 0.0

            color: (THEME_COLOR_LABEL_INNER)
            uniform color_hover: (THEME_COLOR_LABEL_INNER_HOVER)
            uniform color_down: (THEME_COLOR_LABEL_INNER_DOWN)
            uniform color_focus: (THEME_COLOR_LABEL_INNER_FOCUS)
            uniform color_disabled: (THEME_COLOR_LABEL_INNER_DISABLED)

            text_style: <THEME_FONT_REGULAR> {
                font_size: (THEME_FONT_SIZE_P)
            }
            fn get_color(self) -> vec4 {
                return self.color
            }
        }

        icon_walk: {
            width: (THEME_DATA_ICON_WIDTH), height: Fit,
        }

        draw_bg: {
            instance hover: 0.0
            instance down: 0.0
            instance enabled: 1.0
            instance disabled: 0.0
            instance focus: 0.0

            uniform color_dither: 1.0

            uniform border_size: (THEME_BEVELING)
            uniform border_radius: (THEME_CORNER_RADIUS)

            uniform color: (THEME_COLOR_OUTSET)
            uniform color_hover: #f00
            uniform color_down: (THEME_COLOR_OUTSET_DOWN)
            uniform color_focus: (THEME_COLOR_OUTSET_FOCUS)
            uniform color_disabled: (THEME_COLOR_OUTSET_DISABLED)

            uniform border_color_1: (THEME_COLOR_BEVEL_OUTSET_1)
            uniform border_color_1_hover: (THEME_COLOR_BEVEL_OUTSET_1_HOVER)
            uniform border_color_1_down: (THEME_COLOR_BEVEL_OUTSET_1_DOWN)
            uniform border_color_1_focus: (THEME_COLOR_BEVEL_OUTSET_1_FOCUS)
            uniform border_color_1_disabled: (THEME_COLOR_BEVEL_OUTSET_1_DISABLED)

            uniform border_color_2: (THEME_COLOR_BEVEL_OUTSET_2)
            uniform border_color_2_hover: (THEME_COLOR_BEVEL_OUTSET_2_HOVER)
            uniform border_color_2_down: (THEME_COLOR_BEVEL_OUTSET_2_DOWN)
            uniform border_color_2_focus: (THEME_COLOR_BEVEL_OUTSET_2_FOCUS)
            uniform border_color_2_disabled: (THEME_COLOR_BEVEL_OUTSET_2_DISABLED)

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size)
                let dither = Math::random_2d(self.pos.xy) * 0.04 * self.color_dither;

                let border_sz_uv = vec2(
                    self.border_size / self.rect_size.x,
                    self.border_size / self.rect_size.y
                )

                let gradient_border = vec2(
                    self.pos.x + dither,
                    self.pos.y + dither
                )

                let sz_inner_px = vec2(
                    self.rect_size.x - self.border_size * 2.,
                    self.rect_size.y - self.border_size * 2.
                );

                let scale_factor_fill = vec2(
                    self.rect_size.x / sz_inner_px.x,
                    self.rect_size.y / sz_inner_px.y
                );

                let gradient_fill = vec2(
                    self.pos.x * scale_factor_fill.x - border_sz_uv.x * 2. + dither,
                    self.pos.y * scale_factor_fill.y - border_sz_uv.y * 2. + dither
                )

                sdf.box(
                    self.border_size,
                    self.border_size,
                    self.rect_size.x - self.border_size * 2.,
                    self.rect_size.y - self.border_size * 2.,
                    self.border_radius
                )

                // sdf.fill_keep(
                //     mix(
                //         mix(
                //             mix(
                //                 mix(self.color, self.color_focus, self.focus),
                //                 self.color_hover,
                //                 self.hover
                //             ),
                //             self.color_down,
                //             self.down
                //         ),
                //         self.color_disabled,
                //         self.disabled
                //     )
                // )
                sdf.fill_keep(self.color);

                sdf.stroke(
                    mix(
                        mix(
                            mix(
                                mix(
                                    mix(self.border_color_1, self.border_color_2, gradient_border.y),
                                    mix(self.border_color_1_focus, self.border_color_2_focus, gradient_border.y),
                                    self.focus
                                ),
                                mix(self.border_color_1_hover, self.border_color_2_hover, gradient_border.y),
                                self.hover
                            ),
                            mix(self.border_color_1_down, self.border_color_2_down, gradient_border.y),
                            self.down
                        ),
                        mix(self.border_color_1_disabled, self.border_color_2_disabled, gradient_border.y),
                        self.disabled
                    ), self.border_size
                )
                return sdf.result;
            }
        }

        animator: {
            hover = {
                default: off,
                off = {
                    from: {all: Forward {duration: 0.3}}
                    apply: {
                        draw_bg: {color: #f00} // red
                        height: 60.0
                    }
                }

                on = {
                    from: {
                        all: Forward {duration: 0.3}
                        down: Forward {duration: 0.1}
                    }
                    apply: {
                        draw_bg: {color: #ff0} // yellow
                        height: 40.0
                    }
                }

                down = {
                    from: {all: Forward {duration: 0.3}}
                    apply: {
                        draw_bg: {color: #fff} // white
                        height: 90.0
                    }
                }
            }
        }
    }

}

#[derive(Live, LiveHook, Widget)]
pub struct AButton {
    #[animator]
    animator: Animator,
    #[redraw]
    #[live]
    draw_bg: DrawQuad,
    #[live]
    draw_text: DrawText,
    #[live]
    icon_walk: Walk,
    #[live]
    label_walk: Walk,
    #[walk]
    walk: Walk,
    #[layout]
    layout: Layout,
    #[live(true)]
    grab_key_focus: bool,
    #[live(true)]
    enabled: bool,
    #[live(true)]
    #[visible]
    visible: bool,
    #[live]
    pub enable_long_press: bool,
    #[live]
    reset_hover_on_click: bool,
    #[live]
    pub text: ArcStringMut,
}

impl Widget for AButton {
    fn set_disabled(&mut self, cx: &mut Cx, disabled: bool) {
        self.animator_toggle(
            cx,
            disabled,
            Animate::Yes,
            id!(disabled.on),
            id!(disabled.off),
        );
    }

    fn disabled(&self, cx: &Cx) -> bool {
        self.animator_in_state(cx, id!(disabled.on))
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let uid = self.widget_uid();
        if self.animator_handle_event(cx, event).must_redraw() {
            self.draw_bg.redraw(cx);
        }

        match event.hits(cx, self.draw_bg.area()) {
            Hit::FingerDown(fe) if self.enabled && fe.is_primary_hit() => {
                if self.grab_key_focus {
                    cx.set_key_focus(self.draw_bg.area());
                }

                self.animator_play(cx, id!(hover.down));
                self.set_key_focus(cx);
            }
            Hit::FingerHoverIn(_) => {
                // self.draw_bg.apply_over(cx, live!{
                //     color: (vec4(1.0, 0.0, 1.0, 1.0)),
                // });
                // self.apply_over(cx, live!{
                //     height: 40.0
                // });
                self.set_an(cx);
                if self.enabled {
                    cx.set_cursor(MouseCursor::Hand);
                    self.animator_play(cx, id!(hover.on));
                } else {
                    cx.set_cursor(MouseCursor::NotAllowed);
                }
                self.redraw(cx);
            }
            Hit::FingerHoverOut(_) => {
                self.animator_play(cx, id!(hover.off));
            }

            _ => (),
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        if !self.visible {
            return DrawStep::done();
        }

        self.draw_bg.begin(cx, walk, self.layout);
        self.draw_text
            .draw_walk(cx, self.label_walk, Align::default(), self.text.as_ref());
        self.draw_bg.end(cx);
        cx.add_nav_stop(self.draw_bg.area(), NavRole::TextInput, Margin::default());
        DrawStep::done()
    }

    fn text(&self) -> String {
        self.text.as_ref().to_string()
    }

    fn set_text(&mut self, cx: &mut Cx, v: &str) {
        self.text.as_mut_empty().push_str(v);
        self.redraw(cx);
    }
}

impl AButton {
    pub fn draw_button(&mut self, cx: &mut Cx2d, label: &str) {
        self.draw_bg.begin(cx, self.walk, self.layout);
        self.draw_text
            .draw_walk(cx, self.label_walk, Align::default(), label);
        self.draw_bg.end(cx);
    }
    pub fn set_an(&mut self, cx: &mut Cx) {
        // if let Some(an) = &mut self.animator.state {
        //     let mut hover_node = LiveNodeVec::new();
        //     hover_node.push_live(live_object! {
        //         draw_bg: {
        //             color: (vec4(0.0, 0.0, 0.0, 1.0)),
        //         },
        //         height: 40.0
        //     });

        //     an.replace_or_insert_last_node_by_path(
        //         0,
        //         &[
        //             live_id!(hover).as_field(),
        //             live_id!(on).as_field(),
        //             live_id!(apply).as_field(),
        //         ],
        //         &hover_node,
        //     );
        // }
        // dbg!(&self.animator.state);
        let fi = match self.animator.live_ptr {
            Some(ptr) => ptr.file_id.0,
            None => return,
        };

        let mut reg = cx.live_registry.borrow_mut();

        let live_file = match reg.live_files.get_mut(fi as usize) {
            Some(lf) => lf,
            None => return,
        };
        // dbg!("--- livefile ---");
        let nodes = &mut live_file.expanded.nodes;

        let hover_on_index = nodes
            .iter()
            .enumerate()
            .fold(None, |mut hover_on, (index, node)| {
                if node.id == live_id!(on) && !matches!(node.value, LiveValue::Close) {
                    hover_on = Some(index);
                }
                hover_on
            });
        // dbg!("hover_on_index: {:?}", hover_on_index);
        if let Some(index) = hover_on_index {
            if let Some(v) = nodes.child_by_path(
                index,
                &[
                    live_id!(apply).as_field(),
                    live_id!(draw_bg).as_field(),
                    live_id!(color).as_field(),
                    // live_id!(height).as_field(),
                ],
            ) {
                dbg!(&nodes[v].value); // 4294902015,
                // dbg!("Setting height to 40.0");
                // nodes[v].value = LiveValue::Float64(40.0);
                nodes[v].value = LiveValue::Color(rgba_to_u32(0, 0, 0, 255)); // black
            }
        }
    }
}

fn rgba_to_u32(r: u8, g: u8, b: u8, a: u8) -> u32 {
    ((r as u32) << 24) | ((g as u32) << 16) | ((b as u32) << 8) | (a as u32)
}