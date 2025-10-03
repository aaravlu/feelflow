use anyhow::anyhow;
use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    MomentsList = {{MomentsList}} {
        flow: Down
        spacing: 0.
        keep_invisible: false

        moment = <View> {
            tag = <Label> {
                // It is a holder
                text = ""
            }

            content = <Label> {
                // It is a holder
                text = ""
            }
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

impl MomentsList {
    pub fn add_moments_list(&mut self, cx: &mut Cx, moments: Vec<Moment>) {
        self.moments.extend(moments);
        cx.action(MomentsListAction::Update);
    }

    pub fn add_single_moment(&mut self, cx: &mut Cx, moment: Moment) {
        self.moments.push(moment);
        cx.action(MomentsListAction::Update);
    }
}

#[derive(Debug)]
pub enum MomentsListAction {
    Update,
}

#[derive(Clone, Debug)]
pub struct Moment {
    tag: String,
    content: String,
}

impl Moment {
    pub fn new(tag: String, content: String) -> Self {
        Moment { tag, content }
    }
    pub fn from_csv(data: &str) -> anyhow::Result<Vec<Self>> {
        let mut reader = csv::Reader::from_reader(data.as_bytes());
        let mut moments = Vec::new();

        for result in reader.records() {
            let record = result?;
            let tag = record
                .get(0)
                .ok_or(anyhow!("Can not parse tag"))?
                .to_string();
            let content = record
                .get(1)
                .ok_or(anyhow!("Can not parse content"))?
                .to_string();
            moments.push(Moment::new(tag, content));
        }

        Ok(moments)
    }
    pub fn tag(&self) -> &str {
        &self.tag
    }
    pub fn content(&self) -> &str {
        &self.content
    }
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
                let Moment { tag, content } = moment;
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
