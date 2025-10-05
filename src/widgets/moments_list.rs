use makepad_widgets::*;

use crate::widgets::{moment::Moment, moments_adder::MomentsAdderAction};

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::widgets::moment::MomentUI;

    pub MomentsList = {{MomentsList}} {
        flow: Down
        spacing: 0.
        keep_invisible: false

        moment = <MomentUI> {

        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct MomentsList {
    #[deref]
    list: PortalList,
    #[rust]
    moments: Vec<Moment>,
}

impl Widget for MomentsList {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.list.handle_event(cx, event, scope);
    }
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let range = self.moments.len();
        let mut moments_iter = self.moments.iter();
        self.list.set_item_range(cx, 0, range);

        while let Some(item_id) = self.list.next_visible_item(cx) {
            let item = if let Some(moment) = moments_iter.next() {
                let (content, tag) = (moment.content(), moment.tag());
                let moment = self.list.item(cx, item_id, live_id!(moment)).as_view();
                moment.label(id!(tag)).set_text(cx, tag);
                moment.label(id!(content)).set_text(cx, content);
                moment
            } else {
                continue;
            };
            item.draw_all(cx, scope)
        }
        DrawStep::done()
        // self.list.draw_walk(cx, scope, walk)
    }
}

impl MatchEvent for MomentsList {
    fn handle_action(&mut self, _cx: &mut Cx, action: &Action) {
        if let Some(MomentsAdderAction::Add(content)) = action.downcast_ref() {
            self.add_single_moment(Moment::from_content(content.clone()));
        }
    }
}

impl MomentsList {
    pub fn add_moments_list(&mut self, moments: Vec<Moment>) {
        self.moments.extend(moments);
    }

    pub fn add_single_moment(&mut self, moment: Moment) {
        self.moments.push(moment);
    }
}
