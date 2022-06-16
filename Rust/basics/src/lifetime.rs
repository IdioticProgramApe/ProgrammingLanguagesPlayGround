// &i32 : normal reference
// &'a i32: lifetime marked reference
// &'a mut i32: mutable lifetime marked reference

/// # the following code won't compile because compiler don't know if the reference is valid or not
///
/// ```
/// fn longer_bad(s1: &str, s2: &str) -> &str {
///     if s2.len() > s1.len() { s2 } else { s1 }
/// }
///
/// pub fn test_bad_longer() {
///     let r;
///     {
///         let s1 = "some string";
///         let s2 = "some other string";
///         r = longer_bad(s1, s2);
///         println!("the longer string is {}", r);
///     }
/// }
/// ```

// lifetime marked longer function, means the inputs and output have the same lifetime
fn longer_good<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s2.len() > s1.len() { s2 } else { s1 }
}

pub fn test_good_longer_v1() {
    let r;
    {
        // literals are static
        let s1 = "some string";
        let s2 = "some other string";
        r = longer_good(s1, s2);
        println!("the longer string is \"{}\"", r);
    }
    // r is still valid here!
    println!("the longer string still is \"{}\"", r);
}

pub fn test_good_longer_v2() {
    let r;
    {
        // String object is not static
        let s1 = String::from("some string");
        let s2 = String::from("some other string");
        r = longer_good(&s1, &s2);
        println!("the longer string is \"{}\"", r);
    }
    // the following line will crash, because s1 s2 have been dropped
    // println!("the longer string still is \"{}\"", r);
}

struct PersonWithLifeTime<'a> {
    name: &'a str,
    age: i32,
    nationality: &'a str,
    profession: &'a str
}

impl<'a> PersonWithLifeTime<'a> {
    fn get_name(&self) -> &str {
        self.name
    }

    fn get_nationality(&self) -> &'a str {
        self.nationality
    }
}

pub fn test_person_with_lifetime() {
    let person_f = PersonWithLifeTime {
        name: "F",
        age: 45,
        nationality: "Canadian",
        profession: "Teacher"
    };
    println!("person_f's name and nationality are {} and {}",
             person_f.get_name(), person_f.get_nationality());
}
