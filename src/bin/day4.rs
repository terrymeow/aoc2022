use std::fs;
const TEST_PATH: &str = "test/4";

fn puzzle(input: String, any_overlap: bool) {
	let mut sum = 0;
	for l in input.lines() {
		let digits: Vec<_> = l.split(|x: char| !x.is_numeric())
			.map(|x| x.parse::<u32>().unwrap())
			.collect();
		let range1 = digits[0]..=digits[1];
		let range2 = digits[2]..=digits[3];

		let r1r2s = range1.contains(range2.start());
		let r1r2e = range1.contains(range2.end());
		let r2r1s = range2.contains(range1.start());
		let r2r1e = range2.contains(range1.end());

		if any_overlap {
			sum += ((r1r2s || r1r2e) || (r2r1s || r2r1e)) as i32;
		} else {
			sum += ((r1r2s && r1r2e) || (r2r1s && r2r1e)) as i32;
		}
	}

	println!("{sum}");
}

#[test]
fn test_1a() {
	let input = fs::read_to_string(format!("{TEST_PATH}a.txt")).unwrap();
	puzzle(input, false)
}

#[test]
fn test_1b() {
	let input = fs::read_to_string(format!("{TEST_PATH}b.txt")).unwrap();
	puzzle(input, false)
}

#[test]
fn test_2a() {
	let input = fs::read_to_string(format!("{TEST_PATH}a.txt")).unwrap();
	puzzle(input, true)
}

#[test]
fn test_2b() {
	let input = fs::read_to_string(format!("{TEST_PATH}b.txt")).unwrap();
	puzzle(input, true)
}

fn main() {}
