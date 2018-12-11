#[macro_use]
extern crate lazy_static;
extern crate regex;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

struct Shape {
    top: u32,
    right: u32,
    bottom: u32,
    left: u32,
    width: u32,
    height: u32,
    id: u32
}

#[derive(Debug)]
enum Hits {
    Once,
    Twice,
}

fn read_input(file_name: &str) -> BufReader<File> {
    let file = File::open(file_name).expect("file not found");
    BufReader::new(file)
}

fn parse_line(row: &String, shape: &mut Shape) {
    lazy_static! {
        //   1      2   3    4  5
        // #1220 @ 258,186: 10x29
        static ref RE: Regex = Regex::new(r"^#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
    }

    let captures = RE.captures(row).unwrap();
    shape.id = captures.get(1).unwrap().as_str().parse().unwrap();
    shape.left = captures.get(2).unwrap().as_str().parse().unwrap();
    shape.top = captures.get(3).unwrap().as_str().parse().unwrap();
    shape.width = captures.get(4).unwrap().as_str().parse().unwrap();
    shape.height = captures.get(5).unwrap().as_str().parse().unwrap();
    shape.right = shape.left + shape.width;
    shape.bottom = shape.top + shape.height;
}

fn calculate_coverage(covered: &mut HashMap<u32, HashMap<u32, Hits>>, lines: &mut BufReader<File>) {
    for line in lines.lines() {
        let mut shape = Shape {
            top: 0,
            right: 0,
            left: 0,
            bottom: 0,
            width: 0,
            height: 0,
            id:0
        };
        parse_line(&line.unwrap(), &mut shape);

        for row in shape.top..shape.bottom {
            let row_hash = covered.entry(row).or_insert(HashMap::new());

            for column in shape.left..shape.right {
                match row_hash.get_mut(&column) {
                    None => {
                        row_hash.insert(column, Hits::Once);
                    }
                    Some(y) => match y {
                        Hits::Once => {
                            *y = Hits::Twice;
                        }
                        _ => (),
                    },
                };
            }
        }
    }
}

fn count_covered_twice(covered: &mut HashMap<u32, HashMap<u32, Hits>>) -> u32 {
    let mut count: u32 = 0;
    for row in covered.values() {
        for column in row.values() {
            match column {
                Hits::Twice => count += 1,
                _ => (),
            }
        }
    }

    count
}

fn shape_is_unique(shape :&Shape, covered: &mut HashMap<u32, HashMap<u32, Hits>>) -> bool {
    
    for column in shape.top..shape.bottom {
        for row in shape.left..shape.right {
            let coverage = covered.get(&row).expect("invalid row").get(&column).expect("invalid column");
            let is_clean = match coverage {
                Hits::Once => true,
                Hits::Twice => false
            };
            if !is_clean {
                return false;
            };
        }
    }

    true
}

fn find_unique_shapes(lines: &mut BufReader<File>, mut coverage: &mut HashMap<u32, HashMap<u32, Hits>>) {
    for line in lines.lines() {
        let mut shape = Shape {
            top: 0,
            right: 0,
            left: 0,
            bottom: 0,
            width: 0,
            height: 0,
            id:0
        };
        parse_line(&line.unwrap(), &mut shape);
        if shape_is_unique(&mut shape, &mut coverage) {
            println!("{} is unique", shape.id);
        } else {
            println!("{} is not unique", shape.id);
        };
    }
}

fn main() {
    let mut line_buffer = read_input("/Users/bdhowe/git/aoc2018/day3/pt1/input.txt");
    let mut covered: HashMap<u32, HashMap<u32, Hits>> = HashMap::new();
    calculate_coverage(&mut covered, &mut line_buffer);
    let twice_covered = count_covered_twice(&mut covered);
    println!("Covered twice: {}", twice_covered);
    find_unique_shapes(&mut line_buffer, &mut covered);
}

// the answer is 4
#[test]
fn shape_checker() {
    let mut line_buffer = read_input("/Users/bdhowe/git/aoc2018/day3/pt1/test_input.txt");
    let mut covered: HashMap<u32, HashMap<u32, Hits>> = HashMap::new();
    calculate_coverage(&mut covered, &mut line_buffer);
    let twice_covered = count_covered_twice(&mut covered);
    assert_eq!(twice_covered, 4);
    println!("Checking unique");
    find_unique_shapes(&mut line_buffer, &mut covered);
}
