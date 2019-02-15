#![feature(range_contains)]
mod scanner;
use scanner::scanner::scan;

fn main() {
    scan("asp/mini.asp");
}
