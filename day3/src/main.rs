#[macro_use]
extern crate lazy_static;
extern crate regex;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug)]
struct Shape {
    top: u32,
    right: u32,
    bottom: u32,
    left: u32,
    width: u32,
    height: u32,
    id: u32
}

impl Shape {
    fn new() -> Shape {
        Shape {
            top: 0,
            right: 0,
            left: 0,
            bottom: 0,
            width: 0,
            height: 0,
            id:0
        }
    }
}

struct Coverage {
    shapes: Vec<Shape>,
    covered_cells: HashMap<u32, HashMap<u32, Hits>>
}

#[derive(Debug)]
enum Hits {
    Once,
    Twice,
}

impl Coverage {
    fn new() -> Coverage {
        Coverage {
            shapes: Vec::new(),
            covered_cells: HashMap::new()
        }
    }

    fn add(&mut self, shape: Shape) {

        for row in shape.top..shape.bottom {
            let row_hash = self.covered_cells.entry(row).or_insert(HashMap::new());

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

        self.shapes.push(shape);
    }

    fn count_covered_twice(&mut self) -> u32 {
        let mut count: u32 = 0;
        for row in self.covered_cells.values() {
            for column in row.values() {
                match column {
                    Hits::Twice => count += 1,
                    _ => (),
                }
            }
        }

        count
    }

    fn shape_is_unique(& self, shape: &Shape) -> bool {
        for row in shape.top..shape.bottom {
        for column in shape.left..shape.right {
            let coverage = self.covered_cells.get(&row).expect("invalid row").get(&column).expect("invalid column");
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

    fn find_unique_shapes(& self) -> Vec<&Shape> {
        let mut unique_shapes: Vec<&Shape> = Vec::new();
        for shape in & self.shapes {
            if self.shape_is_unique(&shape) {
                unique_shapes.push(shape);
            }
        };

        unique_shapes
    }
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

fn main() {
    let line_buffer = read_input("/Users/bdhowe/git/aoc2018/day3/input.txt");
    let mut coverage: Coverage = Coverage::new();

    for line in line_buffer.lines() {
        let mut shape = Shape::new();
        parse_line(&line.unwrap(), &mut shape);
        coverage.add(shape);
    }

    println!("Covered twice: {}", coverage.count_covered_twice());
    println!("Unique: {:?}", coverage.find_unique_shapes());
}

// the answer is 4
#[test]
fn shape_checker() {
    let line_buffer = read_input("/Users/bdhowe/git/aoc2018/day3/test_input.txt");
    let mut coverage: Coverage = Coverage::new();

    for line in line_buffer.lines() {
        let mut shape = Shape::new();
        parse_line(&line.unwrap(), &mut shape);
        coverage.add(shape);
    }

    let twice_covered = coverage.count_covered_twice();
    assert_eq!(twice_covered, 4);
    let unique_shapes = coverage.find_unique_shapes();
    println!("{:?}", unique_shapes);
}