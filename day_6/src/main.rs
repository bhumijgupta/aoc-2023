use std::{cmp, fs, time::Instant};

const PART_ONE: bool = false;
const BRUTE_FORCE: bool = false;

fn main() {
    let input = fs::read_to_string("./assets/input2.txt").unwrap();
    let input: Vec<&str> = input.split("\n").collect();
    let time_input = process_line(input[0]);
    let distance_input = process_line(input[1]);

    let now = Instant::now();

    if PART_ONE {
        let mut resp = 1;
        for race in 0..time_input.len() {
            println!("Processing race: {}", race + 1);
            if BRUTE_FORCE {
                resp *= calculate_winning_ways_brute_force(
                    &(time_input[race] as u64),
                    &(distance_input[race] as u64),
                )
            } else {
                resp *= calculate_winning_ways_optimised(
                    &(time_input[race] as u64),
                    &(distance_input[race] as u64),
                )
            }
        }

        println!("{resp}");
    } else {
        let race_time = time_input
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join("")
            .parse::<u64>()
            .unwrap();

        let min_distance = distance_input
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join("")
            .parse::<u64>()
            .unwrap();

        let mut resp = 0;

        if BRUTE_FORCE {
            resp = calculate_winning_ways_brute_force(&race_time, &min_distance);
        } else {
            resp = calculate_winning_ways_optimised(&race_time, &min_distance);
        }
        println!("{}", resp);
    }

    println!("Took {} micro seconds", now.elapsed().as_micros());
}

fn calculate_winning_ways_brute_force(race_time: &u64, min_dist: &u64) -> u64 {
    let mut num_ways = 0;

    for waiting_time in 1..*race_time {
        let speed = waiting_time;
        let remaining_time = race_time - waiting_time;
        let dist = speed * remaining_time;
        if dist > *min_dist {
            num_ways += 1;
        }
    }

    return num_ways;
}

fn calculate_winning_ways_optimised(race_time: &u64, min_dist: &u64) -> u64 {
    let min_wait_win_time = find_min_wait_win(race_time, min_dist);
    let max_wait_win_time = find_max_wait_win(race_time, min_dist);

    return max_wait_win_time - min_wait_win_time + 1;
}

fn find_min_wait_win(race_time: &u64, min_dist: &u64) -> u64 {
    // println!("Case time: {}, dist: {}", race_time, min_dist);

    let mut l = 0;
    let mut r = *race_time;
    let mut ans = 0;

    while l <= r {
        let mid = (l + r) / 2;
        // println!("  Checking {}", mid);

        let speed = mid;
        let remaining_time = race_time - mid;
        let dist = speed * remaining_time;

        // println!("      Dist: {}", dist);
        match dist.cmp(min_dist) {
            cmp::Ordering::Less => {
                l = mid + 1;
            }
            cmp::Ordering::Equal => {
                l = mid + 1;
            }
            cmp::Ordering::Greater => {
                ans = mid;
                r = mid - 1;
            }
        }
        // println!("      Ans: {}", ans)
    }

    // println!("min: {}", ans);

    return ans;
}

fn find_max_wait_win(race_time: &u64, min_dist: &u64) -> u64 {
    // println!("Case time: {}, dist: {}", race_time, min_dist);
    let mut l = 0;
    let mut r = *race_time;
    let mut ans = 0;

    while l <= r {
        let mid = (l + r) / 2;
        // println!("  Checking {}", mid);

        let speed = mid;
        let remaining_time = race_time - mid;
        let dist = speed * remaining_time;

        // println!("      Dist: {}", dist);
        match dist.cmp(min_dist) {
            cmp::Ordering::Greater => {
                ans = mid;
                l = mid + 1;
            }
            cmp::Ordering::Equal => {
                r = mid - 1;
            }
            cmp::Ordering::Less => {
                r = mid - 1;
            }
        }
        // println!("      Ans: {}", ans);
    }

    // println!("max: {}", ans);

    return ans;
}

fn process_line(line: &str) -> Vec<u32> {
    return line
        .split(":")
        .last()
        .unwrap()
        .split_ascii_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect();
}
