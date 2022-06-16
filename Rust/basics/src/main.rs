// the macros to suppress the warnings, at crate level don't forget the !
#![allow(unused_variables)]
#![allow(unreachable_code)]
#![allow(dead_code)]

mod structs;
mod enums;
mod filesystem;
mod error_handle;
mod generics;
mod traits;
mod lifetime;

use crate::filesystem::subfolder_alpha::file_alpha::self_introduction;
use crate::filesystem::subfolder_beta::file_beta::self_introduction as beta_introduction;
use crate::filesystem::subfolder_beta::get_disk_surface;
use crate::traits::Description;

fn main() {
    // go to structs.rs to get more details
    {
        structs::test_struct();
        let mut person_c = structs::Person::create(
            String::from("C"),
            10,
            String::from("German"),
            String::from("Student")
        );
        println!("person_c's name is {}", person_c.name);
        person_c.name = String::from("CC");
        println!("person_c's name has been changed to {}", person_c.name);
    }

    // go to enums.rs to get more details
    {
        enums::test_enums();
    }

    // go to filesystem to see how organize the rust project modules
    {
        self_introduction();
        beta_introduction();
        println!("the surface of a unit disk is {}", get_disk_surface(1.0));
    }

    // go to error_handle.rs to get more details
    {
        // error_handle::test_panic();                                          // Crash!
        error_handle::read_file("external/hello.txt");                  // OK!
        error_handle::read_file("external/not_exist_file.txt");         // OK!

        error_handle::read_file_s("external/hello.txt");                // OK!
        // error_handle::read_file_s("external/not_exist_file.txt");            // Crash!

        error_handle::test_catch_result(10);

        error_handle::test_get_file_content("external/hello.txt");
        error_handle::test_get_file_content("external/not_exist_file.txt");
    }

    // go to generics.rs to get more details
    {
        println!("the max i32 is {}", generics::test_max_i32(&[0, 1, 2, 3, 4]));
        println!("the max i32 is {}", generics::test_max_i32(&[]));

        let point = generics::Point::create(3.0, 4.0);
        let origin = generics::Point::create(0.0, 0.0);
        println!("the point (3, 4)'s x value is {}, and its y value is {}",
                 point.get_x(), point.get_y());
        println!("the distance is {}", origin.get_distance(&point));

        let texel1 = generics::Texel{ s: 0, t: 1.0 };
        let texel2 = generics::Texel{ s: 1.0, t: 0 };
        let mixed_texel = texel1.mix(texel2);
        println!("the mixed texel is ({}, {})", mixed_texel.s, mixed_texel.t);
    }

    // go to traits.rs to get more details
    {
        let person_d = structs::Person::create(
            String::from("D"),
            70,
            String::from("Japanese"),
            String::from("Retired")
        );
        println!("{}", person_d.describe());
        traits::introduction(&person_d);        // doing the same as the prev line
        traits::generic_introduction(&person_d);

        // some lambda struct
        struct RandomStruct {}
        
        // use default trait function implementation
        impl Description for RandomStruct {} 

        let random_struct = RandomStruct {};
        println!("{}", random_struct.describe());

        let max_result = traits::max(&[0.0, 1.0, 2.0, 3.0, 4.0]);
        match max_result {
            Ok(v) => println!("the max value is {}", v),
            _ => println!("we don't care!")
        }

        println!("{}", traits::person().describe());
    }
}
