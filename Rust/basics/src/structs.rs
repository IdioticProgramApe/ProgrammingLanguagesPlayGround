// This is a macro needed to pretty print a struct
#[derive(Debug)]

// struct declaration
struct Person {
    name: String,
    age: i8,
    nationality: String,
    profession: String
}

impl Person {
    // struct methods, has &self passed in
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn older(&self, other: &Person) -> bool {
        self.age > other.age
    }

    // struct associated function, no self passed in
    fn create(name: String, age: i8, nationality: String, profession: String) -> Person {
        Person {name, age, nationality, profession}
    }
}

// tuple struct, no key-pair only a tuple
struct Color(u8, u8, u8);
struct Point(f64, f64);

fn test_struct() {
    {
        let person_a = Person::create(
            String::from("A"),
            28,
            String::from("Chinese"),
            String::from("Programmer")
        );

        let person_b = Person {
            name: String::from("B"),
            age: 30,
            nationality: String::from("French"),
            profession: String::from("Researcher")
        };

        // pretty print struct, use {:?} to print into one line
        println!("The information about A: {:#?}", person_a);
        println!("A is {} than B", if person_a.older(&person_b) {"older"} else {"younger"});
        println!("person b's name is {}", person_b.get_name())
    }

    {
        let red = Color(255, 0, 0);
        let origin = Point(0.0, 0.0);

        println!("the red value of red color is {}, the y value of origin is {:}", red.0, origin.1);
    }
}
