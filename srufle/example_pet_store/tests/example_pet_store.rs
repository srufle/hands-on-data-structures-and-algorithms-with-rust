use example_pet_store::*;
#[cfg(test)]
use pretty_assertions::assert_eq;

#[test]
fn test_speak() {
    let dog = Dog {
        name: "Spot".to_string(),
    };
    assert_eq!(dog.speak(), "Spot says Woof!");
}

#[test]
fn create_pet_store_inline() {
    let cat = Cat {
        name: String::from("Kitty"),
    };
    let dog = Dog {
        name: String::from("Spot"),
    };
    let pet_store = PetStore {
        animals: vec![Box::new(cat), Box::new(dog)],
    };

    pet_store.chatter();
    assert_eq!(pet_store.animals[0].speak(), "Kitty says Meow!");
    assert_eq!(pet_store.animals[1].speak(), "Spot says Woof!");
}

#[test]
fn create_pet_store_passing() {
    let cat = Cat {
        name: String::from("Kitty"),
    };
    let dog = Dog {
        name: String::from("Spot"),
    };
    let animals: Vec<Box<dyn Animal>> = vec![Box::new(cat), Box::new(dog)];
    let mut pet_store = PetStore::new(animals);

    pet_store.chatter();
    assert_eq!(pet_store.animals[0].speak(), "Kitty says Meow!");
    assert_eq!(pet_store.animals[1].speak(), "Spot says Woof!");

    let dog = Dog {
        name: String::from("LuLu"),
    };

    pet_store.add(Box::new(dog));
    assert_eq!(pet_store.animals[2].speak(), "LuLu says Woof!");
}
