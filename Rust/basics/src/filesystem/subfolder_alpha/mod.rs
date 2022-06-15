pub mod file_alpha;

// a private method can be called at this level and in its children
fn self_introduction() -> String {
    String::from("subfolder_alpha!")
}
