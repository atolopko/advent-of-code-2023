use std::{path::Path, fs};

struct Draw {
    red: u8,
    green: u8,
    blue: u8,
}

pub fn day2a() -> u32 {
    let path = Path::new("data/day2.txt");
    let lines = fs::read_to_string(path).unwrap();
    let lines_itr = lines.lines();
    let mut answer: u32 = 0;
    for line in lines_itr {
        let (id, draws) = parse_game(line);
        let max_red = draws.iter().max_by_key(|d| d.red).unwrap().red;
        let max_green = draws.iter().max_by_key(|d| d.green).unwrap().green;
        let max_blue = draws.iter().max_by_key(|d| d.blue).unwrap().blue;
        if max_red <= 12 && max_green <= 13 && max_blue <= 14 {
            answer += id;

        }
    }
    answer
}

pub fn day2b() -> u32 {
    let path = Path::new("data/day2.txt");
    let lines = fs::read_to_string(path).unwrap();
    let lines_itr = lines.lines();
    let mut answer: u32 = 0;
    for line in lines_itr {
        let (_id, draws) = parse_game(line);
        let max_red = draws.iter().max_by_key(|d| d.red).unwrap().red;
        let max_green = draws.iter().max_by_key(|d| d.green).unwrap().green;
        let max_blue = draws.iter().max_by_key(|d| d.blue).unwrap().blue;
        let power: u32 = max_red as u32 * max_green as u32 * max_blue as u32;
        answer += power;
    }
    answer
}

fn parse_game(line: &str) -> (u32, Vec<Draw>) {
    let mut draws = Vec::<Draw>::new();
    let mut id_draws = line.split(":");
    let id = id_draws
        .next()
        .unwrap()
        .rsplitn(2, " ")
        .next().unwrap()
        .parse::<u32>()
        .unwrap();
    let draws_str = id_draws.next().unwrap();

    for draw_str in draws_str.split(";").collect::<Vec<&str>>() {
        let mut red: u8 = 0;
        let mut blue: u8 = 0;
        let mut green: u8 = 0;

        for color_counts in draw_str.split(",").collect::<Vec<&str>>() {
            let mut itr = color_counts.split_whitespace();
            let count = itr.next().unwrap().parse::<u8>().unwrap();
            let color = itr.next().unwrap();
            match color {
                "red" => {
                    red = count
                }
                "green" => {
                    green = count
                }
                "blue" => {
                    blue = count
                }
                _ => {}
            }
        }
        draws.push(Draw { red: red, green: green, blue: blue });
    }
    (id, draws)
}
