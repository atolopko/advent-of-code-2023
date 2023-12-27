use std::{fs, path::Path, iter::zip, str::from_utf8, cmp::{max, min}};

use multimap::MultiMap;

pub fn day3a() -> u32 {
    let path = Path::new("data/day3.txt");
    let rows_iter = fs::read_to_string(path).unwrap();
    // Add an extra column to simplify end-of-line processing
    let rows_s: Vec<String> = rows_iter.lines().map(|l| -> String { String::from(l) + "." }).collect();
    let rows: Vec<&[u8]> = rows_s.iter().map(|s| s.as_bytes()).collect();
    let row_len = rows.get(0).unwrap().len();
    let mut parts: Vec<u32> = Vec::new();


    for (j, row) in zip(0..rows.len(), rows.clone()) {
        let mut i_start: i32 = -1;
        for (i, c) in zip(0..row.len(), row) {
            if c.is_ascii_digit() {
                if i_start < 0 {
                    // start scanning part number
                    i_start = i as i32;
                }
            } else {
                if i_start >= 0 {
                    // end scanning part number
                    if is_symbol_adjacent_str(i_start as usize, i, j, &rows, row_len) {
                        let part_number = from_utf8(&row[i_start as usize..i]).unwrap();
                        parts.push(part_number.parse().unwrap());
                        // println!("{}", part);
                    }
                    i_start = -1;
                }
            }
        }
    }
    return parts.iter().sum()
}


fn is_symbol_adjacent_str(i1: usize, i2: usize, j: usize, rows: &Vec<&[u8]>, row_len: usize) -> bool {
    let is_symbol= |i: usize, j: usize| -> bool {
        rows[j][i] as char != '.' && !rows[j][i].is_ascii_digit()
    };

    // check left- and right-flanking symbol
    if (i1 > 0 && is_symbol(i1 - 1, j)) || (i2 < row_len && is_symbol(i2, j)) { 
        return true
    }

    // check rows above and below, extending one extra char to the left and right (safely)
    let h1 = max(0, i1 as i32 - 1) as usize;
    let h2 = min(i2 + 1, row_len);
    for i in h1..h2 {
        if (j > 0 && is_symbol(i, j - 1)) || (j < rows.len() - 1 && is_symbol(i, j + 1)) { 
            return true
        }
    }

    false
}

pub fn day3b() -> u32 {
    let path = Path::new("data/day3.txt");
    let input = fs::read_to_string(path).unwrap();
    // Add an extra column to simplify end-of-line processing
    let rows_s: Vec<String> = input.lines().map(|l| -> String { String::from(l) + "." }).collect();
    let rows: Vec<&[u8]> = rows_s.iter().map(|s| s.as_bytes()).collect();
    let row_len = rows.get(0).unwrap().len();
    let mut candidate_gears: MultiMap<(usize, usize), u32> = MultiMap::new();

    for (j, row) in zip(0..rows.len(), rows.clone()) {
        let mut i_start: i32 = -1;
        for (i, c) in zip(0..row.len(), row) {
            if c.is_ascii_digit() {
                if i_start < 0 {
                    // start scanning part number
                    i_start = i as i32;
                }
            } else {
                if i_start >= 0 {
                    // end scanning part number
                    let part_str = from_utf8(&row[i_start as usize..i]).unwrap();
                    let part_number = part_str.parse().unwrap();

                    for symb_coord in find_adjacent_symbols(i_start as usize, i, j, &rows, row_len) {
                        candidate_gears.insert(symb_coord, part_number)
                    }

                    i_start = -1;
                }
            }
        }
    }
    let mut result: u32 = 0;
    for (_symb_coord, part_numbers) in candidate_gears {
        // println!("{:?}: {:?}", symb_coord, part_numbers);
        if part_numbers.len() > 1 {
            result += part_numbers.iter().product::<u32>();
        }
    }
    result
}


fn find_adjacent_symbols(i1: usize, i2: usize, j: usize, rows: &Vec<&[u8]>, row_len: usize) -> Vec<(usize, usize)> {
    let mut adj_symbs = Vec::new();

    let is_symbol= |i: usize, j: usize| -> bool {
        rows[j][i] as char != '.' && !rows[j][i].is_ascii_digit()
    };

    // check left- and right-flanking symbol
    if i1 > 0 && is_symbol(i1 - 1, j) {
        adj_symbs.push((j, i1 - 1));
    }
    if i2 < row_len && is_symbol(i2, j) { 
        adj_symbs.push((j, i2));
    }

    // check rows above and below, extending one extra char to the left and right (safely)
    let h1 = max(0, i1 as i32 - 1) as usize;
    let h2 = min(i2 + 1, row_len);
    for i in h1..h2 {
        if j > 0 && is_symbol(i, j - 1) {
            adj_symbs.push((j - 1, i));
        }
        if j < rows.len() - 1 && is_symbol(i, j + 1) { 
            adj_symbs.push((j + 1, i));
        }
    }

    adj_symbs
}