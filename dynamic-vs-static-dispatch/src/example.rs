pub mod basic;

trait Animal {
    fn talk(&self);
}

struct Cat {
    name: String
}

impl Animal for Cat {
    fn talk(&self) {
        println!("{} says Meow", self.name);
    }
}

struct Dog {
    name: String
}

impl Animal for Dog {
    fn talk(&self) {
        println!("{} says Woof!", self.name);
    }
}

fn talk_to_animal_static(animal: &impl Animal) {
    animal.talk();
}

fn talk_to_animal_dynamic(animal: &dyn Animal) {
    animal.talk();
}

fn buy_pet(want_dog: bool, name: String) -> Box<dyn Animal> {
    if want_dog {
        Box::new(Dog {name: name})
    } else {
        Box::new(Cat {name: name})
    }
}

fn main() {
    let tom = Cat{name: "Tom".to_owned()};
    let barney = Dog{name: "Barney".to_owned()};


    println!("With dynamic dispatch:");
    talk_to_animal_dynamic(&tom);
    talk_to_animal_dynamic(&barney);

    println!("\nWith static dispatch:");
    talk_to_animal_static(&tom);
    talk_to_animal_static(&barney);
}
