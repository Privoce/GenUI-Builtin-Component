pub mod register;

use makepad_widgets::*;

use crate::{
    shader::{
        draw_popup::DrawGPopup,
        manual::{CloseMode, PopupMode, Position},
    },
    themes::Themes,
    utils::{BoolToF32, ThemeColor, ToBool},
};

use super::view::GView;

live_design! {
    link gen_base;

    pub GPopupContainerBase = {{GPopupContainer}} {
        height: Fill,
        width: Fill,
        animation_key: false,
        background_visible: false,
    }
    pub GPopupBase = {{GPopup}} {}
}
#[derive(Live, LiveRegister)]
pub struct GPopupContainer {
    // #[live]
    #[deref]
    pub deref_widget: GView,
}

impl LiveHook for GPopupContainer {
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.deref_widget.after_apply(cx, apply, index, nodes);
    }
}

impl GPopupContainer {
    pub fn area(&self) -> Area {
        self.area
    }
    pub fn draw_item(&mut self, cx: &mut Cx2d, scope: &mut Scope) {
        let _ = self.deref_widget.draw_walk(cx, scope, self.walk);
    }
    pub fn draw_item_drawer(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) {
        let _ = self.deref_widget.draw_walk(cx, scope, walk);
    }
    pub fn handle_event_with(
        &mut self,
        cx: &mut Cx,
        event: &Event,
        sweep_area: Area,
        scope: &mut Scope,
    ) {
        let _ = self
            .deref_widget
            .handle_event_with(cx, event, scope, sweep_area);
    }
    pub fn redraw(&mut self, cx: &mut Cx) {
        self.deref_widget.redraw(cx);
    }
}

/// A popup is a floating window that appears on top of other content
/// It can be used to display an additional information or to ask for a confirmation
#[derive(Live, LiveRegister)]
pub struct GPopup {
    #[live]
    pub theme: Themes,
    #[live(0.6)]
    pub opacity: f32,
    #[live]
    pub background_color: Option<Vec4>,
    #[live]
    pub hover_color: Option<Vec4>,
    #[live]
    pub focus_color: Option<Vec4>,
    #[live]
    pub border_color: Option<Vec4>,
    #[live(0.0)]
    pub border_width: f32,
    #[live(0.0)]
    pub border_radius: f32,
    #[live(true)]
    pub visible: bool,
    #[live(true)]
    pub background_visible: bool,
    #[live]
    pub cursor: Option<MouseCursor>,
    #[live]
    pub mode: PopupMode,
    #[live]
    pub close_mode: CloseMode,
    #[live]
    pub shadow_color: Option<Vec4>,
    #[live(0.0)]
    pub spread_radius: f32,
    #[live(10.0)]
    pub blur_radius: f32,
    #[live]
    pub shadow_offset: Vec2,
    // deref ---------------------
    #[live]
    pub draw_popup: DrawGPopup,
    #[walk]
    pub walk: Walk,
    #[layout]
    pub layout: Layout,
    #[live]
    pub container: GPopupContainer,
    /// draw list is necessary!!!
    /// because we need to draw the popup on top of everything
    /// although the name of DrawList2d may let you think it's only for 2d list drawing
    /// actually it's for all the drawing that needs to be on top of everything!!!
    #[live]
    draw_list: DrawList2d,
    #[rust]
    pub container_walk: Option<Walk>,
}

impl LiveHook for GPopup {
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.draw_popup.after_apply(cx, apply, index, nodes)
    }
    fn after_apply_from_doc(&mut self, cx: &mut Cx) {
        if !self.visible {
            return;
        }
        if let Err(e) = self.render(cx) {
            error!("GPopup render error: {:?}", e);
        }
    }
}

impl GPopup {
    pub fn render(&mut self, cx: &mut Cx) -> Result<(), Box<dyn std::error::Error>> {
        // ----------------- background color -------------------------------------------
        let bg_color = self.background_color.get(self.theme, 500);
        let shadow_color = self.shadow_color.get(self.theme, 500);
        // ------------------ hover color -----------------------------------------------
        let hover_color = self.hover_color.get(self.theme, 400);
        // ------------------ focus color ---------------------------------------------
        let focus_color = self.focus_color.get(self.theme, 600);
        // ------------------ border color ----------------------------------------------
        let border_color = self.border_color.get(self.theme, 800);
        // ------------------ is background_visible --------------------------------------------
        let background_visible = self.background_visible.to_f32();
        // ------------------ apply draw_popup --------------------------------------------
        self.draw_popup.apply_over(
            cx,
            live! {
                opacity: (self.opacity),
                background_color: (bg_color),
                border_color: (border_color),
                border_width: (self.border_width),
                border_radius: (self.border_radius),
                focus_color: (focus_color),
                hover_color: (hover_color),
                background_visible: (background_visible),
                shadow_color: (shadow_color),
                shadow_offset: (self.shadow_offset),
                spread_radius: (self.spread_radius),
                blur_radius: (self.blur_radius)
            },
        );
        Ok(())
    }
    pub fn area(&self) -> Area {
        self.draw_popup.area()
    }
    /// ## Get the popup container position
    /// if you need to get the position of the popup container, use this method
    /// ### Example
    /// ```rust
    /// let global = cx.global::<PopupMenuGlobal>().clone();
    /// let mut map = global.map.borrow_mut();
    /// let popup_menu = map.get_mut(&self.popup.unwrap()).unwrap();
    /// if let Event::MouseDown(e) = event {
    ///     if !popup_menu.menu_contains_pos(cx, e.abs) {
    ///         self.close(cx);
    ///         self.animator_play(cx, id!(hover.off));
    ///         return;
    ///     }
    /// }
    /// ```
    pub fn menu_contains_pos(&self, cx: &mut Cx, pos: DVec2) -> bool {
        self.draw_popup.area().clipped_rect(cx).contains(pos)
    }
    pub fn container_contains_pos(&self, cx: &mut Cx, pos: DVec2) -> bool {
        self.container.area().clipped_rect(cx).contains(pos)
    }
    /// ## Begin to draw popup
    /// this method is used to begin drawing the popup
    pub fn begin(&mut self, cx: &mut Cx2d) {
        self.draw_list.begin_overlay_reuse(cx);
        cx.begin_pass_sized_turtle(Layout::flow_down());
        self.draw_popup.begin(cx, self.walk, self.layout);
    }
    /// ## End to draw popup
    pub fn end(&mut self, cx: &mut Cx2d, _scope: &mut Scope, shift_area: Area, shift: DVec2) {
        self.draw_popup.end(cx);
        cx.end_pass_sized_turtle_with_shift(shift_area, shift);
        self.draw_list.end(cx);
    }
    pub fn redraw_container(&mut self, cx: &mut Cx) {
        self.draw_popup.redraw(cx);
        self.container.redraw(cx);
    }
    pub fn redraw(&mut self, cx: &mut Cx) {
        self.draw_list.redraw(cx);
        // self.draw_popup.redraw(cx);
    }
    /// ## Draw items
    pub fn draw_container(
        &mut self,
        cx: &mut Cx2d,
        scope: &mut Scope,
        position: Option<Position>,
        angle_offset: f32,
        redraw: &mut bool,
    ) {
        let _ = position.map(|position| {
            self.draw_popup.position = position;
        });
        self.draw_popup.angle_offset = angle_offset;
        self.container.draw_item(cx, scope);
        if *redraw {
            self.draw_popup.redraw(cx);
            *redraw = !*redraw;
        }
    }
    pub fn draw_container_drawer(
        &mut self,
        cx: &mut Cx2d,
        scope: &mut Scope,
        position: Position,
        proportion: f32,
        redraw: &mut bool,
    ) {
        self.draw_popup.position = position;
        let w = Walk {
            height: Size::All,
            width: Size::All,
            ..Default::default()
        };
        let popup_size = cx.peek_walk_turtle(w).size;
        // now get virtual box as rect
        let (adjust_size, adjust_pos) = match position {
            Position::Left | Position::LeftTop | Position::LeftBottom => {
                let x = if proportion > 1.0 {
                    proportion as f64
                } else {
                    proportion as f64 * popup_size.x
                };
                let size = DVec2 { x, y: popup_size.y };
                let pos = DVec2 { x: 0.0, y: 0.0 };
                (size, pos)
            }
            Position::Right | Position::RightTop | Position::RightBottom => {
                let x = if proportion > 1.0 {
                    proportion as f64
                } else {
                    proportion as f64 * popup_size.x
                };
                let size = DVec2 { x, y: popup_size.y };
                let pos = DVec2 {
                    x: (1.0 - proportion) as f64 * popup_size.x,
                    y: 0.0,
                };
                (size, pos)
            }
            Position::Top | Position::TopLeft | Position::TopRight => {
                let y = if proportion > 1.0 {
                    proportion as f64
                } else {
                    proportion as f64 * popup_size.y
                };
                let size = DVec2 { x: popup_size.x, y };
                let pos = DVec2 { x: 0.0, y: 0.0 };
                (size, pos)
            }
            Position::Bottom | Position::BottomLeft | Position::BottomRight => {
                let y = if proportion > 1.0 {
                    proportion as f64
                } else {
                    proportion as f64 * popup_size.y
                };
                let size = DVec2 { x: popup_size.x, y };
                let pos = DVec2 {
                    x: 0.0,
                    y: (1.0 - proportion) as f64 * popup_size.y,
                };
                (size, pos)
            }
        };

        self.container_walk.replace(Walk {
            abs_pos: Some(adjust_pos),
            width: Size::Fixed(adjust_size.x),
            height: Size::Fixed(adjust_size.y),
            ..Default::default()
        });

        self.container
            .draw_item_drawer(cx, scope, self.container_walk.unwrap());

        if *redraw {
            self.draw_popup.redraw(cx);
            *redraw = !*redraw;
        }
    }

    pub fn container_area(&self) -> Area {
        self.container.area()
    }
    pub fn handle_event_with(
        &mut self,
        cx: &mut Cx,
        event: &Event,
        scope: &mut Scope,
        sweep_area: Area,
    ) {
        self.container
            .handle_event_with(cx, event, sweep_area, scope)
    }
    pub fn get(&self) -> &GPopupContainer {
        &self.container
    }
    pub fn get_mut(&mut self) -> &mut GPopupContainer {
        &mut self.container
    }
    pub fn get_theme(&self) -> Themes {
        self.theme
    }
    pub fn set_theme(
        &mut self,
        cx: &mut Cx,
        theme: Themes,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.theme = theme;
        self.render(cx)
    }
    pub fn get_opacity(&self) -> f32 {
        self.opacity
    }
    pub fn set_opacity(
        &mut self,
        cx: &mut Cx,
        opacity: f32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.opacity = opacity;
        self.render(cx)
    }
    pub fn get_background_color(&self) -> String {
        crate::utils::vec4_to_hex(&self.draw_popup.background_color)
    }
    pub fn set_background_color(
        &mut self,
        cx: &mut Cx,
        background_color: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.background_color
            .replace(crate::utils::hex_to_vec4(&background_color)?);
        self.render(cx)
    }
    pub fn get_hover_color(&self) -> String {
        crate::utils::vec4_to_hex(&self.draw_popup.hover_color)
    }
    pub fn set_hover_color(
        &mut self,
        cx: &mut Cx,
        hover_color: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.hover_color
            .replace(crate::utils::hex_to_vec4(&hover_color)?);
        self.render(cx)
    }
    pub fn get_focus_color(&self) -> String {
        crate::utils::vec4_to_hex(&self.draw_popup.focus_color)
    }
    pub fn set_focus_color(
        &mut self,
        cx: &mut Cx,
        focus_color: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.focus_color
            .replace(crate::utils::hex_to_vec4(&focus_color)?);
        self.render(cx)
    }
    pub fn get_border_color(&self) -> String {
        crate::utils::vec4_to_hex(&self.draw_popup.border_color)
    }
    pub fn set_border_color(
        &mut self,
        cx: &mut Cx,
        border_color: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.border_color
            .replace(crate::utils::hex_to_vec4(&border_color)?);
        self.render(cx)
    }
    pub fn get_border_width(&self) -> f32 {
        self.draw_popup.border_width
    }
    pub fn set_border_width(
        &mut self,
        cx: &mut Cx,
        border_width: f32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.border_width = border_width;
        self.render(cx)
    }
    pub fn get_border_radius(&self) -> f32 {
        self.draw_popup.border_radius
    }
    pub fn set_border_radius(
        &mut self,
        cx: &mut Cx,
        border_radius: f32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.border_radius = border_radius;
        self.render(cx)
    }
    pub fn get_visible(&self) -> bool {
        self.visible
    }
    pub fn set_visible(
        &mut self,
        cx: &mut Cx,
        visible: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.visible = visible;
        self.render(cx)
    }
    pub fn get_background_visible(&self) -> bool {
        self.draw_popup.background_visible.to_bool()
    }
    pub fn set_background_visible(
        &mut self,
        cx: &mut Cx,
        background_visible: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.background_visible = background_visible;
        self.render(cx)
    }
    pub fn get_cursor(&self) -> MouseCursor {
        self.cursor.unwrap_or_default()
    }
    pub fn set_cursor(
        &mut self,
        cx: &mut Cx,
        cursor: MouseCursor,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.cursor.replace(cursor);
        self.render(cx)
    }
    pub fn get_mode(&self) -> PopupMode {
        self.mode
    }
    pub fn set_mode(
        &mut self,
        cx: &mut Cx,
        mode: PopupMode,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.mode = mode;
        self.render(cx)
    }
    pub fn get_close_mode(&self) -> CloseMode {
        self.close_mode
    }
    pub fn set_close_mode(
        &mut self,
        cx: &mut Cx,
        close_mode: CloseMode,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.close_mode = close_mode;
        self.render(cx)
    }
    pub fn get_shadow_color(&self) -> String {
        crate::utils::vec4_to_hex(&self.draw_popup.shadow_color)
    }
    pub fn set_shadow_color(
        &mut self,
        cx: &mut Cx,
        shadow_color: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.shadow_color
            .replace(crate::utils::hex_to_vec4(&shadow_color)?);
        self.render(cx)
    }
    pub fn get_spread_radius(&self) -> f32 {
        self.draw_popup.spread_radius
    }
    pub fn set_spread_radius(
        &mut self,
        cx: &mut Cx,
        spread_radius: f32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.spread_radius = spread_radius;
        self.render(cx)
    }
    pub fn get_blur_radius(&self) -> f32 {
        self.draw_popup.blur_radius
    }
    pub fn set_blur_radius(
        &mut self,
        cx: &mut Cx,
        blur_radius: f32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.blur_radius = blur_radius;
        self.render(cx)
    }
    pub fn get_shadow_offset(&self) -> Vec2 {
        self.draw_popup.shadow_offset
    }
    pub fn set_shadow_offset(
        &mut self,
        cx: &mut Cx,
        shadow_offset: Vec2,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.shadow_offset = shadow_offset;
        self.render(cx)
    }
    pub fn get_height(&self) -> Size {
        self.walk.height
    }
    pub fn set_height(
        &mut self,
        cx: &mut Cx,
        height: Size,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.walk.height = height;
        self.render(cx)
    }
    pub fn get_width(&self) -> Size {
        self.walk.width
    }
    pub fn set_width(
        &mut self,
        cx: &mut Cx,
        width: Size,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.walk.width = width;
        self.render(cx)
    }
    pub fn get_margin(&self) -> Margin {
        self.walk.margin
    }
    pub fn set_margin(
        &mut self,
        cx: &mut Cx,
        margin: Margin,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.walk.margin = margin;
        self.render(cx)
    }
    pub fn get_abs_pos(&self) -> Option<DVec2> {
        self.walk.abs_pos
    }
    pub fn set_abs_pos(
        &mut self,
        cx: &mut Cx,
        abs_pos: Option<DVec2>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.walk.abs_pos = abs_pos;
        self.render(cx)
    }
    pub fn get_padding(&self) -> Padding {
        self.layout.padding
    }
    pub fn set_padding(
        &mut self,
        cx: &mut Cx,
        padding: Padding,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.layout.padding = padding;
        self.render(cx)
    }
    pub fn get_flow(&self) -> Flow {
        self.layout.flow
    }
    pub fn set_flow(&mut self, cx: &mut Cx, flow: Flow) -> Result<(), Box<dyn std::error::Error>> {
        self.layout.flow = flow;
        self.render(cx)
    }
    pub fn get_scroll(&self) -> DVec2 {
        self.layout.scroll
    }
    pub fn set_scroll(
        &mut self,
        cx: &mut Cx,
        scroll: DVec2,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.layout.scroll = scroll;
        self.render(cx)
    }
    pub fn get_align(&self) -> Align {
        self.layout.align
    }
    pub fn set_align(
        &mut self,
        cx: &mut Cx,
        align: Align,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.layout.align = align;
        self.render(cx)
    }
    pub fn get_spacing(&self) -> f64 {
        self.layout.spacing
    }
    pub fn set_spacing(
        &mut self,
        cx: &mut Cx,
        spacing: f64,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.layout.spacing = spacing;
        self.render(cx)
    }
}
