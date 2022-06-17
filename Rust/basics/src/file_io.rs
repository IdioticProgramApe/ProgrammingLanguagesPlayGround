use std::fs::OpenOptions;
use std::io;
use std::{io::{stdin, Read, Write}, fs};

/// # Careful
/// Here, we want to output a variable created in the functi&on, \
/// so the output type shouldn't be the reference because it's local \
/// we need to move the ownership to outside, therefore no & \
/// the following code won't compile
///
/// ```
/// fn get_args() -> Vec<&str> {
///     let mut arg_strings = Vec::new();
///     for arg in std::env::args() {
///         arg_strings.push(arg.as_str());
///     }
///     arg_strings
/// }
/// ```

// get commandlet string
fn get_args() -> Vec<String> {
    let mut arg_strings = Vec::new();
    for arg in std::env::args() {
        arg_strings.push(arg);
    }
    arg_strings
}

pub fn print_args() {
    for arg in get_args() {
        println!("{}", arg)
    }
}

// get the input from terminal during the program execution
// read_line returns a Result object, usually use expect or unwrap to deal with Err
// see an example of unwrap in error_handle.rs
pub fn input() -> String {
    let mut input_buf = String::new();
    stdin().read_line(&mut input_buf).expect("Read line failed.");
    input_buf
}

// file reading
pub fn read_file(filepath: &str) -> String {
    fs::read_to_string(filepath).expect(&format!("Cannot read from file \"{}\"", filepath))
}

pub fn read_file_as_binary(filepath: &str, limit: usize) -> Vec<u8> {
    let original_vec = fs::read(filepath).unwrap();
    if original_vec.len() > limit { Vec::from(&original_vec[..limit]) } else { original_vec }
}

pub fn test_read_file_to_buffer(filepath: &str) {
    let mut buffer = [0u8; 10];
    let mut file = fs::File::open(filepath).unwrap();

    let mut index = 0;
    while index < 3 {
        file.read(&mut buffer).unwrap();
        println!("{:?}", buffer);
        index += 1;
    }
}

// file writing
pub fn direct_writein(filepath: &str, content: &str) {
    // this will clean the old content or create the file
    fs::write(filepath, content).unwrap();
}

pub fn write_buffer(filepath: &str, content: &[u8]) {
    let mut file = fs::File::create(filepath).unwrap();
    file.write(content).unwrap();
}

// OpenOption to deal with append, open, write
pub fn append_string(filepath: &str, content: &[u8]) -> io::Result<()> {
    let mut file = OpenOptions::new().append(true).open(filepath)?;
    file.write(content)?;
    Ok(())
}

// overwrite the content without clean them first
pub fn overwrite_with_string(filepath: &str, content: &[u8]) -> io::Result<()> {
    let mut file = OpenOptions::new().read(true).write(true).open(filepath)?;
    file.write(content)?;
    Ok(())
}
