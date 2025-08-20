use makepad_widgets::*;
mod prop;

pub use prop::*;

use crate::{components::{label::GLabel, svg::GSvg, traits::{BasicProp, Prop}, view::GView}, prop::MenuItemMode, shader::draw_view::DrawView};

#[derive(Live, WidgetRef, WidgetSet, LiveRegisterWidget)]
pub struct GSubMenu {
    #[live]
    pub prop: SubMenuProp,
    #[live(true)]
    pub visible: bool,
    #[live]
    pub draw_sub_menu: DrawView,
    #[live]
    pub icon: GSvg,
    #[live]
    pub text: GLabel,
    #[live]
    pub extra: GSvg,
    #[live]
    pub items: GView,
    #[rust]
    pub item_modes: Vec<MenuItemMode>,
    #[rust]
    pub active: Option<String>,
    #[rust]
    pub state: SubMenuState,
}

impl WidgetNode for GSubMenu {
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

impl Widget for GSubMenu {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        if !self.visible {
            return DrawStep::done();
        }
        let prop = self.prop.get(self.state);
        self.draw_sub_menu.begin(cx, walk, prop.layout());
        if self.text.visible() {
            let title_walk = self.title.walk(cx);
            let _ = self.title.draw_walk(cx, scope, title_walk);
        }
        if self.items.visible() {
            let items_walk = self.items.walk(cx);
            let _ = self.items.draw_walk(cx, scope, items_walk);
        }

        self.draw_sub_menu.end(cx);
        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if !self.visible {
            return;
        }

        if self.title.visible() {
            let _ = self.title.handle_event(cx, event, scope);
        }
        if self.items.visible() {
            let actions = cx.capture_actions(|cx| self.items.handle_event(cx, event, scope));

            let mut fresh = None;
            for (index, ((id, child), item_mode)) in self
                .items
                .children
                .iter()
                .zip(self.item_modes.iter())
                .enumerate()
            {
                match item_mode {
                    MenuItemMode::SubMenu(_) => {
                        child.as_gsub_menu().borrow_mut().map(|mut sub_menu| {
                            sub_menu.handle_event(cx, event, scope);
                        });
                    }
                    MenuItemMode::MenuItem(_) => {
                        child.as_gmenu_item().borrow().map(|item| {
                            if let Some(e) = item.clicked(&actions) {
                                if e.selected {
                                    // means need to change self.selected
                                    self.selected.replace(vec![index]);
                                    // do fresh selected
                                    fresh.replace((id.clone(), e.e));
                                }
                            }
                        });
                    }
                }
            }
            if let Some((id, e)) = fresh {
                self.fresh_selected(cx);

                cx.widget_action(
                    self.widget_uid(),
                    &scope.path,
                    GSubMenuEvent::Changed(GSubMenuChangedParam {
                        selected: self.selected.clone(),
                        selected_id: id,
                        e,
                    }),
                );
            }
        }
    }
}

