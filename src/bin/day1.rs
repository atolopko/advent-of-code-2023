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

fn day1() {
    let path = Path::new("data/day1.txt");
    let lines = fs::read_to_string(path).unwrap();
    let lines_itr = lines.lines();
    let mut answer = 0;
    for line in lines_itr {
        let tens_idx = line.find(|c: char| c.is_digit(10)).unwrap();
        let ones_idx = line.rfind(|c: char| c.is_digit(10)).unwrap();
        let tens: u16 = line.get(tens_idx..(tens_idx+1)).unwrap().parse().unwrap();
        let ones: u16 = line.get(ones_idx..(ones_idx+1)).unwrap().parse().unwrap();
        let value = tens * 10 + ones;
        // println!("{} -> {}", line, value);
        answer += value;
    }
    println!("{}", answer);
}

fn main() {
    day1();
}