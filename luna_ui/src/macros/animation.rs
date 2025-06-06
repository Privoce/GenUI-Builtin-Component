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

#[macro_export]
macro_rules! play_animation {
    () => {
        fn play_animation(&mut self, cx: &mut Cx, state: &[LiveId; 2]) -> (){
            if self.animation_open {
                self.clear_animation(cx);
                self.animator_play(cx, state);
            }
        }
    };
}
