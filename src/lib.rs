use std::fmt;
use std::sync::atomic::AtomicUsize;

pub trait NodeType {
	fn node_type(&self) -> &str;
}

#[derive(Debug)]
pub enum Element {
	ChildLayout(Layout),
	ChildComponent(Box<dyn Component>)
}

#[derive(Debug)]
pub struct Layout
{
    // elements: Option<Vec<Box<dyn Component>>>,
    // child_layouts: Option<Vec<Layout>>,
    elements: Vec<Element>,
    layout_type: LayoutType,
    proportions: Vec<u32>,
    padding: Vec<u32>,
}

impl NodeType for Layout {
	fn node_type(&self) -> &str {
		"Layout"
	}
}

#[derive(Debug)]
pub enum LayoutType {
    Column,
    Row
}

// Work on to_graphviz()

impl Layout {
    pub fn empty() -> Layout {
    	// let id = VAL_CNT.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        Layout {
            elements: vec![],
            layout_type: LayoutType::Column,
            proportions: vec![1],
            padding: vec![0],
        }
    }

    pub fn new_rows<const N: usize>(proportions: [u32; N]) -> Layout {
    	// let id = VAL_CNT.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        Layout {
            elements: vec![],
            layout_type: LayoutType::Row,
            proportions: proportions.to_vec(),
            padding: vec![0],
        }
    }
    
    pub fn new_columns<const N: usize>(proportions: [u32; N]) -> Layout {
    	// let id = VAL_CNT.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        Layout {
            elements: vec![],
            layout_type: LayoutType::Column,
            proportions: proportions.to_vec(),
            padding: vec![0],
        }
    }
    
    pub fn add_element<C: Component + 'static>(&mut self, component: C) {
        let boxed_component = Element::ChildComponent(Box::new(component));
        self.elements.push(boxed_component);
    }
    
    pub fn add_layout(&mut self, layout: Layout) {
        self.elements.push(Element::ChildLayout(layout));
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
    fn calculate_locations(&mut self) {

    }
    
    pub fn render(&mut self) {
        // TODO
        self.calculate_locations();
    }
}

#[derive(Debug)]
pub struct UI
{
    width: u32,
    height: u32,
    elements: Layout
}

impl NodeType for UI {
	fn node_type(&self) -> &str {
		"UI"
	}
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
    
    pub fn add_layout_container(&mut self, layout: Layout) {
        self.elements.add_layout(layout);
    }
    
    pub fn set_padding(&mut self, padding: [u32; 4]) {
        self.elements.set_padding(padding);
    }

    pub fn to_graphviz(&self) {
    	// self.elements.
    }
}


pub struct Button {
    label: String,
    location: Option<(f32, f32)>,
    dimensions: Option<(f32, f32)>, // (width, height)
}

impl NodeType for Button {
	fn node_type(&self) -> &str {
		"Button"
	}
}

impl Button {
    pub fn new(label: &str) -> Button {
        Button {
            label: label.to_string(),
            location: None,
            dimensions: None,
        }
    }
}

impl Component for Button {
    fn name(&self) -> &'static str {
        "Button"
    }
    
    fn set_computed_location(&mut self, location: (f32, f32)) {
        self.location = Some(location)
    }
    
    fn get_computed_location(&self) -> Option<&(f32, f32)> {
        self.location.as_ref()
    }
}

pub struct Label {
    name: String,
    location: Option<(f32, f32)>,
    dimensions: Option<(f32, f32)>,
}

impl Label {
    pub fn new(name: &str) -> Label {
        Label {
            name: name.to_string(),
            location: None,
            dimensions: None,
        }
    }
}

impl NodeType for Label {
	fn node_type(&self) -> &str {
		"Label"
	}
}

impl Component for Label {
    fn name(&self) -> &'static str {
        "Label"
    }
    
    fn set_computed_location(&mut self, location: (f32, f32)) {
        self.location = Some(location)
    }
    
    fn get_computed_location(&self) -> Option<&(f32, f32)> {
        self.location.as_ref()
    }
}

pub trait Component {
    fn name(&self) -> &'static str;
    fn set_computed_location(&mut self, location: (f32, f32));
    fn get_computed_location(&self) -> Option<&(f32, f32)>;
    // will be implemented with elara-gfx drawing backend
    // fn draw(&self, tr: TextRenderer, lr: LineRenderer, rr: RectRenderer); 
}

impl fmt::Debug for dyn Component {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Component(type: {}, location: {:?})", self.name(), self.get_computed_location())
    }
}
