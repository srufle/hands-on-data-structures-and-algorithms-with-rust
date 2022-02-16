pub struct PetStore {
    pub animals: Vec<Box<dyn Animal>>,
}

impl PetStore {
    pub fn new(animals: Vec<Box<dyn Animal>>) -> Self {
        PetStore { animals }
    }

    pub fn chatter(&self) {
        for animal in self.animals.iter() {
            animal.speak();
        }
    }

    pub fn add(&mut self, value: Box<dyn Animal>) {
        self.animals.push(value);
    }
}

pub trait Animal {
    fn speak(&self) -> String;
}

pub struct Dog {
    pub name: String,
}

pub struct Cat {
    pub name: String,
}

impl Animal for Dog {
    fn speak(&self) -> String {
        format!("{} says Woof!", &self.name)
    }
}

impl Animal for Cat {
    fn speak(&self) -> String {
        format!("{} says Meow!", &self.name)
    }
}
