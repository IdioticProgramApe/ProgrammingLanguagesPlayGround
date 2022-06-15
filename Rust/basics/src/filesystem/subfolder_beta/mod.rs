pub mod file_beta;

// standard libs: https://doc.rust-lang.org/stable/std/all.html
use std::f64::consts::PI;

pub fn get_disk_surface(radius: f64) -> f64 {
    PI * radius * radius
}

