use std::fmt;

// Notes:
// Assign an ID to every element upon creation,
// use ID to select an element in the tree
// e.g. UI.get("main-btn")
macro_rules! uvec2 {
    [$x:ty, $y:ty] => {
        UVec2::new(x, y)
    }
}

pub struct UVec2 {
    pub x: u32,
    pub y: u32
}

impl UVec2 {
    pub fn new(x: u32, y: u32) -> UVec2 {
        UVec2 { x, y }
    }
}

impl fmt::Debug for UVec2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "UVec2({}, {})", self.x, self.y)
    }
}

#[derive(Debug)]
pub struct Layout
{
    elements: Option<Vec<Box<dyn Component>>>,
    child_layouts: Option<Vec<Layout>>,
    layout_type: LayoutType,
    proportions: Vec<u32>,
    padding: Vec<u32>
}

#[derive(Debug)]
pub enum LayoutType {
    Column,
    Row
}

// Work on to_graphviz()

impl Layout {
    pub fn empty() -> Layout {
        Layout {
            elements: None,
            child_layouts: None,
            layout_type: LayoutType::Column,
            proportions: vec![1],
            padding: vec![0]
        }
    }

    pub fn new_rows<const N: usize>(proportions: [u32; N]) -> Layout {
        Layout {
            elements: Some(vec![]),
            child_layouts: None,
            layout_type: LayoutType::Row,
            proportions: proportions.to_vec(),
            padding: vec![0]
        }
    }
    
    pub fn new_columns<const N: usize>(proportions: [u32; N]) -> Layout {
        Layout {
            elements: Some(vec![]),
            child_layouts: None,
            layout_type: LayoutType::Column,
            proportions: proportions.to_vec(),
            padding: vec![0]
        }
    }
    
    pub fn add_element<C: Component + 'static>(&mut self, component: C) {
        // If child nodes are None, then make a 
        // vec to push the component into,
        // otherwise, do nothing
        match self.elements.as_mut() {
            None => self.elements = Some(vec![]),
            Some(_) => ()
        }
        self.elements.as_mut().unwrap().push(Box::new(component));
    }
    
    pub fn add_layout(&mut self, layout: Layout) {
        // If child layouts are None, then make a 
        // vec to push the component into,
        // otherwise, do nothing
        match self.child_layouts.as_mut() {
            None => self.child_layouts = Some(vec![]),
            Some(_) => ()
        }
        self.child_layouts.as_mut().unwrap().push(layout);
    }
    
    pub fn set_layout<const N: usize>(&mut self, proportions: [u32; N]) {
        self.proportions = proportions.to_vec();
    }
    
    pub fn set_padding(&mut self, padding: [u32; 4]) {
        self.padding = padding.to_vec();
    }
    
    pub fn set_layout_type(&mut self, layout_type: LayoutType) {
        self.layout_type = layout_type;
    }
    
    // Calculate the (x, y) locations
    // of each component based on the
    // layout tree and propagate the
    // location to each component
    fn calculate_locations(&mut self) {}
    
    pub fn render(&mut self) {}
}

#[derive(Debug)]
pub struct UI
{
    width: u32,
    height: u32,
    elements: Layout
}

impl UI
{
    pub fn new(width: u32, height: u32) -> UI {
        UI {
            width,
            height,
            elements: Layout::empty()
        }
    }
    
    pub fn set_layout_column<const N: usize>(&mut self, proportions: [u32; N]) {
        self.elements.set_layout(proportions);
        self.elements.set_layout_type(LayoutType::Column);
    }
    
    pub fn set_layout_row<const N: usize>(&mut self, proportions: [u32; N]) {
        self.elements.set_layout(proportions);
        self.elements.set_layout_type(LayoutType::Row);
    }
    
    pub fn add<C: Component + 'static>(&mut self, element: C) {
        self.elements.add_element(element)
    }
    
    pub fn add_layout(&mut self, layout: Layout) {
        self.elements.add_layout(layout);
    }
    
    pub fn set_padding(&mut self, padding: [u32; 4]) {
        self.elements.set_padding(padding);
    }
}


pub struct Button {
    label: String,
    location: Option<UVec2>
}

impl Button {
    pub fn new(label: &str) -> Button {
        Button {
            label: label.to_string(),
            location: None
        }
    }
}

impl Component for Button {
    fn name(&self) -> &'static str {
        "Button"
    }
    
    fn set_computed_location(&mut self, location: UVec2) {
        self.location = Some(location)
    }
    
    fn get_computed_location(&self) -> Option<&UVec2> {
        self.location.as_ref()
    }
}

pub struct Label {
    name: String,
    location: Option<UVec2>
}

impl Label {
    pub fn new(name: &str) -> Label {
        Label {
            name: name.to_string(),
            location: None
        }
    }
}

impl Component for Label {
    fn name(&self) -> &'static str {
        "Label"
    }
    
    fn set_computed_location(&mut self, location: UVec2) {
        self.location = Some(location)
    }
    
    fn get_computed_location(&self) -> Option<&UVec2> {
        self.location.as_ref()
    }
}

pub trait Component {
    fn name(&self) -> &'static str;
    fn set_computed_location(&mut self, location: UVec2);
    fn get_computed_location(&self) -> Option<&UVec2>;
    // fn on_draw(&self); will be implemented with drawing backend
}

impl fmt::Debug for dyn Component {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Component(type: {}, location: {:?})", self.name(), self.get_computed_location())
    }
}