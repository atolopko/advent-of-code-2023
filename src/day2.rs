use std::{path::Path, fs};

/*
struct ColorCounts {
    red: u8,
    green: u8,
    blue: u8,
}

impl ColorCounts {
    
}
*/
pub fn day2() -> u32 {
    let path = Path::new("data/day2.txt");
    let lines = fs::read_to_string(path).unwrap();
    let lines_itr = lines.lines();
    let mut answer: u32 = 0;
    for line in lines_itr {
        answer += parse_game(line);
        // println!("{} -> {}", line, value);
    }
    answer
}

/* struct Game {
    draws: [u8; 3],
} */

fn parse_game(line: &str) -> u32 {
    let mut id_draws = line.split(":");
    let id = id_draws
        .next()
        .unwrap()
        .rsplitn(2, " ")
        .next().unwrap()
        .parse::<u32>()
        .unwrap();
    let draws = id_draws.next().unwrap();
    for draw in draws.split(";").collect::<Vec<&str>>() {
        for color_counts in draw.split(",").collect::<Vec<&str>>() {
            let mut itr = color_counts.split_whitespace();
            let count = itr.next().unwrap().parse::<u32>().unwrap();
            let color = itr.next().unwrap();
            match color {
                "red" if count > 12 => {
                    println!("invalid game {id}; draw={draw}");
                    return 0;
                }
                "green" if count > 13 => {
                    println!("invalid game {id}; draw={draw}");
                    return 0;
                }
                "blue" if count > 14 => {
                    println!("invalid game {id}; draw={draw}");
                    return 0;
                }
                _ => {
                }
            }
        }
    }
    println!("GOOD game: {id}");
    id
}
