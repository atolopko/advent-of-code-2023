/*

[Y]ou have to figure out which of the numbers you have appear in the list of winning numbers. The first match makes the card worth one point and each match after the first doubles the point value of that card.

For example:

Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
In the above example, card 1 has five winning numbers (41, 48, 83, 86, and 17) and eight numbers you have (83, 86, 6, 31, 17, 9, 48, and 53). Of the numbers you have, four of them (48, 83, 17, and 86) are winning numbers! That means card 1 is worth 8 points (1 for the first match, then doubled three times for each of the three matches after the first).

Card 2 has two winning numbers (32 and 61), so it is worth 2 points.
Card 3 has two winning numbers (1 and 21), so it is worth 2 points.
Card 4 has one winning number (84), so it is worth 1 point.
Card 5 has no winning numbers, so it is worth no points.
Card 6 has no winning numbers, so it is worth no points.
So, in this example, the Elf's pile of scratchcards is worth 13 points.

Take a seat in the large pile of colorful cards. How many points are they worth in total? 
*/

use std::{fs, path::Path, collections::HashSet};

use regex::Regex;

pub fn day4a() -> u32 {
    let path = Path::new("data/day4.txt");
    let lines = fs::read_to_string(path).unwrap();
    let re = Regex::new(r"^Card\s+(\d+):(.+)\|(.+)$").unwrap();
    let lines_itr = lines.lines();
    let mut answer: u32 = 0;
    for line in lines_itr {
        let (_, [_, winning_numbs_str, selected_numbs_str]) = re.captures(line).unwrap().extract();
        let selected_numbs: HashSet<u16> = selected_numbs_str.split_ascii_whitespace().map(|s| s.parse::<u16>().unwrap()).collect();
        let winning_numbs: HashSet<u16> = winning_numbs_str.split_ascii_whitespace().map(|s| s.parse::<u16>().unwrap()).collect();
        let matches = winning_numbs.intersection(&selected_numbs).count() as u32;
        let points = if matches > 0 { u32::pow(2, matches - 1) } else { 0 };
        answer += points;
    }
    answer
}


/*
[Y]ou win copies of the scratchcards below the winning card equal to the number of matches. So, if card 10 were to have 5 matching numbers, you would win one copy each of cards 11, 12, 13, 14, and 15.

Copies of scratchcards are scored like normal scratchcards and have the same card number as the card they copied. So, if you win a copy of card 10 and it has 5 matching numbers, it would then win a copy of the same cards that the original card 10 won: cards 11, 12, 13, 14, and 15. This process repeats until none of the copies cause you to win any more ca

Process all of the original and copied scratchcards until no more scratchcards are won. Including the original set of scratchcards, how many total scratchcards do you end up with?
 */

pub fn day4b() -> u32 {
    let path = Path::new("data/day4.txt");
    let lines = fs::read_to_string(path).unwrap();
    let re = Regex::new(r"^Card\s+(\d+):(.+)\|(.+)$").unwrap();
    let lines_itr = lines.lines();
    let n_lines = lines.lines().count();
    let mut card_counts: Vec<u32> = vec![1; n_lines];
    println!("{}", card_counts.len());
    for line in lines_itr {
        let (_, [card_str, winning_numbs_str, selected_numbs_str]) = re.captures(line).unwrap().extract();
        let card: usize = card_str.parse().unwrap();
        let card_idx: usize = card - 1;
        let selected_numbs: HashSet<u16> = selected_numbs_str.split_ascii_whitespace().map(|s| s.parse::<u16>().unwrap()).collect();
        let winning_numbs: HashSet<u16> = winning_numbs_str.split_ascii_whitespace().map(|s| s.parse::<u16>().unwrap()).collect();
        let matches = winning_numbs.intersection(&selected_numbs).count();
        let card_count = *card_counts.get(card_idx).unwrap();
        for i in (card_idx + 1)..(card_idx + 1 + matches) {
            if let Some(cnt) = card_counts.get_mut(i) {
                *cnt += card_count;
            }
        }
    }
    let answer: u32 = card_counts.iter().sum::<u32>();
    answer
}
