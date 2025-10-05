pub mod moment;
pub mod moments_adder;
pub mod moments_list;

use makepad_widgets::Cx;

pub fn live_design(cx: &mut Cx) {
    moment::live_design(cx);
    moments_adder::live_design(cx);
    moments_list::live_design(cx);
}
