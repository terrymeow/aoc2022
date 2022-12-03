use std::fs;
const TEST_PATH: &str = "test/1";

fn puzzle(input: String, elves: usize) {
    let mut sum = 0;
    let mut max = Vec::new();
    for l in input.lines() {
        match l {
            "" => {
                max.push(sum);
                sum = 0;
            },
            _ => sum += l.parse::<u32>().unwrap()
        }
    }
    max.push(sum);
    max.sort_by(|a, b| b.cmp(a));

    println!("{}", &max[0..elves].iter().sum::<u32>());
}

#[test]
fn test_1a() {
    let input = fs::read_to_string(format!("{TEST_PATH}a.txt")).unwrap();
    puzzle(input, 1)
}

#[test]
fn test_1b() {
    let input = fs::read_to_string(format!("{TEST_PATH}b.txt")).unwrap();
    puzzle(input, 1)
}

#[test]
fn test_2a() {
    let input = fs::read_to_string(format!("{TEST_PATH}a.txt")).unwrap();
    puzzle(input, 3)
}

#[test]
fn test_2b() {
    let input = fs::read_to_string(format!("{TEST_PATH}b.txt")).unwrap();
    puzzle(input, 3)
}

fn main() {}