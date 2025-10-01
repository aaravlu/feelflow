use makepad_widgets::*;

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
                <Label> {
                    text: "Welcome to FeelFlow!"
                    draw_text: {
                        color: #ffffff
                        text_style: {
                            line_spacing: 1.5,
                            font_size: 20.,
                        }
                    }
                }
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
    selected_note: Option<usize>,
}
app_main!(App);

impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        makepad_widgets::live_design(cx);
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}
