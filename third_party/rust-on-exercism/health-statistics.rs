pub struct User {
    name: String,
    age: u32,
    height: f32,
}

impl User {
    pub fn new(name: String, age: u32, height: f32) -> Self {
        unimplemented!()
    }

    pub fn name(&self) -> &str {
        unimplemented!()
    }

    pub fn age(&self) -> u32 {
        unimplemented!()
    }

    pub fn height(&self) -> f32 {
        unimplemented!()
    }

    pub fn set_age(&mut self, new_age: u32) {
        unimplemented!()
    }

    pub fn set_height(&mut self, new_height: f32) {
        unimplemented!()
    }
}
