/// This macro generates the following functions: 
/// - `text`
/// - `set_text`
/// - `is_visible` in Widget trait
#[macro_export]
macro_rules! set_text_and_visible_fn {
    () => {
        fn text(&self) -> String {
            self.text.as_ref().to_string()
        }
        fn set_text(&mut self, cx: &mut Cx, v: &str) {
            self.text.as_mut_empty().push_str(v);
            self.redraw(cx)
        }
        fn is_visible(&self) -> bool {
            self.visible
        }
    };
}

#[macro_export]
macro_rules! default_hit_finger_down {
    ($self:ident, $cx:ident, $focus_area:expr, $e:expr) => {
        if $self.grab_key_focus {
            $cx.set_key_focus($focus_area);
        }
        $self.play_animation($cx, id!(hover.focus));
        $self.active_focus($cx, $e);
    };
}

#[macro_export]
macro_rules! default_hit_hover_in {
    ($self:ident, $cx:ident, $e:expr) => {
        let _ = set_cursor($cx, $self.cursor.as_ref());
        $self.play_animation($cx, id!(hover.on));
        $self.active_hover_in($cx, $e);
    };
}

#[macro_export]
macro_rules! default_hit_hover_out {
    ($self:ident, $cx:ident, $e:expr) => {
        $self.play_animation($cx, id!(hover.off));
        $self.active_hover_out($cx, $e);
    };
}

#[macro_export]
macro_rules! default_hit_finger_up {
    ($self:ident, $cx:ident, $e:expr) => {
        if $e.is_over {
            if $e.device.has_hovers() {
                $self.play_animation($cx, id!(hover.on));
            } else {
                $self.play_animation($cx, id!(hover.off));
            }
            $self.active_clicked($cx, $e);
        } else {
            $self.play_animation($cx, id!(hover.off));
            $self.active_focus_lost($cx, $e);
        }
    };
}

#[macro_export]
macro_rules! default_hit_finger_up_some {
    ($self:ident, $cx:ident, $e:expr) => {
        if $e.is_over {
            if $e.device.has_hovers() {
                $self.play_animation($cx, id!(hover.on));
            } else {
                $self.play_animation($cx, id!(hover.off));
            }
            $self.active_clicked($cx, Some($e));
        } else {
            $self.play_animation($cx, id!(hover.off));
            $self.active_focus_lost($cx, Some($e));
        }
    };
}

#[macro_export]
macro_rules! default_handle_animation {
    ($self:ident, $cx:ident, $event: ident) => {
        if $self.animation_key {
            if $self.animator_handle_event($cx, $event).must_redraw() {
                $self.redraw($cx);
            }
        }
    };
}
