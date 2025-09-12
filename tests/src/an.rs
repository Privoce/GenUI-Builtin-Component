use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use link::gen_ui::*;

    TestPList = {{TestPList}} {
        list = <PortalList> {
            scroll_bar: <ScrollBar> {}
            TopSpace = <View> {height: 0.}
            BottomSpace = <View> {height: 100.}

            Post = <CachedView>{
                flow: Down,
                <GButton>{
                    style: {
                        basic: {
                            border_width: 2.0,
                            border_color: #ff0,
                        }
                        hover: {
                            border_width: 4.0,
                        }
                    }
                }
                <Hr> {}
            }
        }
    }

}

#[derive(Live, LiveHook, Widget)]
struct TestPList {
    #[deref]
    view: View,
}

impl Widget for TestPList {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        while let Some(item) = self.view.draw_walk(cx, scope, walk).step() {
            if let Some(mut list) = item.as_portal_list().borrow_mut() {
                list.set_item_range(cx, 0, 1000);
                while let Some(item_id) = list.next_visible_item(cx) {
                    let template = match item_id {
                        0 => live_id!(TopSpace),
                        _ => live_id!(Post),
                    };
                    let item = list.item(cx, item_id, template);
                    let text = match item_id % 4 {
                        1 => format!("At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet. Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum."),
                        2 => format!("How are you?"),
                        3 => format!("Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet."),
                        _ => format!("Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua."),
                    };
                    item.label(id!(content.text)).set_text(cx, &text);
                    item.button(id!(likes))
                        .set_text(cx, &format!("{}", item_id % 23));
                    item.button(id!(comments))
                        .set_text(cx, &format!("{}", item_id % 6));
                    item.draw_all(cx, &mut Scope::empty());
                }
            }
        }
        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope)
    }
}
