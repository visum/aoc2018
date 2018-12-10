#[macro_use] extern crate lazy_static;
extern crate regex;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::collections::HashMap;
use regex::Regex;

struct Cell {
    x: u32,
    y: u32,
}

struct Shape {
    top: u32,
    right: u32,
    bottom: u32,
    left: u32,
    width: u32,
    height: u32,
}

enum Hits {
    Once,
    Twice
}

fn read_input(file_name: &str ) -> BufReader<File> {
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
    shape.left = captures.get(2).unwrap().as_str().parse().unwrap();
    shape.top = captures.get(3).unwrap().as_str().parse().unwrap();
    shape.width = captures.get(4).unwrap().as_str().parse().unwrap();
    shape.height = captures.get(5).unwrap().as_str().parse().unwrap();
    shape.right = shape.left + shape.width;
    shape.bottom = shape.top + shape.height;
}

fn calculate_coverage(covered: &mut HashMap<u32, HashMap<u32,Hits>>, lines: &BufReader<File>) {
    for line in lines.lines() {
        let mut shape = Shape{top: 0, right: 0, left: 0, bottom: 0, width: 0, height: 0};
        parse_line(&line.unwrap(), &mut shape);
        let mut row: &HashMap<u32,Hits> = match covered.get(&shape.top) {
            None => {
                let new_row: HashMap<u32,Hits> = HashMap::new();
                covered.insert(shape.top, new_row);
                &new_row
            },
            Some(y) => y,
        };
        for column in shape.left..=shape.right {
            match row.get_mut(&column) {
                None => {row.insert(column, Hits::Once);},
                Some(y) => match y {
                    Hits::Once => *y = Hits::Twice,
                    _ => ()
                }
            };
        }
    }
}

fn count_covered_twice(covered: &mut HashMap<u32, HashMap<u32,Hits>>) -> u32 {
    let mut count: u32 = 0;
    for row in covered.values() {
        for column in row.values() {
            match column {
                Hits::Twice => count += 1,
                _ => ()
            }
        }
    }

    count
}


fn main() {
    let line_buffer = read_input("/Users/bdhowe/git/aoc2018/day3/pt1/input.txt");
    let mut covered: HashMap<u32, HashMap<u32, Hits>> = HashMap::new();
    calculate_coverage(&mut covered, &line_buffer);
}

#[cfg(test)]
mod tests {
    #[test]
    // the answer is 4
    fn shape_checker() {
        let line_buffer = read_input("/Users/bdhowe/git/aoc2018/day3/pt1/test_input.txt");
        let mut covered: HashMap<u32, HashMap<Hits, u32>> = HashMap::new();
        calculate_coverage(&mut covered, &line_buffer);
        let twice_covered = count_covered_twice(& covered);
        assert_eq!(twice_covered, 4);
    }
}