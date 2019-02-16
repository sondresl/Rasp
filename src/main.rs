#![feature(range_contains)]
mod scanner;
use scanner::scanner::scan;

fn main() {
    scan("asp/mini.asp")
        .expect("Failed to scan mini.asp");
}
