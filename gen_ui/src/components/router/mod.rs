pub mod event;
pub mod page;
mod prop;
mod schema;

use event::RouterEvent;
use makepad_widgets::*;
use page::GPageWidgetRefExt;
pub use prop::*;
// use types::{NavMode, PageType, RouterStack, RouterStackItem};

use crate::{
    components::{
        lifecycle::LifeCycle,
        router::schema::{PageType, RouterStack, RouterStackItem},
        traits::Component,
        view::GViewWidgetExt,
    },
    error::Error,
    inherits_view_widget_node, lifecycle, play_animation,
    prop::{
        traits::{HeapLiveIdPathExp, LiveIdExp},
        NavMode, RouterIndicatorMode,
    },
    set_index, set_scope_path, sync,
};

use super::{
    tabbar::GTabbarWidgetExt,
    view::{GView, GViewWidgetRefExt},
};

live_design! {
    link genui_basic;

    pub GRouterBase = {{GRouter}}{}
}

#[derive(Live, WidgetRef, WidgetSet, LiveRegisterWidget)]
pub struct GRouter {
    #[deref]
    pub deref_widget: GView,
    #[rust(id!(bar_pages)[0])]
    pub active_router: LiveId,
    #[rust]
    pub active_page: Option<HeapLiveIdPath>,
    #[rust]
    pub stack: RouterStack,
    #[rust]
    pub bar_pages: Vec<HeapLiveIdPath>,
    #[rust]
    pub nav_pages: Vec<HeapLiveIdPath>,
    #[rust]
    pub page_type: PageType,
    #[rust]
    pub mode: RouterIndicatorMode,
    #[rust]
    pub nav_actions: Option<Box<dyn FnMut(&mut GRouter, &mut Cx)>>,
    #[live]
    pub nav_mode: NavMode,
    #[rust]
    pub default_page: Option<HeapLiveIdPath>,
}

impl LiveHook for GRouter {}

inherits_view_widget_node!(GRouter);

impl Widget for GRouter {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        // self.scope_path = scope.path.clone();
        self.set_scope_path(&scope.path);
        match self.page_type {
            PageType::Bar | PageType::Nav => self
                .widget(&[self.active_router])
                .draw_walk(cx, scope, walk),

            PageType::None => DrawStep::done(),
        }
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.deref_widget.handle_event(cx, event, scope);
    }
}

impl Component for GRouter {
    type Error = Error;

    type State = RouterState;

    fn merge_conf_prop(&mut self, cx: &mut Cx) -> () {
        self.deref_widget.merge_conf_prop(cx);
    }

    fn render(&mut self, cx: &mut Cx) -> Result<(), Self::Error> {
        self.deref_widget.render(cx)
    }

    fn handle_widget_event(&mut self, _cx: &mut Cx, _event: &Event, _hit: Hit, _area: Area) {
        ()
    }

    fn switch_state(&mut self, state: Self::State) -> () {
        self.state = state.into();
    }

    fn switch_state_with_animation(&mut self, cx: &mut Cx, state: Self::State) -> () {
        self.deref_widget
            .switch_state_with_animation(cx, state.into());
    }

    fn focus_sync(&mut self) -> () {
        self.deref_widget.focus_sync();
    }

    fn set_animation(&mut self, cx: &mut Cx) -> () {
        self.deref_widget.set_animation(cx);
    }

    sync!();
    play_animation!();
    lifecycle!();
    set_index!();
    set_scope_path!();
}

impl GRouter {
    pub fn action_nav_to(&mut self, cx: &mut Cx, actions: &Actions) {
        for action in actions {
            if let Some(action) = action.as_widget_action() {
                if let RouterEvent::NavTo(path) = action.cast() {
                    self.nav_to(cx, path.as_slice());
                    break;
                }
            }
        }
    }

    pub fn indicator_nav_to(&mut self, cx: &mut Cx, actions: &Actions) -> Option<()> {
        let mut active = None;
        if let RouterIndicatorMode::Bind(bind_id) = self.mode {
            self.gtabbar(bind_id.as_slice()).borrow().map(|tabbar| {
                if let Some(e) = tabbar.changed(actions) {
                    active.replace(e.index as usize);
                }
            });
        }
        if let Some(active) = active {
            // call nav to
            let path = self.bar_pages[active].last();
            self.nav_to(cx, &[path]);
            Some(())
        } else {
            None
        }
    }
    pub fn sync_indicator(&mut self, cx: &mut Cx) -> Option<()> {
        if let RouterIndicatorMode::Bind(bind_id) = self.mode {
            let active_page = self.active_page.clone().unwrap();
            let (_, index) = self.check_route_and_find(&active_page);
            self.gtabbar(bind_id.as_slice())
                .borrow_mut()
                .map(|mut tabbar| {
                    tabbar.set_active_index(cx, index);
                    return Some(());
                });
        }
        None
    }

    /// ## handle nav back event
    pub fn handle_nav_back(&mut self, cx: &mut Cx, actions: &Actions) {
        for action in actions {
            if let RouterEvent::NavBack(_current) = action.as_widget_action().cast() {
                // get last item from stack
                self.stack.pop().map(|last| match self.nav_mode {
                    NavMode::History => self.nav_history(cx, &last.path),
                    NavMode::Switch => self.nav2(cx, &last.path),
                });
                break;
            }
        }
    }
    pub fn set_visible_page(&mut self, cx: &mut Cx, target: &HeapLiveIdPath) {
        // first check route
        self.page_type = self.check_route(target);
        self.active_router = self.page_type.live_id();
        self.gview(&[self.active_router])
            .borrow()
            .map(|active_router| {
                match self.page_type {
                    PageType::Bar => {
                        for (id, child) in active_router.children.iter() {
                            // find path in bar_pages and get path
                            if let Some(path) = self.bar_pages.iter().find(|p| p.contains_id(id)) {
                                child.as_gview().borrow_mut().map(|mut child| {
                                    if path.eq(target) {
                                        child.visible = true;
                                    } else {
                                        child.visible = false;
                                    }
                                });
                            }
                        }
                    }
                    PageType::Nav => {
                        for (id, child) in active_router.children.iter() {
                            // find path in bar_pages and get path
                            if let Some(path) = self.nav_pages.iter().find(|p| p.contains_id(id)) {
                                child.as_gpage().borrow_mut().map(|mut child| {
                                    if path.eq(target) {
                                        child.visible = true;
                                    } else {
                                        child.visible = false;
                                    }
                                });
                            }
                        }
                    }
                    PageType::None => {}
                }
            });

        // after all change active_page
        self.active_page.replace(target.clone());
        self.sync_indicator(cx);
        self.redraw(cx);
    }
    fn get_visible_page(&self) -> Option<HeapLiveIdPath> {
        // find the visible page
        if let Some(active_router) = self.gview(&[self.active_router]).borrow() {
            let mut res = None;

            for (id, child) in active_router.children.iter() {
                if child.visible() && !self.mode.eq_bind(id) {
                    let mut p = self.scope_path.as_ref().unwrap().clone();
                    p.push(*id);
                    res.replace(p);
                    break;
                }
            }
            res
        } else {
            None
        }
    }
    pub fn nav_to(&mut self, cx: &mut Cx, path: &[LiveId]) {
        let path = self.bar_scope_path(path);
        self.nav2(cx, &path);
    }
    pub fn nav_back(&mut self, cx: &mut Cx) {
        if let Some(last) = self.stack.pop() {
            match self.nav_mode {
                NavMode::History => self.nav_history(cx, &last.path),
                NavMode::Switch => self.nav2(cx, &last.path),
            }
        } else {
            let path = self.default_page.as_ref().cloned().unwrap();
            self.nav2(cx, &path);
        }
    }
    pub fn nav_to_path(cx: &mut Cx, uid: WidgetUid, scope: &mut Scope, path: &[LiveId]) {
        cx.widget_action(uid, &scope.path, RouterEvent::NavTo(path[0]));
    }
    pub fn nav_back_path(cx: &mut Cx, uid: WidgetUid, scope: &mut Scope) {
        let path = scope.path.clone();
        cx.widget_action(uid, &scope.path, RouterEvent::NavBack(path.last()));
    }
    fn nav_history(&mut self, cx: &mut Cx, path: &HeapLiveIdPath) {
        self.set_visible_page(cx, path);
        if let Some(mut actions) = self.nav_actions.take() {
            let _ = actions(self, cx);
            // set back
            self.nav_actions = Some(actions);
        }
    }
    fn nav2(&mut self, cx: &mut Cx, path: &HeapLiveIdPath) {
        self.active_page.as_ref().map(|path| {
            // push stack
            self.stack.push(RouterStackItem {
                path: path.clone(),
                ty: self.page_type,
            });
        });
        self.nav_history(cx, path);
    }
    pub fn check_route(&mut self, path: &HeapLiveIdPath) -> PageType {
        if !self.bar_pages.iter().any(|x| x.contains(path).unwrap()) {
            if self.nav_pages.iter().any(|x| x.contains(path).unwrap()) {
                PageType::Nav
            } else {
                panic!("unregister page path!: {:?}", path);
            }
        } else {
            PageType::Bar
        }
    }
    pub fn check_route_and_find(&mut self, path: &HeapLiveIdPath) -> (PageType, usize) {
        self.bar_pages.iter().position(|x| x.eq(path)).map_or_else(
            || {
                self.nav_pages.iter().position(|x| x.eq(path)).map_or_else(
                    || {
                        panic!("unregister page path!: {:?}", path);
                    },
                    |index| (PageType::Nav, index),
                )
            },
            |index| (PageType::Bar, index),
        )
    }
    pub fn bar_scope_path(&self, child: &[LiveId]) -> HeapLiveIdPath {
        let mut path = self.scope_path.as_ref().unwrap().clone();
        child.into_iter().for_each(|x| {
            path.push(*x);
        });
        path
    }
    pub fn nav_scope_path(&self, child: &[LiveId]) -> HeapLiveIdPath {
        let mut path = self.scope_path.as_ref().unwrap().clone();
        child.into_iter().for_each(|x| {
            // path.push(id!(nav_pages)[0]);
            path.push(*x);
        });
        path
    }
    /// ## Init Router
    /// This fn help you init a router by setting bar_pages and nav_pages
    /// ### Example (in `draw_walk()`)
    /// ```rust
    /// self.lifetime
    /// .init()
    /// .execute(|| {
    ///     let router = self.grouter(id!(app_router));
    ///
    ///     router.borrow_mut().map(|mut router| {
    ///         let _ = router.init(ids!(page1, page2, page3), Some(ids!(nav_page1), None)).build(cx);
    ///     });
    /// })
    /// .map(|_| {
    ///     let router = self.grouter(id!(app_router));
    ///     router.borrow().map(|router| {
    ///         if router.scope_path.is_some() {
    ///             self.lifetime.next();
    ///         }
    ///     })
    /// });
    /// ```
    pub fn init(
        &mut self,
        bar_pages: &[&[LiveId]],
        nav_pages: Option<&[&[LiveId]]>,
        mode: Option<RouterIndicatorMode>,
    ) -> &mut Self {
        if self.scope_path.is_some() {
            self.nav_pages.clear();
            self.bar_pages.clear();
            bar_pages.iter().for_each(|x| {
                let bar_path = self.bar_scope_path(x);
                self.bar_pages.push(bar_path);
            });
            nav_pages.map(|x| {
                x.iter().for_each(|x| {
                    let nav_path = self.nav_scope_path(x);
                    self.nav_pages.push(nav_path);
                });
            });
            mode.map(|mode| self.mode = mode);
            self.after_init_check();
        }
        self
    }
    /// ## Auto init Router by inner children (bar_pages, nav_pages)
    /// this fn consider bar_id is tabbar(it will never change)
    pub fn init_auto(&mut self) -> &mut Self {
        // do loop to get need children
        if self.scope_path.is_some() {
            self.nav_pages.clear();
            self.bar_pages.clear();
            let mut flag = true; // let it do only once
            self.gview(id!(bar_pages)).borrow().map(|bar| {
                for (id, child) in bar.children.iter() {
                    if !self.mode.eq_bind(id) {
                        let bar_path = self.bar_scope_path(&[id.clone()]);
                        if child.visible() && flag {
                            self.ty(PageType::Bar);
                            self.active_page.replace(bar_path.clone());
                            flag = false;
                        }
                        self.bar_pages.push(bar_path);
                    }
                }
            });
            self.gview(id!(nav_pages)).borrow().map(|nav| {
                for (id, child) in nav.children.iter() {
                    let nav_path = self.nav_scope_path(&[id.clone()]);
                    if child.visible() && flag {
                        self.ty(PageType::Nav);
                        self.active_page.replace(nav_path.clone());
                        flag = false;
                    }
                    self.nav_pages.push(nav_path);
                }
            });
        }
        self
    }
    /// ## check router page type and active page
    fn after_init_check(&mut self) -> () {
        // let it do only once
        let mut flag = true;
        // loop bar_pages and nav_pages
        for bar in self.bar_pages.clone().iter() {
            if flag {
                self.gview(id!(bar_pages)).borrow().map(|container| {
                    if container.widget(&[bar.last()]).visible() {
                        self.ty(PageType::Bar);
                        self.active_page.replace(bar.clone());
                        flag = false;
                    }
                });
            } else {
                break;
            }
        }
        for nav in self.nav_pages.clone().iter() {
            if flag {
                self.gview(id!(nav_pages)).borrow().map(|container| {
                    if container.widget(&[nav.last()]).visible() {
                        self.ty(PageType::Nav);
                        self.active_page.replace(nav.clone());
                    }
                    flag = false;
                });
            } else {
                break;
            }
        }
    }
    /// ## Set default active page
    /// set page as active page, you can use this if you need to control
    pub fn active(&mut self, id: &[LiveId]) -> &mut Self {
        // if scope is empty, do nothing
        if self.scope_path.is_some() {
            let mut path = self.scope_path.as_ref().unwrap().clone();
            path.push(id[0].clone());
            self.active_page.replace(path.clone());
            self.default_page.replace(path);
        }
        self
    }
    pub fn nav_actions<F>(&mut self, f: F) -> &mut Self
    where
        F: FnMut(&mut Self, &mut Cx) -> () + 'static,
    {
        self.nav_actions.replace(Box::new(f));
        self
    }
    /// ## Finish Router Build
    pub fn build(&mut self, cx: &mut Cx) -> () {
        if let Some(active_page) = self.active_page.clone().as_ref() {
            // do set visible page
            let _ = self.set_visible_page(cx, active_page);
        } else {
            // do get_visible_page and set as active_page
            self.get_visible_page()
                .map(|page| self.active_page.replace(page));
        }
    }
    pub fn ty(&mut self, ty: PageType) -> &mut Self {
        self.page_type = ty;
        self
    }
    pub fn handle_nav_events(&mut self, cx: &mut Cx, actions: &Actions) -> () {
        self.handle_nav_back(cx, actions);
        self.action_nav_to(cx, actions);
        self.indicator_nav_to(cx, &actions).map(|_| {
            return;
        });
        self.redraw_active(cx);
    }
    pub fn redraw_active(&mut self, cx: &mut Cx) {
        if self.scope_path.is_some() {
            match self.page_type {
                PageType::Bar => {
                    self.gview(id!(bar_pages)).redraw(cx);
                }
                PageType::Nav => {
                    self.gview(id!(nav_pages)).redraw(cx);
                }
                PageType::None => {}
            }
        }
    }
    /// ## Judget page is eq active page?
    /// - true: eq
    /// - false:
    ///     - not eq
    ///     - active_page is none(almost impossible to happen)
    pub fn eq_active_page(&self, page: &[LiveId]) -> bool {
        let path = self.bar_scope_path(page);
        if let Some(active) = self.active_page.as_ref() {
            active.eq(&path)
        } else {
            false
        }
    }
}

impl GRouterRef {
    pub fn nav_to(&self, cx: &mut Cx, path: &[LiveId]) {
        self.borrow_mut().map(|mut router| {
            router.nav_to(cx, path);
        });
    }
    pub fn nav_back(&self, cx: &mut Cx) {
        self.borrow_mut().map(|mut router| {
            router.nav_back(cx);
        });
    }
    pub fn handle_nav_events(&self, cx: &mut Cx, actions: &Actions) {
        self.borrow_mut().map(|mut router| {
            router.handle_nav_events(cx, actions);
        });
    }
}

#[macro_export]
macro_rules! nav_to {
    (
        $path: tt, $cx: expr, $uid: expr, $scope: expr
    ) => {
        gen_components::GRouter::nav_to_path($cx, $uid, $scope, id!($path));
    };
}

#[macro_export]
macro_rules! nav_back {
    (
        $cx: expr, $uid: expr, $scope: expr
    ) => {
        gen_components::GRouter::nav_back_path($cx, $uid, $scope);
    };
}
