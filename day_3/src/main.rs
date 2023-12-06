use std::{
    collections::{HashMap, HashSet},
    fs,
};

struct PartInfo {
    part_number: u32,
    row_num: u32,
    col_num: u32,
}

const PART_ONE: bool = false;

fn main() {
    let input = fs::read_to_string("./assets/input3.txt").unwrap();

    let preprocess_part_info = preprocess_input(&input);

    let mut row_num = 0;
    let mut col_num = 0;
    let mut res = 0;
    let mut included_parts: HashSet<String> = HashSet::new();
    for line in input.lines() {
        for char in line.chars() {
            if !(char >= '0' && char <= '9') && char != '.' {
                res += compute_adjacent_part_numbers(
                    row_num,
                    col_num,
                    &preprocess_part_info,
                    &mut included_parts,
                )
            }
            col_num += 1;
        }

        row_num += 1;
        col_num = 0;
    }

    println!("{res}")
}

fn compute_adjacent_part_numbers(
    row: u32,
    col: u32,
    index_num_map: &HashMap<String, PartInfo>,
    included_part: &mut HashSet<String>,
) -> u32 {
    let mut res: u32 = 0;
    let mut num_adj = 0;
    for row in row - 1..=row + 1 {
        for col in col - 1..=col + 1 {
            let key = generate_key(row, col);
            match index_num_map.get(&key) {
                Some(num) => {
                    let part_info_key =
                        format!("{}:{}:{}", num.part_number, num.row_num, num.col_num);
                    if PART_ONE {
                        if !included_part.contains(&part_info_key) {
                            res += num.part_number;
                            println!("Including: {}", num.part_number);
                            included_part.insert(part_info_key);
                        }
                    } else {
                        if !included_part.contains(&part_info_key) {
                            num_adj += 1;
                            if num_adj == 1 {
                                res = 1;
                            }
                            res *= num.part_number;
                            println!("Including: {}", num.part_number);
                            included_part.insert(part_info_key);
                        }
                    }
                }
                None => continue,
            }
        }
    }
    if !PART_ONE {
        if num_adj == 1 {
            res = 0;
        } else {
            println!("{}:{}", row, col)
        }
    }
    return res;
}

fn generate_key(row_num: u32, index: u32) -> String {
    format!("{}:{}", row_num, index)
}

fn process_number(
    num: &str,
    si: u32,
    ei: u32,
    row_num: u32,
    index_num_map: &mut HashMap<String, PartInfo>,
) {
    let num: u32 = num.parse().unwrap();
    println!("Processing: {}", num);

    for i in si..=ei {
        index_num_map.insert(
            generate_key(row_num, i),
            PartInfo {
                part_number: num,
                row_num: row_num,
                col_num: si,
            },
        );
    }
}

fn preprocess_input(input: &String) -> HashMap<String, PartInfo> {
    let mut index_num_map: HashMap<String, PartInfo> = HashMap::new();

    let mut row_num = 0;
    let mut col_num: u32 = 0;
    for line in input.lines() {
        let mut num_start: i32 = -1;
        for char in line.chars() {
            // Process digit
            if char >= '0' && char <= '9' {
                if num_start == -1 {
                    num_start = col_num as i32;
                }
            } else if num_start != -1 {
                process_number(
                    line.get(num_start as usize..col_num as usize).unwrap(),
                    num_start as u32,
                    col_num - 1,
                    row_num,
                    &mut index_num_map,
                );
                num_start = -1;
            }

            col_num += 1;
        }
        if num_start != -1 {
            process_number(
                line.get(num_start as usize..col_num as usize).unwrap(),
                num_start as u32,
                col_num - 1,
                row_num,
                &mut index_num_map,
            );
        }

        row_num += 1;
        col_num = 0;
    }

    return index_num_map;
}
