use std::fs;
const TEST_PATH: &str = "test/2";

fn puzzle1(input: String) {
    let mut sum = 0;
    for l in input.lines() {
        let tuple = l.split_once(" ").unwrap();
        sum += match tuple { // shape reward
            (_, "X") => 1,
            (_, "Y") => 2,
            (_, "Z") => 3,
            _ => 0
        };
        sum += match tuple { // win/draw reward
            ("C", "X") | ("A", "Y") | ("B", "Z") => 6,
            ("A", "X") | ("B", "Y") | ("C", "Z") => 3,
            _ => 0
        };
    }

    println!("{sum}");
}

fn puzzle2(input: String) {
    let mut sum = 0;
    for l in input.lines() {
        let tuple = l.split_once(" ").unwrap();
        sum += match tuple { // win/draw reward
            (_, "Z") => 6,
            (_, "Y") => 3,
            _ => 0
        };
        sum += match tuple { // shape reward
            ("B", "X") | ("A", "Y") | ("C", "Z") => 1,
            ("C", "X") | ("B", "Y") | ("A", "Z") => 2,
            ("A", "X") | ("C", "Y") | ("B", "Z") => 3,
            _ => 0
        };
    }

    println!("{sum}");
}

#[test]
fn test_1a() {
    let input = fs::read_to_string(format!("{TEST_PATH}a.txt")).unwrap();
    puzzle1(input)
}

#[test]
fn test_1b() {
    let input = fs::read_to_string(format!("{TEST_PATH}b.txt")).unwrap();
    puzzle1(input)
}

#[test]
fn test_2a() {
    let input = fs::read_to_string(format!("{TEST_PATH}a.txt")).unwrap();
    puzzle2(input)
}

#[test]
fn test_2b() {
    let input = fs::read_to_string(format!("{TEST_PATH}b.txt")).unwrap();
    puzzle2(input)
}

fn main() {}