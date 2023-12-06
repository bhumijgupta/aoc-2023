use std::cmp::max;
use std::fs;

// const MAX_RED: u32 = 12;
// const MAX_GREEN: u32 = 13;
// const MAX_BLUE: u32 = 14;

fn main() {
    let file_content = fs::read_to_string("./assets/input4.txt").expect("Error reading file");
    let mut res = 0;

    for line in file_content.lines() {
        let metadata: Vec<&str> = line.split(": ").collect();
        let chances: Vec<&str> = metadata[1].split("; ").collect();
        // println!("{game_num}: {:?}", chances);

        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        for chance in chances {
            let cube_sets: Vec<&str> = chance.split(", ").collect();
            for cube_set in cube_sets {
                let cube_set: Vec<&str> = cube_set.split(" ").collect();
                let count: u32 = cube_set[0].parse().unwrap();
                let color = cube_set[1];
                if color == "red" {
                    red = max(count, red);
                } else if color == "blue" {
                    blue = max(count, blue);
                } else if color == "green" {
                    green = max(count, green);
                }
            }
        }

        res += red * green * blue
    }
    println!("{}", res)
}
