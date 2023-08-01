use elaraui::{UI, Label};

fn main() {
    let mut ui = UI::new(1600, 1200);
    ui.set_layout_column([3, 5, 3]);
    ui.set_padding([20, 20, 20, 20]);
    let label_left = Label::new("Left sidebar");
    let label_center = Label::new("Center area");
    let label_right = Label::new("Right sidebar");
    ui.add(label_left);
    ui.add(label_center);
    ui.add(label_right);
    println!("{:#?}", ui);
}