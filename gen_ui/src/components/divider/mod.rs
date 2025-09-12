mod prop;

use makepad_widgets::*;
pub use prop::*;

use crate::{
    components::{
        lifecycle::LifeCycle,
        traits::{BasicStyle, Component, Style},
    }, error::Error, lifecycle, prop::{manuel::BASIC, ApplyStateMap}, pure_after_apply, set_index, set_scope_path, shader::draw_view::DrawView, sync, themes::conf::Conf, visible
};

live_design! {
    link genui_basic;
    pub GDividerBase = {{GDivider}}{}
}

#[derive(Live, WidgetRef, WidgetSet, LiveRegisterWidget)]
pub struct GDivider {
    #[live]
    pub style: DividerProp,
    // --- visible -------------------
    #[live(true)]
    pub visible: bool,
    #[rust]
    pub scope_path: Option<HeapLiveIdPath>,
    #[rust]
    pub apply_state_map: ApplyStateMap<DividerState>,
    // --- draw ----------------------
    #[live]
    pub draw_divider: DrawView,
    // --- init ----------------------
    #[rust]
    pub lifecycle: LifeCycle,
    #[rust]
    index: usize,
    #[live(true)]
    pub sync: bool,
    #[rust]
    pub state: DividerState,
}

impl Widget for GDivider {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        if !self.visible {
            return DrawStep::done();
        }
        let style = self.style.get(self.state);
        self.draw_divider.begin(cx, walk, style.layout());
        self.draw_divider.end(cx);
        self.set_scope_path(&scope.path);
        DrawStep::done()
    }
}

impl WidgetNode for GDivider {
    fn uid_to_widget(&self, _uid: WidgetUid) -> WidgetRef {
        WidgetRef::empty()
    }

    fn find_widgets(&self, _path: &[LiveId], _cached: WidgetCache, _results: &mut WidgetSet) {
        ()
    }

    fn walk(&mut self, _cx: &mut Cx) -> Walk {
        let style = self.style.get(self.state);
        style.walk()
    }

    fn area(&self) -> Area {
        self.draw_divider.area
    }

    fn redraw(&mut self, cx: &mut Cx) {
        let _ = self.render(cx);
        self.draw_divider.redraw(cx);
    }

    fn state(&self) -> String {
        self.state.to_string()
    }
    fn animation_spread(&self) -> bool {
        true
    }
    visible!();
}

impl LiveHook for GDivider {
    pure_after_apply!();
    fn after_new_before_apply(&mut self, cx: &mut Cx) {
        self.merge_conf_prop(cx);
    }

    fn after_apply(&mut self, _cx: &mut Cx, _apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        self.set_apply_state_map(
            nodes,
            index,
            &DividerBasicStyle::live_props(),
            [live_id!(basic)],
            |_| {},
            |prefix, component, applys| match prefix.to_string().as_str() {
                BASIC => {
                    component
                        .apply_state_map
                        .insert(DividerState::Basic, applys);
                }
                _ => {}
            },
        );
    }
}

impl Component for GDivider {
    type Error = Error;

    type State = DividerState;

    fn merge_conf_prop(&mut self, cx: &mut Cx) -> () {
        let style = &cx.global::<Conf>().components.divider;
        self.style = style.clone();
    }

    fn render(&mut self, _cx: &mut Cx) -> Result<(), Self::Error> {
        let style = self.style.get(self.state);
        self.draw_divider.merge(&style.into());
        Ok(())
    }


    fn handle_widget_event(&mut self, _cx: &mut Cx, _event: &Event, _hit: Hit, _area: Area) {
        ()
    }

    fn switch_state(&mut self, state: Self::State) -> () {
        self.state = state;
    }

    fn switch_state_with_animation(&mut self, _cx: &mut Cx, _state: Self::State) -> () {
        ()
    }

    fn focus_sync(&mut self) -> () {
        self.style.sync(&self.apply_state_map);
    }

    fn set_animation(&mut self, _cx: &mut Cx) -> () {
        ()
    }

    fn play_animation(&mut self, _cx: &mut Cx, _state: &[LiveId; 2]) -> () {
        ()
    }
    
    sync!();
    set_scope_path!();
    set_index!();
    lifecycle!();
}
