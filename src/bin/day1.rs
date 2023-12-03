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

use std::path::Path;
use std::fs;

fn day1() -> u32 {
    let path = Path::new("data/day1.txt");
    let lines = fs::read_to_string(path).unwrap();
    let lines_itr = lines.lines();
    let mut answer = 0;
    let parse_digit = |s: &str, rev: bool| -> u16 {
        let find_fn = match rev {
            true => str::rfind,
            false => str::find,
        };
        let d_idx = find_fn(s, |c: char| c.is_ascii_digit()).unwrap();
        s.get(d_idx..(d_idx+1)).unwrap().parse::<u16>().unwrap().into()
    };
    for line in lines_itr {
        let ones = parse_digit(&line, true);
        let tens = parse_digit(&line, false);
        let value: u32 = (tens * 10 + ones).into();
        // println!("{} -> {}", line, value);
        answer += value;
    }
    answer
}

fn main() {
    println!("Answer: {}", day1());
}