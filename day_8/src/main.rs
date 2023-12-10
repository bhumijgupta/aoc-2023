use indicatif::{ProgressBar, ProgressStyle};
use regex::Regex;
use std::{collections::HashMap, fs};

const PART_ONE: bool = false;
const OPTIMISED: bool = true;

fn main() {
    let input = fs::read_to_string("./assets/input2.txt").unwrap();
    let input_re = Regex::new(r"([A-Z]*) = \(([A-Z]*), ([A-Z]*)\)").unwrap();

    let mut steps: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut instruction = "";

    for (idx, line) in input.lines().enumerate() {
        if idx == 0 {
            instruction = line.trim();
            continue;
        }
        if idx < 2 {
            continue;
        }
        let (_, [start, left, right]) = input_re.captures(line).unwrap().extract();

        steps.insert(start, Vec::from([left, right]));
    }

    if PART_ONE {
        solve_part_a(instruction, &steps);
    } else {
        if !OPTIMISED {
            solve_part_b(instruction, &steps);
        } else {
            solve_part_b_optimised(instruction, &steps);
        }
    }
}

fn solve_part_a(instruction: &str, steps: &HashMap<&str, Vec<&str>>) {
    let mut curr_loc = "AAA";
    let mut num_steps = 0;
    while curr_loc != "ZZZ" {
        for ins in instruction.chars() {
            let possible_steps = steps.get(curr_loc).unwrap();
            if ins == 'L' {
                curr_loc = possible_steps[0];
            } else {
                curr_loc = possible_steps[1];
            }
            num_steps += 1;
            if curr_loc == "ZZZ" {
                break;
            }
        }
    }
    println!("{num_steps}")
}

fn solve_part_b(instruction: &str, steps: &HashMap<&str, Vec<&str>>) {
    let mut curr_loc: Vec<&str> = Vec::new();

    for loc in steps.keys() {
        if loc.ends_with("A") {
            curr_loc.push(loc);
        }
    }

    let mut num_steps = 0;

    let bar = ProgressBar::new(instruction.len().try_into().unwrap());
    bar.set_style(
        ProgressStyle::default_bar()
            .template("{wide_bar} {pos}/{len} ETA: {eta} Ops/s: {per_sec}")
            .unwrap(),
    );

    loop {
        bar.inc(1);
        let mut should_break = false;

        for ins in instruction.chars() {
            // println!("{:?}", curr_loc);

            let mut reached = true;

            for idx in 0..curr_loc.len() {
                let loc = curr_loc[idx];
                let possible_steps = steps.get(loc).unwrap();

                if ins == 'L' {
                    curr_loc[idx] = possible_steps[0];
                } else {
                    curr_loc[idx] = possible_steps[1];
                }
                reached &= curr_loc[idx].ends_with("Z");
            }
            num_steps += 1;

            if reached {
                should_break = true;
                break;
            }
        }
        if should_break {
            bar.finish();
            break;
        }
    }
    println!("{num_steps}")
}

fn solve_part_b_optimised(instruction: &str, steps: &HashMap<&str, Vec<&str>>) {
    println!("instruction_size: {}", instruction.len());

    let mut curr_loc: Vec<&str> = Vec::new();

    for loc in steps.keys() {
        if loc.ends_with("A") {
            curr_loc.push(loc);
        }
    }

    let mut all_loop_steps: Vec<u32> = vec![];

    // check looping for each starting point
    for start_loc in curr_loc {
        let mut loop_steps: Vec<u32> = vec![];

        let mut curr_location = start_loc;

        let mut curr_steps = 0;
        let mut cycle_completed = false;
        let mut first_dest = "";
        let mut dir_idx = 0;

        loop {
            while curr_steps == 0 || !curr_location.ends_with("Z") {
                let dir = instruction.chars().nth(dir_idx).unwrap();
                if dir == 'L' {
                    curr_location = steps.get(curr_location).unwrap()[0];
                } else {
                    curr_location = steps.get(curr_location).unwrap()[1];
                }
                curr_steps += 1;
                dir_idx = (dir_idx + 1) % instruction.len();
            }

            if first_dest == "" {
                first_dest = curr_location;
            } else if first_dest == curr_location {
                cycle_completed = true;
            }

            loop_steps.push(curr_steps);
            curr_steps = 0;

            if cycle_completed {
                break;
            }
        }
        all_loop_steps.push(loop_steps[0]);
        println!("{:#?}", loop_steps);
    }

    let mut res: u64 = all_loop_steps[0] as u64;

    for ls in all_loop_steps {
        res = (res * (ls as u64)) / gcd(res, ls as u64)
    }
    println!("{res}")
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }

    return a;
}
