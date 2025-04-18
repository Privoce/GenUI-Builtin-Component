use std::{cell::RefCell, rc::Rc};

use makepad_widgets::{
    font_atlas::{CxFontsAtlasRc, FontLoader},
    Cx, Cx2d, DVec2, Font, LiveDependency, MouseCursor, Rect,
};

use super::ToPath;

pub fn get_font_family(font_family: &LiveDependency, cx: &mut Cx2d, font: &mut Font) -> () {
    let font_family = font_family.clone();

    if font_family.as_str() != font.path.as_str() {
        // let atlas = cx.get_global::<CxFontsAtlasRc>().clone();
        // let font_id = Some(
        //     atlas
        //         .0
        //         .borrow_mut()
        //         .get_or_load_font(cx, font_family.as_str()),
        // );
        {
            let font_loader = cx.get_global::<Rc<RefCell<FontLoader>>>().clone();
            let mut fonts_atlas = cx.get_global::<CxFontsAtlasRc>().0.borrow_mut();
            fonts_atlas.reset_fonts_atlas(&mut *font_loader.borrow_mut());
        }
        let font_loader = cx.get_global::<Rc<RefCell<FontLoader>>>().clone();
        let id = font_loader.borrow_mut().get_or_load(cx, font_family.as_str());
        *font = Font {
            font_id: Some(id),
            path: font_family,
        };
    }
}

pub fn set_cursor(cx: &mut Cx, cursor: Option<&MouseCursor>) -> () {
    if let Some(cursor) = cursor {
        cx.set_cursor(*cursor);
    } else {
        cx.set_cursor(MouseCursor::default());
    }
}

pub trait RectExp {
    /// judget another area is in this area, which usually used in event handle
    fn is_in(&self, rect: &Rect) -> bool;
    /// judget a point is in this area, which usually used in event handle
    fn is_in_pos(&self, pos: &DVec2) -> bool;
}

impl RectExp for Rect {
    fn is_in(&self, rect: &Rect) -> bool {
        // get size and pos to judge
        let self_size = self.size;
        let self_pos = self.pos;
        let rect_size = rect.size;
        let rect_pos = rect.pos;
        if rect_pos.x >= self_pos.x
            && rect_pos.y >= self_pos.y
            && rect_pos.x + rect_size.x <= self_pos.x + self_size.x
            && rect_pos.y + rect_size.y <= self_pos.y + self_size.y
        {
            return true;
        }
        false
    }
    fn is_in_pos(&self, pos: &DVec2) -> bool {
        let self_pos = self.pos;
        let self_size = self.size;
        if pos.x >= self_pos.x
            && pos.y >= self_pos.y
            && pos.x <= self_pos.x + self_size.x
            && pos.y <= self_pos.y + self_size.y
        {
            return true;
        }
        false
    }
}
