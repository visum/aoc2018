#[macro_use]
extern crate lazy_static;
extern crate regex;
use regex::Regex;
use std::fs::File;
use std::io::BufReader;
use std::collections::HashMap;

enum Event {
    Wake,
    Sleep,
}

struct Shift {
    events: HashMap<u32, Event>,
}

struct Guard {
    shifts: Vec<Shift>
}

fn main() {
    
}


#[test]
fn test() {
// answer is 240
}