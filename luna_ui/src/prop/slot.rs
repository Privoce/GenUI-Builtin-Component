use makepad_widgets::{Cx2d, DeferWalk, LiveId, Scope};

use crate::{components::LComponent, prop::DeferWalks};

pub struct SlotDrawer<'s> {
    pub slots: Vec<(LiveId, LComponent<'s>)>,
    pub defer_walks: &'s mut DeferWalks,
}

impl<'s> SlotDrawer<'s> {
    pub fn new<S>(slots: S, defer_walks: &'s mut DeferWalks) -> Self
    where
        S: IntoIterator<Item = (LiveId, LComponent<'s>)>,
    {
        // before new do clear defer walks
        defer_walks.clear();

        Self {
            slots: slots.into_iter().collect(),
            defer_walks,
        }
    }
    pub fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope) -> () {
        let slot_draw_walk =
            |slot: &mut LComponent, cx: &mut Cx2d, scope: &mut Scope, df_walk: &mut DeferWalk| {
                let res_walk = df_walk.resolve(cx);
                let _ = slot.draw_walk(cx, scope, res_walk);
            };

        for (id, component) in &mut self.slots {
            if component.visible() {
                let walk = component.walk(cx);
                if let Some(fw) = cx.defer_walk(walk) {
                    // if is fill, defer the walk
                    self.defer_walks.push((*id, fw));
                } else {
                    let _ = component.draw_walk(cx, scope, walk);
                }
            }
        }

        for (id, df_walk) in self.defer_walks.iter_mut() {
            for (slot_id, slot) in &mut self.slots {
                if *id == *slot_id {
                    slot_draw_walk(slot, cx, scope, df_walk);
                    break;
                }
            }
        }
    }
}
