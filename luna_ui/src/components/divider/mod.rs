mod prop;

use makepad_widgets::*;
pub use prop::*;

use crate::{
    components::{
        lifecycle::LifeCycle,
        traits::{BasicProp, Component, Prop},
    },
    error::Error,
    lifecycle,
    prop::{manuel::BASIC, ApplyStateMap},
    pure_after_apply, set_index, set_scope_path,
    shader::draw_view::DrawView,
    themes::Conf,
};

live_design! {
    link genui_basic;
    pub GDividerBase = {{GDivider}}{}
}

#[derive(Live, WidgetRef, WidgetSet, LiveRegisterWidget)]
pub struct GDivider {
    #[live]
    pub prop: DividerProp,
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
}

impl Widget for GDivider {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, _walk: Walk) -> DrawStep {
        let prop = self.prop.get(self.current_state());
        self.draw_divider.begin(cx, prop.walk(), prop.layout());
        self.draw_divider.end(cx);
        self.set_scope_path(&scope.path);
        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        // let actions = cx.capture_actions(|cx| self.deref_widget.handle_event(cx, event, scope));
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
        let prop = self.prop.get(self.current_state());
        prop.walk()
    }

    fn area(&self) -> Area {
        self.draw_divider.area
    }

    fn redraw(&mut self, cx: &mut Cx) {
        let _ = self.render(cx);
        self.draw_divider.redraw(cx);
    }
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
            &DividerBasicProp::live_props(),
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
        let prop = &cx.global::<Conf>().components.divider;
        self.prop = prop.clone();
    }

    fn render(&mut self, _cx: &mut Cx) -> Result<(), Self::Error> {
        let prop = self.prop.get(self.current_state());
        self.draw_divider.merge(&prop.into());
        Ok(())
    }

    fn current_state(&self) -> Self::State {
        DividerState::Basic
    }

    fn handle_widget_event(&mut self, cx: &mut Cx, event: &Event, hit: Hit, area: Area) {
        todo!()
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
