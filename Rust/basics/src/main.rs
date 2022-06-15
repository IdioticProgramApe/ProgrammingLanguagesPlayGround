// the macros to suppress the warnings, at crate level don't forget the !
#![allow(unused_variables)]
#![allow(unreachable_code)]
#![allow(dead_code)]

mod structs;
mod enums;
mod filesystem;
mod error_handle;

use crate::filesystem::subfolder_alpha::file_alpha::self_introduction;
use crate::filesystem::subfolder_beta::file_beta::self_introduction as beta_introduction;
use crate::filesystem::subfolder_beta::get_disk_surface;

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
        // error_handle::test_panic();                                              // Crash!
        error_handle::read_file(String::from("external/hello.txt"));                // OK!
        error_handle::read_file(String::from("external/not_exist_file.txt"));       // OK!

        error_handle::read_file_s(String::from("external/hello.txt"));              // OK!
        // error_handle::read_file_s(String::from("external/not_exist_file.txt"));  // Crash!

        error_handle::test_catch_result(10);

        error_handle::test_get_file_content("external/hello.txt");
        error_handle::test_get_file_content("external/not_exist_file.txt");
    }
}
