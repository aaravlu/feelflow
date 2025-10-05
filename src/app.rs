use makepad_widgets::*;

use crate::config::Config;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    App = {{App}} {
        ui: <Window> {
            caption_bar = {
                visible: true
                caption_label = {
                    label = {
                        text: "FeelFlow"
                        margin: { left: 0 }
                    }
                }
            }

            body = {
                align: { x: 0.5, y: 0.5 }
                dock = <Dock> {
                    width: Fill
                    height: Fill
                    padding: 0
                    spacing: 0

                    root = Splitter {
                        axis: Horizontal
                        align: FromA(300.0)
                        a: left_tab
                        b: right_tab
                    }

                    left_tab = Tab {
                        kind: left
                    }

                    right_tab = Tab {
                        kind: right
                    }

                    left = <View> {
                        show_bg: true
                        draw_bg: {
                            color: #CFCF9F
                        }
                    }
                    right = <View> {
                        show_bg: true
                        draw_bg: {
                            color: #9FCFCF
                        }
                    }
                }
            }
        }
    }
}

#[derive(Live, LiveHook)]
pub struct App {
    #[live]
    ui: WidgetRef,
    #[rust]
    config: Option<Config>,
}
app_main!(App);

impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        makepad_widgets::live_design(cx);
        crate::widgets::live_design(cx);
    }
}

impl MatchEvent for App {
    fn handle_startup(&mut self, _cx: &mut Cx) {
        self.load_config();
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.match_event(cx, event);
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}

impl App {
    fn load_config(&mut self) {
        let config = Config::load().unwrap();
        self.config = Some(config)
    }
}
