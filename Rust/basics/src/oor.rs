// oor represents: Object Oriented Rust
// Rust is not a OOP language, but we can have some OOP mechanisms

use std::fmt::Debug;

// encapsulation, in rust use struct or enum to encapsule data
#[derive(Debug)]
pub struct SomeClass<T: Debug> {
    internal_data: T,
}

impl<T: Debug> SomeClass<T> {
    pub fn new(data: T) ->SomeClass<T> {
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
