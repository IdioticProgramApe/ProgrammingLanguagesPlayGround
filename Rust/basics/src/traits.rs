use std::fmt::Debug;
use super::structs::Person;

// can work as virtual function in c++, can be pure virtual if no implementation provided
pub trait Description {
    fn describe(&self) -> String {
        String::from("[Default Description]... ")
    }
}

// this function only accepts the instances that implemented Description trait
pub fn introduction(instance: &impl Description) {
    println!("{}", instance.describe());
}

pub fn generic_introduction<T: Description>(instance: &T) {
    introduction(instance);
}

pub fn generic_introduction_v0(instance: &(impl Description + Debug)) {
    println!("{:?}", instance.describe());
}

pub fn generic_introduction_v1<T: Description + Debug> (instance: &T) {
    println!("{:?}", instance.describe());
}

pub fn generic_introduction_v2<T>(instance: &T) -> bool where T: Description + Debug {
    println!("{:?}", instance.describe());
    true
}

// implement a compare function, to make 2 object perform < > ==
// careful with this &Self, mean current type, not the instance (&self)
pub trait Compare {
    fn less_than(object1: &Self, object2: &Self) -> bool;
}

pub fn max<T: Compare + Clone>(array: &[T]) -> Result<T, bool> {
    return if array.len() == 0 {
        Err(false)
    } else {
        let mut max_value = array[0].clone();
        for value in &array[1..] {
            if Compare::less_than(&max_value, value) {
                max_value = (*value).clone();
            }
        }
        Ok(max_value)
    }
}

impl Compare for f64 {
    fn less_than(object1: &f64, object2: &f64) -> bool {
        object1 < object2
    }
}

// use trait as return value: the object which implemented the traits
pub fn person() -> impl Description {
    Person::create(String::from("E"), 0, String::from("American"), String::from("New Born"))
}
