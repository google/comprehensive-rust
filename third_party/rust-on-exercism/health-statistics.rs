struct User {
    name: String,
    age: u32,
    weight: f32,
}

impl User {
    pub fn new(name: String, age: u32, weight: f32) -> Self {
        println!("Use name {:?} age {:?} and weight{:?}", name, age, weight);
        unimplemented!()
    }

    pub fn name(&self) -> &str {
        println!("Use name {:?}", self.name);
        unimplemented!()
    }

    pub fn age(&self) -> u32 {
        println!("Use name {:?}", self.name);
        unimplemented!()
    }

    pub fn weight(&self) -> f32 {
        println!("Use name {:?}", self.name);
        unimplemented!()
    }

    pub fn set_age(&mut self, new_age: u32) {
        println!("Use name {:?} and new age {:?}", self.name, new_age);
        unimplemented!()
    }

    pub fn set_weight(&mut self, new_weight: f32) {
        println!("Use name {:?} and new age {:?}", self.name, new_weight);
        unimplemented!()
    }
}
