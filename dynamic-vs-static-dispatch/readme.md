# Dynamic vs Static Dispatch
This is a set of benchmarks to compare the speed and memory performance of dynamic and static dispatch in Rust. Additionally, we explore the performance of techniques to mitigate the downsides of dynamic or static dispatch.
# Background
## What is dynamic and static dispatch?
Dynamic and static dispatch are two different approaches for handling parameters of different types (but with a common interface) to a function. We implement these both in Rust using traits. We then specifiy whether we want a function to use static or dynamic dispatch with the `impl` or `dyn` keywords respectively. For example, imagine that you have a Cat struct and a Dog struct. Both Cat and Dog implement the talk method via the Animal trait:
```
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
```
Running this we get:
```
With dynamic dispatch:
Tom says Meow
Barney says Woof!

With static dispatch:
Tom says Meow
Barney says Woof!
```
This is exactly the same output, so what happens underneath the hood?
## How does static and dynamic dispatch work?
The difference lies in how the functions are treated at runtime vs compile time. See, when we use static dispatch for a function, the Rust compiler makes a separate function for each variant of the trait. Using the example from above, `talk_to_animal_static` has two separate copies made of it. One for Cat and one for Dog. The compiler then replaces all function calls to `talk_to_animal_static` with the Cat and Dog version respectively. So the compiler turns our code into:
```
// Dog and Cat implementations

fn talk_to_cat(animal &Cat) {
    animal.talk();
}

fn talk_to_dog(animal &Dog) {
    animal.talk();
}

// dynamic dispatch code

fn main() {
    // Tom and Barney Code

    // Dynamic Dispatch calls

    println!("\nWith static dispatch:");
    talk_to_cat(&tom);
    talk_to_dog(&barney);
}
```
The function that is called is determined at compile time for static dispatch. Dynamic dispatch, on the otherhand, has the function call be determined at runtime using a data structure called a vtable. This is essentially a list of the trait variants and a pointer to the relevant methods.
## What is the difference?
There are then four main differences that programmers need to consider: speed, memory, compile time and code friendliness. This is summarised as:
|                  | dynamic        | static         |
|------------------|----------------|----------------|
| speed            | slower         | faster         |
| memory           | less (usually) | more (usually) |
| compile time     | less           | more           |
| code friendliness | more           | less           |
Dynamic dispatch also gives us more flexibility with inputs and outputs of functions. For example dynamic dispatch allows you to have a Vec of trait objects with all different variants:
```
dynamic_vec = Vec::<&dyn Animal>::new(); \\ will compile
```
Additionally, when return trait objects, a static dispatch function must only return a single variant whereas the dynamic dispatch allows you to return any trait variant:
```
fn buy_pet(want_dog: bool, name: String) -> Box<dyn Animal> {
    if want_dog {
        Box::new(Dog {name: name})
    } else {
        Box::new(Cat {name: name})
    }
}
```
Today we will use benchmarks to evaluate the performance differences between dynamic and static dispatch as well as some techniques for mitigating their shortcomings (i.e. Rust enums, function wrapping).
# Overview of Benchmarks
## basic
