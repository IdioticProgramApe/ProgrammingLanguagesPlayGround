use std::collections::HashMap;
use std::fmt::{Debug, Display};

// Vector, Vec<T>
// add element(s) to a vector: push, append
pub fn get_vector() -> Vec<i32> {
    let mut vector: Vec<i32> = Vec::new();
    vector.push(0);
    vector.push(2);
    vector.push(4);
    vector.push(6);
    println!("{:?}", vector);

    let mut vector2: Vec<i32> = vec![1, 3, 5, 7];
    vector.append(&mut vector2);
    println!("{:?}", vector);

    vector
}

// access one element: get (safer), [] operator to be sure
pub fn print_nth_element<T: Display + Debug>(vector: &Vec<T>, index: usize) {
    println!("the {}th element of {:?} is {}", index, vector,
             match vector.get(index) { Some(v) => v.to_string(), None => "None".to_string() })
}

// string appending, utf-8 (chinese character), access
#[derive(Debug)]
pub struct SomeString {
    s1: String,
    s2: String,
    s3: String,
    s4: String,
    s5: String,
}

impl SomeString {
    pub fn create() -> SomeString {
        SomeString {
            s1: String::from("Hello"),
            s2: String::from("World"),
            s3: 1024.to_string(),
            s4: 3.14.to_string(),
            s5: String::from("中文"),
        }
    }

    pub fn concatenate(&self) -> String {
        self.s1.clone() + " " + &self.s2 + "-" + &*format!("{},{}+{}", self.s3, self.s4, self.s5)
    }
}

// HashMap, like the dict in Python
pub fn test_hashmap() {
    let mut map = HashMap::new();

    map.insert("brand", "Sony");
    map.insert("color", "red");

    for pair in map.iter() {
        println!("key: {}, value: {}", pair.0, pair.1);
        println!("or pair: {:?}", pair);
    }

    // insert a default value if the field doesn't exist, otherwise skip
    map.entry("brand").or_insert("Microsoft");
    map.entry("price").or_insert("500$");
    println!("the hashmap now is {:?}", map);

    // modify the value (make sure the key exists)
    if let Some(v) = map.get_mut("color") {
        *v = "white";
    }
    println!("the hashmap now is {:?}", map);
}
