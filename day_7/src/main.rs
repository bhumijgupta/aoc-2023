use std::{cmp::Ordering, collections::HashMap, fs};

#[derive(Debug, PartialEq, PartialOrd)]
enum HandType {
    FiveOfKind = 6,
    FourOfKind = 5,
    FullHouse = 4,
    ThreeOfKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0,
}

#[derive(Debug)]
struct HandBid {
    hand: String,
    bid: u32,
    hand_type: HandType,
}

impl HandBid {
    fn from(line: &str) -> Self {
        let line: Vec<&str> = line.split(" ").collect();
        let mut hand = String::from(line[0]);
        let bid = line[1].parse().unwrap();
        let hand_type = HandBid::get_hand_type(&mut hand);
        println!("Hand: {hand}, type: {:?}", hand_type);
        HandBid {
            hand: hand,
            bid: bid,
            hand_type: hand_type,
        }
    }

    fn get_hand_type(hand: &mut String) -> HandType {
        let mut char_freq_map: HashMap<char, u32> = HashMap::new();

        hand.chars().for_each(|c| {
            let freq = char_freq_map.entry(c).or_insert(0);
            *freq += 1;
        });
        // println!("Hand: {}, Freq:{:?}", hand, char_freq_map);

        let mut max_char_freq = 0;
        let mut max_freq_char = '_';
        let mut num_chars_max_freq = 0;
        let mut joker_freq = 0;
        char_freq_map.iter().for_each(|(k, v)| {
            if *v > max_char_freq && *k != 'J' {
                num_chars_max_freq = 1;

                max_freq_char = *k;
                max_char_freq = *v;
            }
            if *v == max_char_freq && max_char_freq > 0 {
                num_chars_max_freq += 1;
            }
            if *k == 'J' {
                joker_freq = *v;
            }
        });

        if max_freq_char == '_' {
            return HandType::FiveOfKind;
        }

        if joker_freq > 0 {
            let freq = char_freq_map.get_mut(&max_freq_char).unwrap();
            *freq += joker_freq;

            char_freq_map.remove(&'J');
        }

        if char_freq_map.len() == 5 {
            HandType::HighCard
        } else if char_freq_map.len() == 4 {
            HandType::OnePair
        } else if char_freq_map.len() == 3 {
            // 11123, JJ123 (11123), J1123 (11123)
            let max_freq = char_freq_map.values().max().unwrap();
            if *max_freq == 3 {
                return HandType::ThreeOfKind;
            }
            // 11223
            HandType::TwoPair
        } else if char_freq_map.len() == 2 {
            let max_freq = char_freq_map.values().max().unwrap();
            // J1112, 11122,
            if *max_freq == 3 {
                return HandType::FullHouse;
            }
            // JJJ12, 11112, J1122
            HandType::FourOfKind
        } else {
            // JJJJJ, JJJJ1, JJJ11, JJ111, J1111
            HandType::FiveOfKind
        }
    }
}

const PART_A: bool = false;

fn main() {
    let input = fs::read_to_string("./assets/input4.txt").unwrap();
    let mut card_ranking: HashMap<char, u32> = HashMap::from([
        ('1', 1),
        ('2', 2),
        ('3', 3),
        ('4', 4),
        ('5', 5),
        ('6', 6),
        ('7', 7),
        ('8', 8),
        ('9', 9),
        ('T', 10),
        ('J', 11),
        ('Q', 12),
        ('K', 13),
        ('A', 14),
    ]);
    if !PART_A {
        *card_ranking.get_mut(&'J').unwrap() = 0
    }

    let mut all_hands: Vec<HandBid> = Vec::new();

    for line in input.lines() {
        let hand_bid = HandBid::from(line);
        all_hands.push(hand_bid)
    }

    all_hands.sort_by(|a, b| {
        if a.hand_type > b.hand_type {
            Ordering::Greater
        } else if a.hand_type < b.hand_type {
            Ordering::Less
        } else {
            let b_cards: Vec<char> = b.hand.chars().collect();
            for (pos, ch) in a.hand.char_indices() {
                if card_ranking[&ch] > card_ranking[&b_cards[pos]] {
                    return Ordering::Greater;
                } else if card_ranking[&ch] < card_ranking[&b_cards[pos]] {
                    return Ordering::Less;
                }
            }
            Ordering::Equal
        }
    });

    println!("allHands: {:#?}", all_hands);

    let mut winnings = 0;
    for (pos, hand) in all_hands.iter().enumerate() {
        winnings += ((pos + 1) as u32) * hand.bid
    }

    println!("{winnings}")
}

// JJJJJ -> Fiveofkind -> AAAAA
// JJJJ1 -> Fiveofkind -> 11111
// JJJ11 -> Fiveofkind -> 11111
// JJJ12 -> Fourofkind -> 11112
// JJ111 -> Fiveofkind -> 11111
// JJ112 -> Fourofkind -> 11112
// JJ123 -> Threeofkind -> 11123
// J1111 -> Fiveofkind -> 11111
// J1112 -> Fourofkind -> 11112
// J1122 -> Fullhouse -> 11122
// J1123 -> Threeofkind -> 11123
// J1234 -> Onepair -> 11234
// 11223 -> Twopair
