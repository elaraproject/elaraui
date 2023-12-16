use elaraui::{UI, Label, Layout};

fn main() {
    let mut ui = UI::new(1600, 1200);
    // as ratio, that is a 3-5-3 ratio of widths
    // ui.set_layout_row([3, 5, 3]);
    // as percent, so 10 = 10%
    // ui.set_padding([10]);
    let label_left = Label::new("Left sidebar");
    ui.add_element(label_left);
    
    let label_center = Label::new("Center area");
    ui.add_element(label_center);
    
    // let label_right = Label::new("Right sidebar");
    let mut right_label_container = Layout::new_rows([2, 2]);
    
    let right_label_top = Label::new("Right top");
    right_label_container.add_element(right_label_top);
    
    let right_label_bottom = Label::new("Right bottom");
    right_label_container.add_element(right_label_bottom);
    
    ui.add_layout(right_label_container);
    // println!("{:#?}", ui);
    let s = ui.to_graphviz();
    println!("{}", s);
}
