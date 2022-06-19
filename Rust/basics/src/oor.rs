// oor represents: Object Oriented Rust
// Rust is not a OOP language, but we can have some OOP mechanisms

use std::fmt::Debug;

// encapsulation, in rust use struct or enum to encapsule data
#[derive(Debug)]
pub struct SomeClass<T: Debug> {
    internal_data: T,
}

impl<T: Debug> SomeClass<T> where T: Debug {
    pub fn new(data: T) -> SomeClass<T> {
        SomeClass { internal_data: data }
    }

    pub fn public_method(&self) {
        println!("This is a public method from SomeClass");
        self.private_method();
    }

    fn private_method(&self) {
        println!("This is a private method from SomeClass");
        println!("The data in SomeClass is {:?}", self.internal_data);
        println!("This is the instance\n{:#?}", self);
    }
}

// inheritance, done with trait
trait Speak { fn speak (&self); }
trait Animal {
    fn animal_type (&self) -> &str;
    fn noise (&self) -> &str;
}

impl<T> Speak for T where T: Animal {
    fn speak (&self) { println!("The {} said {}", self.animal_type(), self.noise()); }
}

struct Dog {}
struct Cat {}

impl Animal for Dog {
    fn animal_type (&self) -> &str { "dog" }
    fn noise (&self) -> &str { "woof" }
}

impl Animal for Cat {
    fn animal_type (&self) -> &str { "cat" }
    fn noise (&self) -> &str { "meow" }
}

trait Human {
    fn name (&self) -> &str;
    fn sentence (&self) -> &str;
}

struct Person {}

impl Human for Person {
    fn name (&self) -> &str { "person G" }
    fn sentence (&self) -> &str { "hello world" }
}

impl Speak for Person {
    fn speak (&self) { println!("The {} said {}", self.name(), self.sentence()); }
}

pub fn test_inheritance() {
    let dog = Dog {};
    let cat = Cat {};
    dog.speak();
    cat.speak();

    let person_g = Person {};
    person_g.speak();
}
