use elaraui::{UI, Label};

fn main() {
    let mut ui = UI::new(1600, 1200);
    // as ratio, that is a 3-5-3 ratio of widths
    ui.set_layout_row([3, 5, 3]);
    // as percent, so 10 = 10%
    ui.set_padding([10, 10, 10, 10]);
    let label_left = Label::new("Left sidebar");
    let label_center = Label::new("Center area");
    let label_right = Label::new("Right sidebar");
    ui.add(label_left);
    ui.add(label_center);
    ui.add(label_right);
    println!("{:#?}", ui);
}
