// any future modules
// in lib go here

use std::fmt;
use std::sync::atomic::AtomicUsize;
static VAL_CNT: AtomicUsize = AtomicUsize::new(0);

pub trait NodeType {
	fn node_type(&self) -> &str;
}

pub trait HasID {
	fn id(&self) -> usize;
}

#[derive(Debug)]
pub enum Element {
	ChildLayout(Layout),
	ChildComponent(Box<dyn Component>)
}

impl NodeType for Element {
	fn node_type(&self) -> &str {
		match self {
			Self::ChildLayout(layout) => layout.node_type(),
			Self::ChildComponent(boxed_component) => &boxed_component.node_type()
		}
	}
}

impl HasID for Element {
	fn id(&self) -> usize {
		match self {
			Self::ChildLayout(layout) => layout.id(),
			Self::ChildComponent(boxed_component) => boxed_component.id()
		}
	}
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
    id: usize
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

impl Layout {
    pub fn empty() -> Layout {
    	let id = VAL_CNT.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        Layout {
            elements: vec![],
            layout_type: LayoutType::Column,
            proportions: vec![1],
            padding: vec![0],
            id
        }
    }

    pub fn new_rows<const N: usize>(proportions: [u32; N]) -> Layout {
    	let id = VAL_CNT.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        Layout {
            elements: vec![],
            layout_type: LayoutType::Row,
            proportions: proportions.to_vec(),
            padding: vec![0],
            id
        }
    }
    
    pub fn new_columns<const N: usize>(proportions: [u32; N]) -> Layout {
    	let id = VAL_CNT.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        Layout {
            elements: vec![],
            layout_type: LayoutType::Column,
            proportions: proportions.to_vec(),
            padding: vec![0],
            id
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

    pub fn id(&self) -> usize {
    	self.id
    }
}

#[derive(Debug)]
pub struct UI
{
    width: u32,
    height: u32,
    container: Element
}

impl UI
{
    pub fn new(width: u32, height: u32) -> UI {
        UI {
            width,
            height,
            container: Element::ChildLayout(Layout::empty())
        }
    }
    
    pub fn set_layout_column<const N: usize>(&mut self, proportions: [u32; N]) {
    	if let Element::ChildLayout(ref mut container) = &mut self.container {
    	    container.set_layout(proportions);
    	    container.set_layout_type(LayoutType::Column);
    	}
    }

     pub fn set_layout_row<const N: usize>(&mut self, proportions: [u32; N]) {
    	if let Element::ChildLayout(ref mut container) = &mut self.container {
    	    container.set_layout(proportions);
    	    container.set_layout_type(LayoutType::Row);
    	}
    }

    pub fn set_padding(&mut self, padding: [u32; 4]) {
    	if let Element::ChildLayout(ref mut container) = &mut self.container {
    	    container.set_padding(padding);
    	}
    }
 
    pub fn add_element<C: Component + 'static>(&mut self, element: C) {
    	if let Element::ChildLayout(ref mut container) = &mut self.container {
    		container.add_element(element);
    	}
    }

    pub fn add_layout(&mut self, layout: Layout) {
    	if let Element::ChildLayout(ref mut container) = &mut self.container {
    		container.add_layout(layout);
    	}
    }

    pub fn to_graphviz(&self) -> String {
    	fn inner(element: &Element) -> String {
    		let node_type = if element.id() == 0 { "UI" } else { element.node_type() };
    		let mut s = format!("q{} [label=\"{}\"];\n", element.id(), node_type);
    		if let Element::ChildLayout(layout) = element {
	    		for el in layout.elements.iter() {
		    			s.push_str(&inner(el));
		    			s.push_str(&format!("q{} -- q{};\n", element.id(), el.id()));
		    		}	
	    		} 		
    		s
    	}
    	let mut s_graph = String::from("graph {\n");
    	s_graph.push_str(&inner(&self.container));
    	s_graph.push_str("}\n");
    	s_graph
    }

    // Calculate the (x, y) locations and widths
    // of each component based on the
    // layout tree and propagate the
    // location to each component
    fn calculate_sizes_positions(&mut self) {
    	// something like self.iter().calculate_frame() for each element
    }
}


pub struct Button {
    label: String,
    location: Option<(f32, f32)>, // anchor point
    dimensions: Option<(f32, f32)>, // (width, height)
    id: usize
}

impl Button {
    pub fn new(label: &str) -> Button {
    	let id = VAL_CNT.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        Button {
            label: label.to_string(),
            location: None,
            dimensions: None,
            id
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

    fn node_type(&self) -> &str {
    	"Button"
    }

    fn id(&self) -> usize {
       	self.id
    }
}

pub struct Label {
    name: String,
    location: Option<(f32, f32)>,
    dimensions: Option<(f32, f32)>,
    id: usize
}

impl Label {
    pub fn new(name: &str) -> Label {
    	let id = VAL_CNT.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        Label {
            name: name.to_string(),
            location: None,
            dimensions: None,
            id
        }
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

    fn node_type(&self) -> &str {
    	"Label"
    }

    fn id(&self) -> usize {
    	self.id
    }
}

pub trait Component {
    fn name(&self) -> &'static str;
    fn set_computed_location(&mut self, location: (f32, f32));
    fn get_computed_location(&self) -> Option<&(f32, f32)>;
    fn node_type(&self) -> &str;
    fn id(&self) -> usize;
    // will be implemented with elara-gfx drawing backend
    // fn draw(&self, tr: TextRenderer, lr: LineRenderer, rr: RectRenderer); 
}

impl fmt::Debug for dyn Component {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Component(type: {}, location: {:?})", self.name(), self.get_computed_location())
    }
}
