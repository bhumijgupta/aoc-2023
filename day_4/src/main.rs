use std::{
    collections::{HashMap, HashSet},
    fs,
};

const PART_ONE: bool = false;

fn main() {
    let input = fs::read_to_string("./assets/input3.txt").unwrap();
    let mut res = 0;

    let mut card_num = 1;
    let mut card_num_freq: HashMap<i32, i32> = HashMap::new();
    let total_cards = input.split("\n").collect::<Vec<&str>>().len();
    for line in input.lines() {
        let ans = process_card(line);
        if PART_ONE {
            res += ans;
        } else {
            println!(
                "Card: {card_num}, matched: {ans}, freq_map: {:?}",
                card_num_freq
            );

            let curr_card_count = match card_num_freq.get(&card_num) {
                Some(num) => *num,
                None => 1,
            };

            for card in 1..=ans {
                let card = card_num + card;
                if card > total_cards.try_into().unwrap() {
                    continue;
                }
                *card_num_freq.entry(card).or_insert(1) += curr_card_count;
            }

            res += match card_num_freq.get(&card_num) {
                Some(num) => {
                    println!("card {card_num}: {num}");
                    num
                }
                None => &1,
            }
        }
        card_num += 1;
    }

    println!("{res}")
}

fn process_card(card: &str) -> i32 {
    let mut matched_cards = 0;
    let card: Vec<&str> = card.split("|").collect();
    let winning_nums = card[0].split(":").collect::<Vec<&str>>()[1];
    let card_nums = card[1];

    let winning_nums = extract_card_nums(winning_nums);
    let mut winning_nums_set = HashSet::new();
    for num in winning_nums {
        winning_nums_set.insert(num);
    }

    let card_nums = extract_card_nums(card_nums);

    card_nums.iter().for_each(|num| {
        if winning_nums_set.contains(num) {
            matched_cards += 1;
        }
    });

    if matched_cards == 0 {
        return 0;
    }
    if PART_ONE {
        return 1 << (matched_cards - 1);
    }

    return matched_cards;
}

fn extract_card_nums(nums: &str) -> Vec<u32> {
    return nums
        .trim()
        .split(" ")
        .filter(|x| x.len() > 0)
        .map(|x| x.trim())
        .map(|x| x.parse::<u32>().unwrap())
        .collect();
}
