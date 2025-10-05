use anyhow::anyhow;

use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    pub MomentUI = {{MomentUI}} {
        debug: true
        flow: Right

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

#[derive(Live, LiveHook, Widget)]
pub struct MomentUI {
    #[deref]
    view: View,

    #[rust]
    moment: Moment,
}

impl Widget for MomentUI {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl MomentUI {
    fn set_moment(&mut self, moment: Moment) {
        self.moment = moment;
    }
}

#[derive(Clone, Debug, Default)]
pub struct Moment {
    content: String,
    tag: String,
}

impl Moment {
    pub fn new(tag: String, content: String) -> Self {
        Self { tag, content }
    }

    pub fn from_content(content: String) -> Self {
        Self {
            tag: String::new(),
            content,
        }
    }

    pub fn set_tag(&mut self, tag: String) {
        self.tag = tag
    }

    pub fn from_csv(data: &str) -> anyhow::Result<Vec<Self>> {
        let mut reader = csv::Reader::from_reader(data.as_bytes());
        let mut moments = Vec::new();

        for result in reader.records() {
            let record = result?;
            let tag = record
                .get(0)
                .ok_or_else(|| {
                    log!("Can not parse content");
                    anyhow!("Can not parse content")
                })?
                .to_string();
            let content = record
                .get(1)
                .ok_or_else(|| {
                    log!("Can not parse content");
                    anyhow!("Can not parse content")
                })?
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

#[cfg(test)]
mod tests {
    #[test]
    fn test_csv() {
        use crate::widgets::moment::Moment;
        let data = std::fs::read_to_string("test.csv").unwrap();
        let moment = Moment::from_csv(&data).unwrap();
        assert!(!moment.is_empty())
    }
}
