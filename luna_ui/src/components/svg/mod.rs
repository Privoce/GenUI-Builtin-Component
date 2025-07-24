mod event;
mod prop;

pub use event::*;
pub use prop::*;

use crate::{
    components::{
        lifecycle::LifeCycle,
        traits::{BasicProp, Component, Prop},
    }, error::Error, lifecycle, makepad_derive_widget::*, makepad_draw::*, prop::ApplyStateMap, pure_after_apply, set_index, set_scope_path, themes::Conf, widget::*
};

live_design! {
    link widgets
    pub GSvgBase = {{GSvg}} {}

    pub GSvg = <GSvgBase> {
        width: Fit,
        height: Fit,

        icon_walk: {
            width: 17.5,
            height: Fit,
        }

        draw_bg: {
            uniform color: #0000,
            fn pixel(self) -> vec4 {
                return self.color;
            }
        }

        draw_svg: {
            uniform color: #f00
            fn get_color(self) -> vec4 {
                return self.color
            }
        }
    }

    pub GSvgGradientX = <GSvg> {
        draw_svg: {
            uniform color_1: (#f00)
            uniform color_2: (#00f)
            fn get_color(self) -> vec4 {
                return mix(self.color_1, self.color_2, self.pos.x);
            }
        }
    }

    pub GSvgGradientY = <GSvgGradientX> {
        draw_svg: {
            color_1: (#f00)
            color_2: (#00f)
            fn get_color(self) -> vec4 {
                return mix(self.color_1, self.color_2, self.pos.y);
            }
        }
    }

}

#[derive(Live, WidgetRef, WidgetSet, LiveRegisterWidget)]
pub struct GSvg {
    #[live]
    pub prop: SvgProp,
    // --- visible -------------------
    #[live(true)]
    pub visible: bool,
    #[rust]
    pub scope_path: Option<HeapLiveIdPath>,
    #[rust]
    pub apply_state_map: ApplyStateMap<SvgState>,
    // --- draw ----------------------
    #[live]
    draw_svg: DrawIcon,
    // --- init ----------------------
    #[rust]
    lifecycle: LifeCycle,
    #[rust]
    index: usize,
    #[live(true)]
    pub sync: bool,
}

impl Widget for GSvg {
    fn handle_event(&mut self, _cx: &mut Cx, _event: &Event, _scope: &mut Scope) {}

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, _walk: Walk) -> DrawStep {
        let prop = self.prop.get(self.current_state());
        self.draw_svg.draw_walk(cx, prop.walk());
        DrawStep::done()
    }
}

impl WidgetNode for GSvg {
    fn uid_to_widget(&self, _uid: WidgetUid) -> WidgetRef {
        WidgetRef::empty()
    }

    fn find_widgets(&self, _path: &[LiveId], _cached: WidgetCache, _results: &mut WidgetSet) {
        ()
    }

    fn walk(&mut self, _cx: &mut Cx) -> Walk {
        let prop = self.prop.get(self.current_state());
        prop.walk()
    }

    fn area(&self) -> Area {
        self.draw_svg.area
    }

    fn redraw(&mut self, cx: &mut Cx) {
        let _ = self.render(cx);
        self.draw_svg.redraw(cx);
    }
}

impl LiveHook for GSvg {
    pure_after_apply!();
}

impl Component for GSvg {
    type Error = Error;

    type State = SvgState;

    fn merge_conf_prop(&mut self, cx: &mut Cx) -> () {
        let prop = &cx.global::<Conf>().components.svg;
        self.prop = prop.clone();
    }

    fn render(&mut self, _cx: &mut Cx) -> Result<(), Self::Error> {
        let prop = self.prop.get(self.current_state());
        // self.draw_svg.merge(&prop.into());

        Ok(())
    }

    fn current_state(&self) -> Self::State {
        SvgState::Basic
    }

    fn handle_widget_event(&mut self, _cx: &mut Cx, _event: &Event, _hit: Hit, _area: Area) {
        ()
    }

    fn clear_animation(&mut self, _cx: &mut Cx) -> () {
        ()
    }

    fn switch_state(&mut self, _state: Self::State) -> () {
        ()
    }

    fn switch_state_with_animation(&mut self, _cx: &mut Cx, _state: Self::State) -> () {
        ()
    }

    fn sync(&mut self) -> () {
        if !self.sync {
            return;
        }
        // sync state if is not Basic
        self.prop.sync(&self.apply_state_map);
    }

    fn set_animation(&mut self, _cx: &mut Cx) -> () {
        ()
    }

    fn play_animation(&mut self, _cx: &mut Cx, _state: &[LiveId; 2]) -> () {
        ()
    }
    set_scope_path!();
    set_index!();
    lifecycle!();
}
