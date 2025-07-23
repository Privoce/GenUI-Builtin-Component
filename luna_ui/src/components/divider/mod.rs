use makepad_widgets::*;

use crate::{components::{traits::Component, view::GView}, prop::Direction, pure_after_apply};

live_design! {
    link gen_base;
    use link::shaders::*;

    pub GDividerBase = {{GDivider}}{
        height: 2.0,
        width: Fill,
        align: {x: 0.5, y: 0.5},
        draw_view: {
            // direction is 1.0 for horizontal and 0.0 for vertical
            instance direction: 1.0,
            instance stroke_width: 1.4,
            fn pixel(self) -> vec4{
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                if self.direction == 1.0 {
                    sdf.box(
                        self.pos.x,
                        self.pos.y + self.rect_size.y / 2.0 - self.stroke_width / 2.0,
                        self.rect_size.x,
                        self.stroke_width,
                        max(1.0, self.border_radius)
                    );
                } else {
                    sdf.box(
                        self.pos.x + self.rect_size.x / 2.0 - self.stroke_width / 2.0,
                        self.pos.y,
                        self.stroke_width,
                        self.rect_size.y,
                        max(1.0, self.border_radius)
                    );
                }

                if self.background_visible != 0.0 {
                    sdf.fill(self.get_background_color());
                }
                return sdf.result;
            }
        }
    }
}

#[derive(Live, WidgetRef, WidgetSet, LiveRegisterWidget)]
pub struct GDivider {
    #[deref]
    pub deref_widget: GView,
    #[live(1.4)]
    pub stroke_width: f32,
    #[live(Direction::Horizontal)]
    pub direction: Direction,
}

impl WidgetNode for GDivider {
    fn uid_to_widget(&self, _uid: WidgetUid) -> WidgetRef {
        todo!()
    }

    fn find_widgets(&self, _path: &[LiveId], _cached: WidgetCache, _results: &mut WidgetSet) {
        todo!()
    }

    fn walk(&mut self, _cx: &mut Cx) -> Walk {
        todo!()
    }

    fn area(&self) -> Area {
        todo!()
    }

    fn redraw(&mut self, _cx: &mut Cx) {
        todo!()
    }
}

impl Widget for GDivider {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.deref_widget.draw_walk(cx, scope, walk)
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        // let actions = cx.capture_actions(|cx| self.deref_widget.handle_event(cx, event, scope));
        self.deref_widget.handle_event(cx, event, scope)
    }
    
}

impl LiveHook for GDivider {
    // pure_after_apply!();
}

