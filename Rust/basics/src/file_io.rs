/// # Careful
/// Here, we want to output a variable created in the function, \
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
