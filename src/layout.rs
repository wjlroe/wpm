use cgmath::*;

pub enum LayoutType {
    Vertical,
    Horizontal,
}

pub struct Element {
    bounds: Vector2<f32>,
    position: Vector2<f32>,
}

impl Element {
    fn new(bounds: Vector2<f32>) -> Self {
        Self {
            bounds,
            position: vec2(0.0, 0.0),
        }
    }
}

pub struct Layout {
    layout_type: LayoutType,
    pub elements: Vec<Element>,
    bounding_box: Vector2<f32>,
}

impl Layout {
    pub fn center_horizontally(
        bounding_box: Vector2<f32>,
        element_bounds: Vector2<f32>,
        element_position: &mut Vector2<f32>,
    ) {
        let mut horizontal = Self::horizontal(bounding_box);
        horizontal.add_bounds(element_bounds);
        horizontal.calc_positions();
        element_position.x = horizontal.element_position(0).x;
    }

    pub fn vertical(bounding_box: Vector2<f32>) -> Self {
        Self {
            layout_type: LayoutType::Vertical,
            elements: vec![],
            bounding_box,
        }
    }

    pub fn horizontal(bounding_box: Vector2<f32>) -> Self {
        Self {
            layout_type: LayoutType::Horizontal,
            elements: vec![],
            bounding_box,
        }
    }

    pub fn add_element(&mut self, element: Element) -> usize {
        self.elements.push(element);
        self.elements.len() - 1
    }

    pub fn add_bounds(&mut self, bounds: Vector2<f32>) -> usize {
        self.add_element(Element::new(bounds));
        self.elements.len() - 1
    }

    pub fn calc_positions(&mut self) {
        match self.layout_type {
            LayoutType::Vertical => self.calc_vertical(),
            LayoutType::Horizontal => self.calc_horizontal(),
        }
    }

    pub fn calc_vertical(&mut self) {
        let mut total_height = 0.0;
        for element in &self.elements {
            total_height += element.bounds.y;
        }
        let mut top = (self.bounding_box.y - total_height) / 2.0;
        for mut element in self.elements.iter_mut() {
            element.position.y = top;
            top += element.bounds.y;
        }
    }

    fn calc_horizontal(&mut self) {
        let mut total_width = 0.0;
        for element in &self.elements {
            total_width += element.bounds.x;
        }
        let mut left = (self.bounding_box.x - total_width) / 2.0;
        for mut element in self.elements.iter_mut() {
            element.position.x = left;
            left += element.bounds.x;
        }
    }

    pub fn element_position(&self, element_idx: usize) -> Vector2<f32> {
        self.elements[element_idx].position
    }
}
