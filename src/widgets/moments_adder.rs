use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    pub MomentsAdder = {{MomentsAdder}} {

    }
}

#[derive(Clone, Debug, DefaultNone)]
pub enum MomentsAdderAction {
    Add(String),
    None,
}

#[derive(Live, LiveHook, Widget)]
pub struct MomentsAdder {
    #[deref]
    input: TextInput,
}

impl Widget for MomentsAdder {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        // we post a `MomentAdderAction::Add` when user types `Enter` key.

        self.input.handle_event(cx, event, scope);
    }
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.input.draw_walk(cx, scope, walk)
    }
}
