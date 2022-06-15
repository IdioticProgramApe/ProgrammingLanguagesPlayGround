/**
 * macro: panic!("some message") for unrecoverable error
 * enum: Result<T, E> for recoverable errors
 * definition:
 *     enum Result<T, E> {
 *         OK(T),
 *         Err(E),
 *     }
 */

use std::io;
use std::io::Read;
use std::fs::File;

pub fn test_panic() {
    panic!("fatal error");
    println!("unreachable line");
}

pub fn read_file(filename: &str) -> bool {
    let f = File::open(filename);
    return match f {
        Ok(file) => {
            println!("File opened successfully!");
            true
        },
        Err(err) => {
            println!("Failed to open the file");
            false
        }
    };
}

pub fn read_file_s(filename: &str) {
    let f1 = File::open(filename).unwrap();
    let f2 = File::open(filename).expect("Failed to open");
}

fn generate_result(i: i32, reference: i32) -> Result<i32, bool> {
    if i >= reference { Ok(i) }
    else { Err(false) }
}

fn catch_result_v1(i: i32) -> Result<i32, bool> {
    let r = generate_result(i, 0);
    return match r {
        Ok(i) => Ok(i),
        Err(b) => Err(b)
    };
}

fn catch_result_v2(i: i32) -> Result<i32, bool> {
    // the error handled by ?
    // ? operator tries to take out non-error result value,
    // but will return the error if encountered any error
    let r = generate_result(i, 100)?;
    Ok(r)
}

pub fn test_catch_result(i: i32) {
    {
        let r1 = catch_result_v1(i);
        if let Ok(v) = r1 {
            println!("Ok, catch_result v1 get value {}", v);
        } else {
            println!("Err on catch_result v1");
        }
    }

    {
        let r2 = catch_result_v2(i);
        if let Ok(v) = r2 {
            println!("Ok, catch_result v2 get value {}", v);
        } else {
            println!("Err on catch_result v2");
        }
    }
}

fn get_content_from_file(filename: &str) -> Result<String, io::Error> {
    // don't forget ?, otherwise it'll return a Result not a File
    let mut file_handle = File::open(filename)?;
    let mut file_content = String::new();
    file_handle.read_to_string(&mut file_content)?;
    Ok(file_content)
}

pub fn test_get_file_content(filename: &str) {
    let read_result = get_content_from_file(filename);
    match read_result {
        Ok(content) => println!("{}", content),
        Err(e) => {
            match e.kind() {
                io::ErrorKind::NotFound => {
                    println!("The file is not found!");
                }
                _ => {
                    println!("Unknown error!");
                }
            }
        }
    }
}
