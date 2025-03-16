mod event;
pub mod register;
mod types;
pub use event::*;
use makepad_widgets::*;

use shader::draw_text::TextWrap;
use types::{Edit, EditKind, History};
use unicode_segmentation::{GraphemeCursor, UnicodeSegmentation};

use crate::{
    animatie_fn, event_option, ref_getter, prop_setter, ref_event_option, set_event,
    shader::{draw_text::DrawGText, draw_view::DrawGView},
    themes::Themes,
    utils::{get_font_family, BoolToF32, ThemeColor, ToBool},
    widget_area,
};

live_design! {
    link gen_base;
    use link::shaders::*;
    use link::gen_theme::*;

    pub GInputBase = {{GInput}}{
        font_family: (FONT_FAMILY),
        font_size: (FONT_SIZE),
        background_color: vec4(1.0, 1.0, 1.0, 1.0),
        hover_color: vec4(0.9, 0.9, 0.9, 1.0),
        focus_color: vec4(0.9, 0.9, 0.9, 1.0),
        text_hover_color: vec4(0.2, 0.2, 0.2, 1.0),
        text_focus_color: vec4(0.2, 0.2, 0.2, 1.0),
        shadow_offset: vec2(0.0, 0.0),
        color: #667085,
        height: Fill,
        width: 180.0,
        // align: {x: 0.0, y: 0.0},
        padding: 8.6,
        clip_x: false,
        clip_y: false,
        placeholder: "Please Input",
        text_align: {y: 0.},
        read_only: false,
        numeric_only: false,
        animator: {
            hover = {
                default: off
                off = {
                    from: {all: Forward {duration: 0.1}}
                    apply: {
                        draw_selection: {hover: 0.0},
                        draw_text: {hover: 0.0},
                        draw_input: {hover: 0.0},
                    }
                }
                on = {
                    from: {all: Snap}
                    apply: {
                        draw_selection: {hover: 1.0},
                        draw_text: {hover: 1.0},
                        draw_input: {hover: 1.0},
                    }
                }
            }
            focus = {
                default: off
                off = {
                    from: {all: Forward {duration: .25}}
                    apply: {
                        draw_cursor: {focus: 0.0},
                        draw_input: {focus: 0.0},
                        draw_selection: {focus: 0.0}
                        draw_text: {focus: 0.0}
                    }
                }
                on = {
                    from: {all: Snap}
                    apply: {
                        draw_cursor: {focus: 1.0},
                        draw_input: {focus: 1.0},
                        draw_selection: {focus: 1.0}
                        draw_text: {focus: 1.0}
                    }
                }
            }
        },

        draw_text: {
            // instance focus: 0.0;
            instance placeholder_color: vec4;
            fn get_color(self) -> vec4 {
                return mix(
                    mix(
                        self.color,
                        mix(self.stroke_hover_color, self.stroke_focus_color, self.focus),
                        self.hover
                    ),
                    self.placeholder_color,
                    self.empty
                )
            }
        }

        draw_cursor: {
            // instance focus: 0.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(
                    0.,
                    0.,
                    self.rect_size.x,
                    self.rect_size.y,
                    self.border_radius
                )
                sdf.fill(self.get_background_color());
                return sdf.result;
            }
        }

        // draw_selection: {
        //     instance hover: 0.0
        //     instance focus: 0.0

        //     fn pixel(self) -> vec4 {
        //         //return mix(#f00,#0f0,self.pos.y)
        //         let sdf = Sdf2d::viewport(self.pos * self.rect_size);
        //         sdf.box(
        //             0.,
        //             0.,
        //             self.rect_size.x,
        //             self.rect_size.y,
        //             0.5
        //         )
        //         sdf.fill(
        //             self.get_color()
        //         );
        //         return sdf.result
        //     }
        // }

    }
}

#[derive(Live, Widget)]
pub struct GInput {
    #[live]
    pub theme: Themes,
    #[live]
    pub shadow_color: Option<Vec4>,
    #[live(0.0)]
    pub spread_radius: f32,
    #[live(4.8)]
    pub blur_radius: f32,
    #[live]
    pub shadow_offset: Vec2,
    #[live]
    pub placeholder_color: Option<Vec4>,
    #[live]
    pub color: Option<Vec4>,
    #[live]
    pub cursor_color: Option<Vec4>,
    #[live]
    pub select_color: Option<Vec4>,
    #[live]
    pub background_color: Option<Vec4>,
    #[live(true)]
    pub background_visible: bool,
    #[live(true)]
    pub visible: bool,
    #[live]
    pub hover_color: Option<Vec4>,
    #[live]
    pub text_hover_color: Option<Vec4>,
    #[live]
    pub text_focus_color: Option<Vec4>,
    #[live]
    pub cursor_hover_color: Option<Vec4>,
    #[live]
    pub cursor_focus_color: Option<Vec4>,
    #[live]
    pub select_hover_color: Option<Vec4>,
    #[live]
    pub select_focus_color: Option<Vec4>,
    #[live]
    pub focus_color: Option<Vec4>,
    #[live]
    pub border_color: Option<Vec4>,
    #[live(1.0)]
    pub border_width: f32,
    #[live(2.0)]
    pub border_radius: f32,
    // text --------------------
    #[live]
    pub text_align: Align,
    #[live(9.0)]
    pub font_size: f64,
    // #[live(1.0)]
    // pub brightness: f32,
    // #[live(0.5)]
    // pub curve: f32,
    // #[live(1.2)]
    // pub top_drop: f64,
    #[live(1.3)]
    pub height_factor: f64,
    #[live(TextWrap::Word)]
    pub wrap: TextWrap,
    #[live]
    pub font_family: LiveDependency,
    #[live(1.0)]
    cursor_border_radius: f64,
    // deref --------------
    #[animator]
    animator: Animator,
    #[redraw]
    #[live]
    draw_input: DrawGView,
    #[live]
    draw_text: DrawGText,
    #[live]
    draw_selection: DrawGView,
    #[live]
    draw_cursor: DrawGView,
    #[layout]
    layout: Layout,
    #[walk]
    walk: Walk,
    #[live(2.0)]
    cursor_width: f64,
    #[live]
    pub read_only: bool,
    #[live]
    pub numeric_only: bool,
    #[live]
    pub placeholder: String,
    #[live]
    pub text: String,
    #[rust]
    cursor: Cursor,
    #[rust]
    history: History,
    #[live]
    scroll_bars: ScrollBars,
    #[live(true)]
    pub event_key: bool,
}

impl Widget for GInput {
    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        if !self.visible {
            return DrawStep::done();
        }
        // self.draw_text.wrap = self.wrap.clone();
        let _ = get_font_family(&self.font_family, cx, &mut self.draw_text.text_style.font);
        self.draw_input.begin(cx, walk, self.layout);

        self.draw_selection.append_to_draw_call(cx);

        let inner_walk = self.inner_walk();
        // Draw text
        if self.text.is_empty() {
            self.draw_text.empty = 1.0;
            self.draw_text
                .draw_walk(cx, inner_walk, self.text_align, &self.placeholder);
        } else {
            self.draw_text.empty = 0.0;
            self.draw_text
                .draw_walk(cx, inner_walk, self.text_align, &self.text);
        }

        let padded_rect = cx.turtle().padded_rect();

        // Draw selection
        let rects = self.draw_text.selected_rects(
            cx,
            inner_walk,
            self.text_align,
            padded_rect.size.x,
            &self.text,
            self.cursor.head.min(self.cursor.tail),
            self.cursor.head.max(self.cursor.tail),
        );
        for rect in rects {
            self.draw_selection.draw_abs(
                cx,
                Rect {
                    pos: padded_rect.pos + rect.pos,
                    size: rect.size,
                },
            );
        }

        // Draw cursor
        let cursor_position = self.cursor_position(cx, padded_rect.size.x);
        let cursor_height = self.draw_text.line_height(cx);
        self.draw_cursor.draw_abs(
            cx,
            Rect {
                pos: padded_rect.pos
                    + dvec2(
                        cursor_position.x - 0.5 * self.cursor_width,
                        cursor_position.y,
                    ),
                size: dvec2(self.cursor_width, cursor_height),
            },
        );

        self.draw_input.end(cx);

        if cx.has_key_focus(self.draw_input.area()) {
            let padding = dvec2(self.layout.padding.left, self.layout.padding.top);
            cx.show_text_ime(
                self.draw_input.area(),
                padding + cursor_position - self.cursor_width * 0.5,
            );
        }

        cx.add_nav_stop(
            self.draw_input.area(),
            NavRole::TextInput,
            Margin::default(),
        );

        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let rect = self.draw_input.area().rect(cx);
        let padded_rect = Rect {
            pos: rect.pos + self.layout.padding.left_top(),
            size: rect.size - self.layout.padding.size(),
        };

        let uid = self.widget_uid();

        if self.animator_handle_event(cx, event).must_redraw() {
            self.draw_input.redraw(cx);
        }

        match event.hits(cx, self.draw_input.area()) {
            Hit::KeyFocus(e) => {
                self.animator_play(cx, id!(focus.on));
                self.force_new_edit_group();
                // TODO: Select all if necessary
                cx.widget_action(uid, &scope.path, GInputEvent::KeyFocus(e));
            }
            Hit::KeyFocusLost(e) => {
                self.animator_play(cx, id!(focus.off));
                cx.hide_text_ime();
                cx.widget_action(uid, &scope.path, GInputEvent::KeyFocusLost(e));
            }
            Hit::KeyDown(KeyEvent {
                key_code: KeyCode::ArrowLeft,
                modifiers: KeyModifiers {
                    shift: is_select, ..
                },
                ..
            }) => {
                self.move_cursor_left(is_select);
                self.draw_input.redraw(cx);
            }
            Hit::KeyDown(KeyEvent {
                key_code: KeyCode::ArrowRight,
                modifiers: KeyModifiers {
                    shift: is_select, ..
                },
                ..
            }) => {
                self.move_cursor_right(is_select);
                self.draw_input.redraw(cx);
            }
            Hit::KeyDown(KeyEvent {
                key_code: KeyCode::ArrowUp,
                modifiers: KeyModifiers {
                    shift: is_select, ..
                },
                ..
            }) => {
                let event = DrawEvent::default();
                let mut cx = Cx2d::new(cx, &event);
                self.move_cursor_up(&mut cx, padded_rect.size.x, is_select);
                self.draw_input.redraw(&mut cx);
            }
            Hit::KeyDown(KeyEvent {
                key_code: KeyCode::ArrowDown,
                modifiers: KeyModifiers {
                    shift: is_select, ..
                },
                ..
            }) => {
                let event = DrawEvent::default();
                let mut cx = Cx2d::new(cx, &event);
                self.move_cursor_down(&mut cx, padded_rect.size.x, is_select);
                self.draw_input.redraw(&mut cx);
            }
            Hit::KeyDown(KeyEvent {
                key_code: KeyCode::Home,
                ..
            }) => {
                self.move_cursor_to(
                    IndexAffinity {
                        index: 0,
                        affinity: Affinity::Before,
                    },
                    false,
                );
                self.history.force_new_edit_group();
                self.draw_input.redraw(cx);
            }
            Hit::KeyDown(KeyEvent {
                key_code: KeyCode::End,
                ..
            }) => {
                self.move_cursor_to(
                    IndexAffinity {
                        index: self.text.len(),
                        affinity: Affinity::After,
                    },
                    false,
                );
                self.history.force_new_edit_group();
                self.draw_input.redraw(cx);
            }
            Hit::KeyDown(KeyEvent {
                key_code: KeyCode::ReturnKey,
                modifiers: KeyModifiers { shift: false, .. },
                ..
            }) => {
                cx.hide_text_ime();
                cx.widget_action(
                    uid,
                    &scope.path,
                    GInputEvent::Changed(GInputChangedParam {
                        text: self.text.clone(),
                        ty: InputEventType::KeyDown(KeyCode::ReturnKey),
                        modifiers: Some(KeyModifiers {
                            shift: false,
                            ..Default::default()
                        }),
                    }),
                );
            }
            Hit::KeyDown(KeyEvent {
                key_code: KeyCode::ReturnKey,
                modifiers: KeyModifiers { shift: true, .. },
                ..
            }) if !self.read_only => {
                self.history
                    .create_or_extend_edit_group(EditKind::Other, self.cursor);
                self.apply_edit(Edit {
                    start: self.cursor.start().index,
                    end: self.cursor.end().index,
                    replace_with: "\n".to_string(),
                });
                self.draw_input.redraw(cx);
                cx.widget_action(
                    uid,
                    &scope.path,
                    GInputEvent::Changed(GInputChangedParam {
                        text: self.text.clone(),
                        ty: InputEventType::KeyDown(KeyCode::ReturnKey),
                        modifiers: Some(KeyModifiers {
                            shift: true,
                            ..Default::default()
                        }),
                    }),
                );
            }
            Hit::KeyDown(KeyEvent {
                key_code: KeyCode::Escape,
                is_repeat,
                modifiers,
                time,
            }) => {
                cx.widget_action(
                    uid,
                    &scope.path,
                    GInputEvent::Escaped(KeyEvent {
                        key_code: KeyCode::Escape,
                        is_repeat,
                        modifiers,
                        time,
                    }),
                );
            }
            Hit::KeyDown(KeyEvent {
                key_code: KeyCode::Backspace,
                ..
            }) if !self.read_only => {
                let mut start = self.cursor.start().index;
                let end = self.cursor.end().index;
                if start == end {
                    start = prev_grapheme_boundary(&self.text, start).unwrap_or(0);
                }
                self.history
                    .create_or_extend_edit_group(EditKind::Backspace, self.cursor);
                self.apply_edit(Edit {
                    start,
                    end,
                    replace_with: String::new(),
                });
                self.draw_input.redraw(cx);
                cx.widget_action(
                    uid,
                    &scope.path,
                    GInputEvent::Changed(GInputChangedParam {
                        text: self.text.clone(),
                        ty: InputEventType::KeyDown(KeyCode::Backspace),
                        modifiers: None,
                    }),
                );
            }
            Hit::KeyDown(KeyEvent {
                key_code: KeyCode::Delete,
                ..
            }) if !self.read_only => {
                let start = self.cursor.start().index;
                let mut end = self.cursor.end().index;
                if start == end {
                    end = next_grapheme_boundary(&self.text, end).unwrap_or(self.text.len());
                }
                self.history
                    .create_or_extend_edit_group(EditKind::Delete, self.cursor);
                self.apply_edit(Edit {
                    start,
                    end,
                    replace_with: String::new(),
                });
                self.draw_input.redraw(cx);
                cx.widget_action(
                    uid,
                    &scope.path,
                    GInputEvent::Changed(GInputChangedParam {
                        text: self.text.clone(),
                        ty: InputEventType::KeyDown(KeyCode::Delete),
                        modifiers: None,
                    }),
                );
            }
            Hit::KeyDown(KeyEvent {
                key_code: KeyCode::KeyA,
                modifiers: KeyModifiers { control: true, .. },
                ..
            })
            | Hit::KeyDown(KeyEvent {
                key_code: KeyCode::KeyA,
                modifiers: KeyModifiers { logo: true, .. },
                ..
            }) => {
                self.select_all();
                self.draw_input.redraw(cx);
            }
            Hit::KeyDown(KeyEvent {
                key_code: KeyCode::KeyZ,
                modifiers,
                ..
            }) if modifiers.is_primary() && !modifiers.shift && !self.read_only => {
                self.undo();
                self.draw_input.redraw(cx);
                cx.widget_action(
                    uid,
                    &scope.path,
                    GInputEvent::Changed(GInputChangedParam {
                        text: self.text.clone(),
                        ty: InputEventType::KeyDown(KeyCode::KeyZ),
                        modifiers: Some(modifiers),
                    }),
                );
            }
            Hit::KeyDown(KeyEvent {
                key_code: KeyCode::KeyZ,
                modifiers,
                ..
            }) if modifiers.is_primary() && modifiers.shift && !self.read_only => {
                self.redo();
                self.draw_input.redraw(cx);
                cx.widget_action(
                    uid,
                    &scope.path,
                    GInputEvent::Changed(GInputChangedParam {
                        text: self.text.clone(),
                        ty: InputEventType::KeyDown(KeyCode::KeyZ),
                        modifiers: Some(modifiers),
                    }),
                );
            }
            Hit::KeyDown(ke) => {
                cx.widget_action(uid, &scope.path, GInputEvent::KeyDownUnhandled(ke));
            }
            Hit::TextInput(TextInputEvent {
                input,
                replace_last,
                was_paste,
                ..
            }) if !self.read_only => {
                let input = self.filter_input(input);
                if !input.is_empty() {
                    let mut start = self.cursor.start().index;
                    let end = self.cursor.end().index;
                    if replace_last {
                        start -= self
                            .history
                            .last_inserted_text(&self.text)
                            .map_or(0, |text| text.len());
                    }
                    self.history.create_or_extend_edit_group(
                        if replace_last || was_paste {
                            EditKind::Other
                        } else {
                            EditKind::Insert
                        },
                        self.cursor,
                    );
                    self.apply_edit(Edit {
                        start,
                        end,
                        replace_with: input,
                    });
                    self.draw_input.redraw(cx);
                    cx.widget_action(
                        uid,
                        &scope.path,
                        GInputEvent::Changed(GInputChangedParam {
                            text: self.text.clone(),
                            ty: InputEventType::Input,
                            modifiers: None,
                        }),
                    );
                }
            }
            Hit::TextCopy(event) => {
                let selection = &self.text[self.cursor.start().index..self.cursor.end().index];
                *event.response.borrow_mut() = Some(selection.to_string());
            }
            Hit::TextCut(event) => {
                let selection = &self.text[self.cursor.start().index..self.cursor.end().index];
                *event.response.borrow_mut() = Some(selection.to_string());
                if !selection.is_empty() {
                    self.history
                        .create_or_extend_edit_group(EditKind::Other, self.cursor);
                    self.apply_edit(Edit {
                        start: self.cursor.start().index,
                        end: self.cursor.end().index,
                        replace_with: String::new(),
                    });
                    self.draw_input.redraw(cx);
                    cx.widget_action(
                        uid,
                        &scope.path,
                        GInputEvent::Changed(GInputChangedParam {
                            text: self.text.clone(),
                            ty: InputEventType::Cut,
                            modifiers: None,
                        }),
                    );
                }
            }
            Hit::FingerHoverIn(_) => {
                cx.set_cursor(MouseCursor::Text);
                self.animator_play(cx, id!(hover.on));
            }
            Hit::FingerHoverOut(_) => {
                self.animator_play(cx, id!(hover.off));
            }
            Hit::FingerDown(FingerDownEvent { abs, tap_count, .. }) => {
                let event = DrawEvent::default();
                let mut cx = Cx2d::new(cx, &event);
                let index_affinity = self.position_to_index_affinity(
                    &mut cx,
                    padded_rect.size.x,
                    abs - padded_rect.pos,
                );
                self.move_cursor_to(index_affinity, false);
                if tap_count == 2 {
                    self.select_word();
                } else if tap_count == 3 {
                    self.select_all();
                }
                self.set_key_focus(&mut *cx);
                self.draw_input.redraw(&mut *cx);
            }
            Hit::FingerMove(FingerMoveEvent { abs, tap_count, .. }) => {
                let event: DrawEvent = DrawEvent::default();
                let mut cx = Cx2d::new(cx, &event);
                let index_affinity = self.position_to_index_affinity(
                    &mut cx,
                    padded_rect.size.x,
                    abs - padded_rect.pos,
                );
                self.move_cursor_to(index_affinity, true);
                if tap_count == 2 {
                    self.select_word();
                } else if tap_count == 3 {
                    self.select_all();
                }
                self.draw_input.redraw(&mut *cx);
            }
            _ => {}
        }
    }
    fn text(&self) -> String {
        self.text.to_string()
    }

    fn set_text(&mut self, cx: &mut Cx, text: &str) {
        if self.text == text {
            return;
        }
        self.text = self.filter_input(text.to_string());
        self.cursor.head.index = self.cursor.head.index.min(text.len());
        self.cursor.tail.index = self.cursor.tail.index.min(text.len());
        self.history.clear();
        self.redraw(cx);
    }

    fn is_visible(&self) -> bool {
        self.visible
    }
}

impl LiveHook for GInput {
    fn after_apply_from_doc(&mut self, cx: &mut Cx) {
        if !self.visible {
            return;
        }
        if let Err(e) = self.render(cx) {
            error!("GInput render error: {:?}", e);
        }
    }
}

impl GInput {
    pub fn render(&mut self, cx: &mut Cx) -> Result<(), Box<dyn std::error::Error>> {
        // ----------------- background color -------------------------------------------
        let bg_color = self.background_color.get(self.theme, 25);
        // ------------------ hover color -----------------------------------------------
        let hover_color = self.hover_color.get(self.theme, 25);
        let shadow_color = self.shadow_color.get(self.theme, 700);
        let text_hover_color = self.text_hover_color.get(self.theme, 600);
        let text_focus_color = self.text_focus_color.get(self.theme, 800);
        let cursor_color = self.cursor_color.get(self.theme, 800);
        let cursor_hover_color = self.cursor_hover_color.get(self.theme, 800);
        let cursor_focus_color = self.cursor_focus_color.get(self.theme, 800);
        let select_color = self.select_color.get(self.theme, 50);
        let select_hover_color = self.select_hover_color.get(self.theme, 100);
        let select_focus_color = self.select_focus_color.get(self.theme, 200);
        let placeholder_color = self.placeholder_color.use_or("#98A2B3")?;
        // ------------------ focus color ---------------------------------------------
        let focus_color = self.focus_color.get(self.theme, 25);
        // ------------------ border color ----------------------------------------------
        let border_color = self.border_color.get(self.theme, 400);
        // ------------------ font ------------------------------------------------------
        let font_color = self.color.get(self.theme, 800);
        // ---------------------- is empty ------------------------------------------------
        let empty = self.text.len().eq(&0).to_f32();
        // draw input --------------------------------------------------------------
        self.draw_input.apply_over(
            cx,
            live! {
                background_color: (bg_color),
                background_visible: (self.background_visible.to_f32()),
                border_color: (border_color),
                border_width: (self.border_width),
                border_radius: (self.border_radius),
                focus_color: (focus_color),
                hover_color: (hover_color),
                shadow_color: (shadow_color),
                shadow_offset: (self.shadow_offset),
                spread_radius: (self.spread_radius),
                blur_radius: (self.blur_radius)
            },
        );
        // draw text ---------------------------------------------------------------
        self.draw_text.apply_over(
            cx,
            live! {
                color: (font_color),
                stroke_hover_color: (text_hover_color),
                stroke_focus_color: (text_focus_color),
                placeholder_color:(placeholder_color),
                empty: (empty),
                text_style: {
                    // brightness: (self.brightness),
                    // curve: (self.curve),
                    // line_spacing: (self.layout.line_spacing),
                    // top_drop: (self.top_drop),
                    font_size: (self.font_size),
                    // height_factor: (self.height_factor),
                }
            },
        );
        self.draw_text.wrap = self.wrap.clone();
        // draw cursor -------------------------------------------------------------
        self.draw_cursor.apply_over(
            cx,
            live! {
                background_color: (cursor_color),
                // border_color: (border_color),
                // border_width: (self.border_width),
                border_radius: (self.cursor_border_radius),
                focus_color: (cursor_focus_color),
                hover_color: (cursor_hover_color),
            },
        );
        // draw select -------------------------------------------------------------
        self.draw_selection.apply_over(
            cx,
            live! {
                background_color: (select_color),
                background_visible: 1.0,
                focus_color: (select_focus_color),
                hover_color: (select_hover_color),
                border_radius: 0.0
            },
        );

        Ok(())
    }
    fn inner_walk(&self) -> Walk {
        if self.walk.width.is_fit() {
            Walk::fit()
        } else {
            Walk::fill_fit()
        }
    }
    widget_area! {
        area, draw_input,
        area_selection, draw_selection
    }
    event_option! {
        changed: GInputEvent::Changed => GInputChangedParam,
        escaped: GInputEvent::Escaped => KeyEvent,
        key_down_unhandled: GInputEvent::KeyDownUnhandled => KeyEvent,
        key_focus: GInputEvent::KeyFocus => KeyFocusEvent,
        key_focus_lost: GInputEvent::KeyFocusLost => KeyFocusEvent
    }
    pub fn animate_hover_on(&mut self, cx: &mut Cx) -> () {
        self.draw_input.apply_over(
            cx,
            live! {
                hover: 1.0,
                focus: 0.0
            },
        );
        self.draw_cursor.apply_over(
            cx,
            live! {
                hover: 1.0,
                focus: 0.0
            },
        );
        self.draw_selection.apply_over(
            cx,
            live! {
                hover: 1.0,
                focus: 0.0
            },
        );
        self.draw_text.apply_over(
            cx,
            live! {
                hover: 1.0,
                focus: 0.0
            },
        );
    }
    pub fn animate_hover_off(&mut self, cx: &mut Cx) -> () {
        self.draw_input.apply_over(
            cx,
            live! {
                hover: 0.0,
                focus: 0.0
            },
        );
        self.draw_cursor.apply_over(
            cx,
            live! {
                hover: 0.0,
                focus: 0.0
            },
        );
        self.draw_selection.apply_over(
            cx,
            live! {
                hover: 0.0,
                focus: 0.0
            },
        );
        self.draw_text.apply_over(
            cx,
            live! {
                hover: 0.0,
                focus: 0.0
            },
        );
    }
    pub fn animate_focus_on(&mut self, cx: &mut Cx) -> () {
        self.draw_input.apply_over(
            cx,
            live! {
                hover: 0.0,
                focus: 1.0
            },
        );
        self.draw_cursor.apply_over(
            cx,
            live! {
                hover: 0.0,
                focus: 1.0
            },
        );
        self.draw_selection.apply_over(
            cx,
            live! {
                hover: 0.0,
                focus: 1.0
            },
        );
        self.draw_text.apply_over(
            cx,
            live! {
                hover: 0.0,
                focus: 1.0
            },
        );
    }

    pub fn animate_focus_off(&mut self, cx: &mut Cx) -> () {
        self.draw_input.apply_over(
            cx,
            live! {
                hover: 0.0,
                focus: 0.0
            },
        );
        self.draw_cursor.apply_over(
            cx,
            live! {
                hover: 0.0,
                focus: 0.0
            },
        );
        self.draw_selection.apply_over(
            cx,
            live! {
                hover: 0.0,
                focus: 0.0
            },
        );
        self.draw_text.apply_over(
            cx,
            live! {
                hover: 0.0,
                focus: 0.0
            },
        );
    }

    pub fn set_key_focus(&self, cx: &mut Cx) {
        cx.set_key_focus(self.draw_input.area());
    }

    pub fn set_input_cursor(&mut self, cursor: Cursor) {
        self.cursor = cursor;
    }

    pub fn select_all(&mut self) {
        self.set_input_cursor(Cursor {
            head: IndexAffinity {
                index: self.text.len(),
                affinity: Affinity::After,
            },
            tail: IndexAffinity {
                index: 0,
                affinity: Affinity::Before,
            },
        });
    }

    pub fn filter_input(&mut self, input: String) -> String {
        if self.numeric_only {
            input
                .chars()
                .filter_map(|char| match char {
                    '.' | ',' => Some('.'),
                    char if char.is_ascii_digit() => Some(char),
                    _ => None,
                })
                .collect()
        } else {
            input
        }
    }

    pub fn force_new_edit_group(&mut self) {
        self.history.force_new_edit_group();
    }

    fn position_to_index_affinity(
        &self,
        cx: &mut Cx2d,
        width: f64,
        position: DVec2,
    ) -> IndexAffinity {
        self.draw_text.position_to_index_affinity(
            cx,
            Walk::fill(),
            self.text_align,
            width,
            &self.text,
            position,
        )
    }

    fn cursor_position(&self, cx: &mut Cx2d, width: f64) -> DVec2 {
        self.draw_text.index_affinity_to_position(
            cx,
            Walk::fill(),
            self.text_align,
            width,
            &self.text,
            self.cursor.head,
        )
    }

    fn move_cursor_left(&mut self, is_select: bool) {
        let Some(index) = prev_grapheme_boundary(&self.text, self.cursor.head.index) else {
            return;
        };
        self.move_cursor_to(
            IndexAffinity {
                index,
                affinity: Affinity::After,
            },
            is_select,
        );
    }

    fn move_cursor_right(&mut self, is_select: bool) {
        let Some(index) = next_grapheme_boundary(&self.text, self.cursor.head.index) else {
            return;
        };
        self.move_cursor_to(
            IndexAffinity {
                index,
                affinity: Affinity::Before,
            },
            is_select,
        );
    }

    fn move_cursor_up(&mut self, cx: &mut Cx2d, width: f64, is_select: bool) {
        let position = self.cursor_position(cx, width);
        let line_spacing = self.draw_text.line_spacing(cx);
        let index_affinity = self.position_to_index_affinity(
            cx,
            width,
            DVec2 {
                x: position.x,
                y: position.y - 0.5 * line_spacing,
            },
        );
        self.move_cursor_to(index_affinity, is_select)
    }

    fn move_cursor_down(&mut self, cx: &mut Cx2d, width: f64, is_select: bool) {
        let position = self.cursor_position(cx, width);
        let line_spacing = self.draw_text.line_spacing(cx);
        let index_affinity = self.position_to_index_affinity(
            cx,
            width,
            DVec2 {
                x: position.x,
                y: position.y + 1.5 * line_spacing,
            },
        );
        self.move_cursor_to(index_affinity, is_select);
    }

    fn move_cursor_to(&mut self, index_affinity: IndexAffinity, is_select: bool) {
        self.cursor.head = index_affinity;
        if !is_select {
            self.cursor.tail = self.cursor.head;
        }
        self.history.force_new_edit_group();
    }

    fn select_word(&mut self) {
        if self.cursor.head.index < self.cursor.tail.index {
            self.cursor.head = IndexAffinity {
                index: self.ceil_word_boundary(self.cursor.head.index),
                affinity: Affinity::After,
            };
        } else if self.cursor.head.index > self.cursor.tail.index {
            self.cursor.head = IndexAffinity {
                index: self.floor_word_boundary(self.cursor.head.index),
                affinity: Affinity::Before,
            };
        } else {
            self.cursor.tail = IndexAffinity {
                index: self.ceil_word_boundary(self.cursor.head.index),
                affinity: Affinity::After,
            };
            self.cursor.head = IndexAffinity {
                index: self.floor_word_boundary(self.cursor.head.index),
                affinity: Affinity::Before,
            };
        }
    }

    fn ceil_word_boundary(&self, index: usize) -> usize {
        let mut prev_word_boundary_index = 0;
        for (word_boundary_index, _) in self.text.split_word_bound_indices() {
            if word_boundary_index > index {
                return prev_word_boundary_index;
            }
            prev_word_boundary_index = word_boundary_index;
        }
        prev_word_boundary_index
    }

    fn floor_word_boundary(&self, index: usize) -> usize {
        let mut prev_word_boundary_index = self.text.len();
        for (word_boundary_index, _) in self.text.split_word_bound_indices().rev() {
            if word_boundary_index < index {
                return prev_word_boundary_index;
            }
            prev_word_boundary_index = word_boundary_index;
        }
        prev_word_boundary_index
    }

    fn apply_edit(&mut self, edit: Edit) {
        self.cursor.head.index = edit.start + edit.replace_with.len();
        self.cursor.tail = self.cursor.head;
        self.history.apply_edit(edit, &mut self.text);
    }

    fn undo(&mut self) {
        if let Some(cursor) = self.history.undo(self.cursor, &mut self.text) {
            self.cursor = cursor;
        }
    }

    fn redo(&mut self) {
        if let Some(cursor) = self.history.redo(self.cursor, &mut self.text) {
            self.cursor = cursor;
        }
    }
}

impl GInputRef {
    prop_setter! {
        GInput{
            set_theme(theme: Themes) {|c_ref| {c_ref.theme = theme; Ok(())}},
            set_shadow_color(color: String) {|c_ref| {c_ref.shadow_color.replace(crate::utils::hex_to_vec4(&color)?); Ok(())}},
            set_spread_radius(radius: f32) {|c_ref| {c_ref.spread_radius = radius; Ok(())}},
            set_blur_radius(radius: f32) {|c_ref| {c_ref.blur_radius = radius; Ok(())}},
            set_shadow_offset(offset: Vec2) {|c_ref| {c_ref.shadow_offset = offset; Ok(())}},
            set_placeholder_color(color: String) {|c_ref| {c_ref.placeholder_color.replace(crate::utils::hex_to_vec4(&color)?); Ok(())}},
            set_color(color: String) {|c_ref| {c_ref.color.replace(crate::utils::hex_to_vec4(&color)?); Ok(())}},
            set_cursor_color(color: String) {|c_ref| {c_ref.cursor_color.replace(crate::utils::hex_to_vec4(&color)?); Ok(())}},
            set_select_color(color: String) {|c_ref| {c_ref.select_color.replace(crate::utils::hex_to_vec4(&color)?); Ok(())}},
            set_background_color(color: String) {|c_ref| {c_ref.background_color.replace(crate::utils::hex_to_vec4(&color)?); Ok(())}},
            set_background_visible(visible: bool) {|c_ref| {c_ref.background_visible = visible; Ok(())}},
            set_visible(visible: bool) {|c_ref| {c_ref.visible = visible; Ok(())}},
            set_hover_color(color: String) {|c_ref| {c_ref.hover_color.replace(crate::utils::hex_to_vec4(&color)?); Ok(())}},
            set_text_hover_color(color: String) {|c_ref| {c_ref.text_hover_color.replace(crate::utils::hex_to_vec4(&color)?); Ok(())}},
            set_text_focus_color(color: String) {|c_ref| {c_ref.text_focus_color.replace(crate::utils::hex_to_vec4(&color)?); Ok(())}},
            set_cursor_hover_color(color: String) {|c_ref| {c_ref.cursor_hover_color.replace(crate::utils::hex_to_vec4(&color)?); Ok(())}},
            set_cursor_focus_color(color: String) {|c_ref| {c_ref.cursor_focus_color.replace(crate::utils::hex_to_vec4(&color)?); Ok(())}},
            set_select_hover_color(color: String) {|c_ref| {c_ref.select_hover_color.replace(crate::utils::hex_to_vec4(&color)?); Ok(())}},
            set_select_focus_color(color: String) {|c_ref| {c_ref.select_focus_color.replace(crate::utils::hex_to_vec4(&color)?); Ok(())}},
            set_focus_color(color: String) {|c_ref| {c_ref.focus_color.replace(crate::utils::hex_to_vec4(&color)?); Ok(())}},
            set_border_color(color: String) {|c_ref| {c_ref.border_color.replace(crate::utils::hex_to_vec4(&color)?); Ok(())}},
            set_border_width(width: f32) {|c_ref| {c_ref.border_width = width; Ok(())}},
            set_border_radius(radius: f32) {|c_ref| {c_ref.border_radius = radius; Ok(())}},
            // set_text_align(align: Align) {|c_ref| {c_ref.text_align = align; Ok(())}},
            set_font_size(size: f64) {|c_ref| {c_ref.font_size = size; Ok(())}},
            set_height_factor(factor: f64) {|c_ref| {c_ref.height_factor = factor; Ok(())}},
            set_wrap(wrap: TextWrap) {|c_ref| {c_ref.wrap = wrap; Ok(())}},
            // set_font_family(font_family: LiveDependency) {|c_ref| {c_ref.font_family = font_family; Ok(())}},
            set_cursor_border_radius(radius: f32) {|c_ref| {c_ref.cursor_border_radius = radius as f64; Ok(())}},
            set_cursor_width(width: f64) {|c_ref| {c_ref.cursor_width = width; Ok(())}},
            set_read_only(read_only: bool) {|c_ref| {c_ref.read_only = read_only; Ok(())}},
            set_numeric_only(numeric_only: bool) {|c_ref| {c_ref.numeric_only = numeric_only; Ok(())}},
            set_placeholder(placeholder: String) {|c_ref| {c_ref.placeholder = placeholder; Ok(())}},
            set_text(text: String) {|c_ref| {c_ref.text = text; Ok(())}},
            set_cursor(cursor: Cursor) {|c_ref| {c_ref.cursor = cursor; Ok(())}},
            set_event_key(event_key: bool) {|c_ref| {c_ref.event_key = event_key; Ok(())}},
            set_abs_pos(abs_pos: DVec2) {|c_ref| {c_ref.walk.abs_pos.replace(abs_pos); Ok(())}},
            set_margin(margin: Margin) {|c_ref| {c_ref.walk.margin = margin; Ok(())}},
            set_height(height: Size) {|c_ref| {c_ref.walk.height = height; Ok(())}},
            set_width(width: Size) {|c_ref| {c_ref.walk.width = width; Ok(())}},
            set_scroll(scroll: DVec2) {|c_ref| {c_ref.layout.scroll = scroll; Ok(())}},
            set_clip_x(clip_x: bool) {|c_ref| {c_ref.layout.clip_x = clip_x; Ok(())}},
            set_clip_y(clip_y: bool) {|c_ref| {c_ref.layout.clip_y = clip_y; Ok(())}},
            set_padding(padding: Padding) {|c_ref| {c_ref.layout.padding = padding; Ok(())}},
            set_align(align: Align) {|c_ref| {c_ref.layout.align = align; Ok(())}},
            set_flow(flow: Flow) {|c_ref| {c_ref.layout.flow = flow; Ok(())}},
            set_spacing(spacing: f64) {|c_ref| {c_ref.layout.spacing = spacing; Ok(())}}
        }
    }
    ref_getter! {
        GInput{
            get_theme(Themes) {||Themes::default()}, {|c_ref| {c_ref.theme}},
            get_shadow_color(String) {||Default::default()}, {|c_ref| {crate::utils::vec4_to_hex(&c_ref.draw_input.shadow_color)}},
            get_spread_radius(f32) {||1.0}, {|c_ref| {c_ref.draw_input.spread_radius}},
            get_blur_radius(f32) {||4.8}, {|c_ref| {c_ref.draw_input.blur_radius}},
            get_shadow_offset(Vec2) {||Vec2::default()}, {|c_ref| {c_ref.draw_input.shadow_offset}},
            get_placeholder_color(String) {||"#98A2B3".to_string()}, {|c_ref| {c_ref.placeholder_color.as_ref().map_or("#98A2B3".to_string(), |v| crate::utils::vec4_to_hex(v))}},
            get_color(String) {||Default::default()}, {|c_ref| {crate::utils::vec4_to_hex(&c_ref.draw_text.color)}},
            get_cursor_color(String) {||Default::default()}, {|c_ref| {crate::utils::vec4_to_hex(&c_ref.draw_cursor.background_color)}},
            get_select_color(String) {||Default::default()}, {|c_ref| {crate::utils::vec4_to_hex(&c_ref.draw_selection.background_color)}},
            get_background_color(String) {||Default::default()}, {|c_ref| {crate::utils::vec4_to_hex(&c_ref.draw_input.background_color)}},
            get_background_visible(bool) {||true}, {|c_ref| {c_ref.draw_input.background_visible.to_bool()}},
            get_visible(bool) {||true}, {|c_ref| {c_ref.visible}},
            get_hover_color(String) {||Default::default()}, {|c_ref| {crate::utils::vec4_to_hex(&c_ref.draw_input.hover_color)}},
            get_text_hover_color(String) {||Default::default()}, {|c_ref| {crate::utils::vec4_to_hex(&c_ref.draw_text.stroke_hover_color)}},
            get_text_focus_color(String) {||Default::default()}, {|c_ref| {crate::utils::vec4_to_hex(&c_ref.draw_text.stroke_focus_color)}},
            get_cursor_hover_color(String) {||Default::default()}, {|c_ref| {crate::utils::vec4_to_hex(&c_ref.draw_cursor.hover_color)}},
            get_cursor_focus_color(String) {||Default::default()}, {|c_ref| {crate::utils::vec4_to_hex(&c_ref.draw_cursor.focus_color)}},
            get_select_hover_color(String) {||Default::default()}, {|c_ref| {crate::utils::vec4_to_hex(&c_ref.draw_selection.hover_color)}},
            get_select_focus_color(String) {||Default::default()}, {|c_ref| {crate::utils::vec4_to_hex(&c_ref.draw_selection.focus_color)}},
            get_focus_color(String) {||Default::default()}, {|c_ref| {crate::utils::vec4_to_hex(&c_ref.draw_input.focus_color)}},
            get_border_color(String) {||Default::default()}, {|c_ref| {crate::utils::vec4_to_hex(&c_ref.draw_input.border_color)}},
            get_border_width(f32) {||1.0}, {|c_ref| {c_ref.draw_input.border_width}},
            get_border_radius(f32) {||2.0}, {|c_ref| {c_ref.draw_input.border_radius}},
            // get_text_align(Align) {||Align::default()}, {|c_ref| {c_ref.draw_text.text_style}},
            get_font_size(f64) {||9.0}, {|c_ref| {c_ref.draw_text.text_style.font_size}},
            get_height_factor(f64) {||1.3}, {|c_ref| {c_ref.draw_text.text_style.height_factor}},
            get_wrap(TextWrap) {||TextWrap::Word}, {|c_ref| {c_ref.draw_text.wrap.clone()}},
            // get_font_family(LiveDependency) {||LiveDependency::default()}, {|c_ref| {c_ref.draw_text.text_style.font}},
            get_cursor_border_radius(f32) {||1.0}, {|c_ref| {c_ref.draw_cursor.border_radius}},
            get_cursor_width(f64) {||2.0}, {|c_ref| {c_ref.cursor_width}},
            get_read_only(bool) {||false}, {|c_ref| {c_ref.read_only}},
            get_numeric_only(bool) {||false}, {|c_ref| {c_ref.numeric_only}},
            get_placeholder(String) {||String::default()}, {|c_ref| {c_ref.placeholder.to_string()}},
            get_text(String) {||String::default()}, {|c_ref| {c_ref.text.to_string()}},
            get_cursor(Cursor) {||Cursor::default()}, {|c_ref| {c_ref.cursor}},
            get_event_key(bool) {||true}, {|c_ref| {c_ref.event_key}},
            get_abs_pos(Option<DVec2>) {||None}, {|c_ref| {c_ref.walk.abs_pos}},
            get_margin(Margin) {||Margin::default()}, {|c_ref| {c_ref.walk.margin}},
            get_height(Size) {||Size::default()}, {|c_ref| {c_ref.walk.height}},
            get_width(Size) {||Size::default()}, {|c_ref| {c_ref.walk.width}},
            get_scroll(DVec2) {||DVec2::default()}, {|c_ref| {c_ref.layout.scroll}},
            get_clip_x(bool) {||true}, {|c_ref| {c_ref.layout.clip_x}},
            get_clip_y(bool) {||true}, {|c_ref| {c_ref.layout.clip_y}},
            get_padding(Padding) {||Padding::default()}, {|c_ref| {c_ref.layout.padding}},
            get_align(Align) {||Align::default()}, {|c_ref| {c_ref.layout.align}},
            get_flow(Flow) {||Flow::default()}, {|c_ref| {c_ref.layout.flow}},
            get_spacing(f64) {||0.0}, {|c_ref| {c_ref.layout.spacing}}
        }
    }
    ref_event_option! {
        changed => GInputChangedParam,
        key_focus => KeyFocusEvent,
        key_focus_lost => KeyFocusEvent,
        escaped => KeyEvent,
        key_down_unhandled => KeyEvent
    }
    animatie_fn! {
        animate_hover_on,
        animate_hover_off,
        animate_focus_on,
        animate_focus_off
    }
    // pub fn changed(&self, actions: &Actions) -> Option<GInputChangedParam> {
    //     if let GInputEvent::Changed(val) = actions.find_widget_action_cast(self.widget_uid()) {
    //         return Some(val);
    //     }
    //     None
    // }

    // pub fn returned(&self, actions: &Actions) -> Option<String> {
    //     if let GInputEvent::Return(val) = actions.find_widget_action_cast(self.widget_uid()) {
    //         return Some(val);
    //     }
    //     None
    // }

    pub fn set_input_cursor(&self, head: usize, tail: usize) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_input_cursor(Cursor {
                head: IndexAffinity {
                    index: head,
                    affinity: Affinity::After,
                },
                tail: IndexAffinity {
                    index: tail,
                    affinity: Affinity::Before,
                },
            });
        }
    }

    pub fn set_key_focus(&self, cx: &mut Cx) {
        if let Some(inner) = self.borrow() {
            inner.set_key_focus(cx);
        }
    }
}

impl GInputSet {
    set_event! {
        changed => GInputChangedParam,
        key_focus => KeyFocusEvent,
        key_focus_lost => KeyFocusEvent,
        escaped => KeyEvent,
        key_down_unhandled => KeyEvent
    }
}

fn next_grapheme_boundary(string: &str, index: usize) -> Option<usize> {
    let mut cursor = GraphemeCursor::new(index, string.len(), true);
    cursor.next_boundary(string, 0).unwrap()
}

fn prev_grapheme_boundary(string: &str, index: usize) -> Option<usize> {
    let mut cursor = GraphemeCursor::new(index, string.len(), true);
    cursor.prev_boundary(string, 0).unwrap()
}
