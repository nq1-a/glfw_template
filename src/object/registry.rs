use crate::object::Render;

pub struct Registry {
    pub objects: Vec<Box<dyn Render>>
}

impl Registry {
    pub fn new() -> Registry {
        Registry {
            objects: Vec::new()
        }
    }

    pub fn add(&mut self, renderable: Box<dyn Render>) {
        self.objects.push(renderable);
    }
}
