mod event;
mod prop;

pub use event::*;
pub use prop::*;

use makepad_widgets::*;

use crate::{
    components::{lifecycle::LifeCycle, traits::{Component, Prop}, view::LView}, error::Error, prop::ApplyStateMap, shader::draw_view::DrawView
};

live_design! {
    link luna_basic;

    pub LCard = {{LCard}} {

    }
}

#[derive(Live, WidgetRef, WidgetSet, LiveRegisterWidget)]
pub struct LCard {
    #[live]
    pub prop: CardProp,
    // --- others -------------------
    #[live]
    pub visible: bool,
    // card can not be disabled, cause it is a container
    // #[live]
    // pub disabled: bool,
    #[live]
    pub grab_key_focus: bool,
    #[live(true)]
    pub event_open: bool,
    #[rust]
    pub scope_path: Option<HeapLiveIdPath>,
    #[rust]
    apply_state_map: ApplyStateMap<CardState>,
    // --- animator ----------------
    #[live(true)]
    pub animation_open: bool,
    #[animator]
    pub animator: Animator,
    // --- slots -------------------
    #[live]
    pub header: LView,
    #[live]
    pub body: LView,
    #[live]
    pub footer: LView,
    // --- init ----------------------
    #[rust]
    pub lifecycle: LifeCycle,
    #[rust]
    index: usize,
    #[live(true)]
    pub sync: bool,
    #[live]
    pub draw_card: DrawView,
}

impl WidgetNode for LCard {
    fn uid_to_widget(&self, uid: WidgetUid) -> WidgetRef {
        for slot in [&self.header, &self.body, &self.footer] {
            for (_, child) in &slot.children {
                let x = child.uid_to_widget(uid);
                if !x.is_empty() {
                    return x;
                }
            }
        }
        WidgetRef::empty()
    }

    fn find_widgets(&self, path: &[LiveId], cached: WidgetCache, results: &mut WidgetSet) {
        for slot in [&self.header, &self.body, &self.footer] {
            for (_, child) in &slot.children {
                child.find_widgets(path, cached, results);
            }
        }
    }

    fn walk(&mut self, _cx: &mut Cx) -> Walk {
       let prop = self.prop.get(self.current_state());
       Walk {
        abs_pos: prop.abs_pos,
        margin: prop.margin,
        width: prop.width,
        height: prop.height,
    }
    }

    fn area(&self) -> Area {
        todo!()
    }

    fn redraw(&mut self, _cx: &mut Cx) {
        todo!()
    }
}

impl Widget for LCard {}

impl LiveHook for LCard {}

impl Component for LCard {
    type Error = Error;

    type State = CardState;

    fn merge_conf_prop(&mut self, cx: &mut Cx) -> () {
        todo!()
    }

    fn render(&mut self, cx: &mut Cx) -> Result<(), Self::Error> {
        todo!()
    }

    fn set_scope_path(&mut self, path: &HeapLiveIdPath) -> () {
        todo!()
    }

    fn current_state(&self) -> Self::State {
        todo!()
    }

    fn handle_widget_event(&mut self, cx: &mut Cx, event: &Event, hit: Hit, area: Area) {
        todo!()
    }

    fn play_animation(&mut self, cx: &mut Cx, state: &[LiveId; 2]) -> () {
        todo!()
    }

    fn clear_animation(&mut self, cx: &mut Cx) -> () {
        todo!()
    }

    fn switch_state(&mut self, state: Self::State) -> () {
        todo!()
    }

    fn switch_state_with_animation(&mut self, cx: &mut Cx, state: Self::State) -> () {
        todo!()
    }

    fn sync(&mut self) -> () {
        todo!()
    }

    fn set_animation(&mut self, cx: &mut Cx) -> () {
        todo!()
    }

    fn lifecycle(&self) -> LifeCycle {
        todo!()
    }

    fn set_index(&mut self, index: usize) -> () {
        todo!()
    }
}