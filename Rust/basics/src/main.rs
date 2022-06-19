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
mod file_io;
mod collections;
mod oor;
mod concurrency;

use std::fmt::Display;
use std::vec;
use crate::filesystem::subfolder_alpha::file_alpha::self_introduction;
use crate::filesystem::subfolder_beta::file_beta::self_introduction as beta_introduction;
use crate::filesystem::subfolder_beta::get_disk_surface;
use crate::traits::Description;         // this use is necessary

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

        let texel1 = generics::Texel { s: 0, t: 1.0 };
        let texel2 = generics::Texel { s: 1.0, t: 0 };
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

        // some lambda struct and
        // use default trait function implementation
        struct RandomStruct {}
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

    // go to lifetime.rs to get more details
    {
        lifetime::test_good_longer_v1();
        lifetime::test_good_longer_v2();
        lifetime::test_person_with_lifetime();

        // the reference itself is not static, will be dropped when leaving the scope
        let static_str: &'static str = "this is a static string";

        // a snippet with generic, trait and lifetime
        fn some_bizarre_stuff<'a, T>(s1: &'a str, s2: &'a str, p: T) -> bool
            where T: Display {
            println!("{}", p);
            if s1.len() > s2.len() { true } else { false }
        }
    }

    // go to file_io.rs to get more details
    {
        // try to type `cargo run hello world` in the terminal to see the output
        file_io::print_args();

        // program stops here, waiting for a input, use `cargo run` to test
        println!("your input line is: \n{}", file_io::input());

        // the following line will panic, uncomment to see it!
        // let something_not_exist = file_io::read_file("blabla_something.txt");

        // read a python script from this repo
        let python_script_path = "../../Python/IO/Asyncio/main.py";
        println!("this is a python script: \n{}", file_io::read_file(python_script_path));
        println!("these are some first binaries of python script: \n{:?}", 
                 file_io::read_file_as_binary("external/hello.txt", 10));
        file_io::test_read_file_to_buffer(python_script_path);

        // write something to a file
        file_io::direct_writein("external/temporary.log", "I don't know to to write.\n");
        file_io::append_string("external/temporary.log", b"again, I really don't know\n").unwrap();
        file_io::overwrite_with_string("external/temporary.log", b"some random words").unwrap();
        file_io::write_buffer("external/temporary_frombuffer.log", b"I don't know to to write.\n")
    }

    // go to collections to get more details
    {
        let mut vector = collections::get_vector();
        collections::print_nth_element(&vector, 5);
        collections::print_nth_element(&vector, vector.len());

        // add some value to all the elements
        if !vector.is_empty() {
            let mut index = 0;
            for element in &mut vector {
                *element += index;
                index += 1;
            }
        }
        println!("the vector now is {:?}", vector);

        let mut s = collections::SomeString::create().concatenate();
        s.push_str(" blabla");
        s.push('!');

        // len() and chars().count() is different because of the chinese character
        // each utf-8 character will have 3 bytes.
        println!("\"{}\"'s length is {}, has {} characters", s, s.len(), s.chars().count());
        println!("the 23rd character is {}", s.chars().nth(23).unwrap());
        println!("the first 20 bytes are \"{}\"", &s[..20]);

        // hashmap
        collections::test_hashmap();
    }

    // go to oor.rs to get more details
    {
        let vector = vec![10, 20, 30, 40, 50];
        let some_instance = oor::SomeClass::new(vector);
        some_instance.public_method();

        oor::test_inheritance();
    }

    // go to concurrency.rs to get more details
    {
        concurrency::test_closure();

        concurrency::test_thread_case_1();
        concurrency::test_thread_case_2();
        concurrency::test_thread_case_3();
        concurrency::test_thread_case_4();
    }
}
