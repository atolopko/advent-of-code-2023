/*

The newly-improved calibration document consists of lines of text; each line originally contained a specific calibration value that the Elves now need to recover. On each line, the calibration value can be found by combining the first digit and the last digit (in that order) to form a single two-digit number.

For example:

1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
In this example, the calibration values of these four lines are 12, 38, 15, and 77. Adding these together produces 142.

Consider your entire calibration document. What is the sum of all of the calibration values?
*/

use std::fs;
use std::path::Path;

const DIGIT_NAMES: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

/// Replace occurrences of digit names ("one" through "nine") with their numeric representations.
/// Proceeds left to right, thereby giving preference to the first match ("oneight" will become "1ight").
fn parse_digit(s: &str, rev: bool) -> u32 {
    for i in 0..s.len() {
        let p = if rev { s.len() - (i + 1) } else { i };
        for (d, digit_name) in (1..=9).zip(DIGIT_NAMES.iter()) {
            if s.get(p..(p + digit_name.len()))
                .is_some_and(|ss| ss.starts_with(digit_name))
                || s.get(p..(p + 1))
                    .is_some_and(|ss| ss.starts_with(d.to_string().as_str()))
            {
                return d;
            }
        }
    }
    panic!("no digits found")
}

fn day1() -> u32 {
    let path = Path::new("data/day1.txt");
    let lines = fs::read_to_string(path).unwrap();
    let lines_itr = lines.lines();
    let mut answer: u32 = 0;
    for line in lines_itr {
        let ones = parse_digit(line, true);
        let tens = parse_digit(line, false);
        let value = tens * 10 + ones;
        // println!("{} -> {}", line, value);
        answer += value;
    }
    answer
}

fn main() {
    println!("Answer: {}", day1());
}
