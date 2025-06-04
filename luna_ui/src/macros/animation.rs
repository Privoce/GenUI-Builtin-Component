#[macro_export]
macro_rules! animation_open_then_redraw {
    ($self:ident, $cx:ident, $event: ident) => {
        if $self.animation_open {
            if $self.animator_handle_event($cx, $event).must_redraw() {
                $self.redraw($cx);
            }
        }
    };
}
