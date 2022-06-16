// the enum class is an ensemble of structs
enum Transport {
    Plane (f64, String),
    Vehicle {speed: f64, field: String},
    Boat {speed: f64, field: String}
}

pub fn test_enums() {
    let plane = Transport::Plane(1000.0, String::from("air"));
    let vehicle = Transport::Vehicle{speed: 200.0, field: String::from("road")};
    let boat = Transport::Boat{speed: 300.0, field: String::from("ocean")};

    match plane {
        Transport::Vehicle {speed, field} => {
            println!("the speed of vehicle is {}", speed);
        },
        Transport::Plane(i, j) => {
            println!("the speed of plane is {}", i);
        },
        Transport::Boat {speed, field} => {
            println!("the speed of boat is {}", speed);
        }
    }

    // branching on float is dangerous
    let branch = "abc";
    match branch {
        "abc" => println!("OK"),
        _ => {}
    }

    // define a value can be null (null is forbidden in most cases in Rust)
    // use Option to construct, Option:: can be omit
    let opt1 = Option::Some("Something");
    match opt1 {
        Option::Some(content) => {
            println!("the content of option is {}", content);
        },
        Option::None => {
            println!("no content in option");
        }
    }

    // if let ... else ... can replace match on 2 cases
    let opt2: Option<i8> = Option::None;
    if let None = opt2 {
        println!("no content in option");
    } else {
        println!("something in option");
    }
}