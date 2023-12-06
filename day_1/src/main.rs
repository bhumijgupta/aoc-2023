// use rand::Rng;
// use std::cmp::Ordering;
use std::fs;
// use std::io;

fn main() {
    let file_content = fs::read_to_string("./assets/input4.txt").expect("error in reading file");

    let mut res = 0;
    for line in file_content.lines() {
        let mut first_num = -1;
        let mut second_num = -1;
        // println!("{line}");

        let mut index = 0;

        for dig in line.chars() {
            let mut num_extracted = -1;
            if dig >= '0' && dig <= '9' {
                num_extracted = dig as i32 - 0x30;
                // println!("f:{first_num}, s:{second_num}")
            } else {
                let substr = line.get(index..).unwrap();
                if substr.starts_with("one") {
                    num_extracted = 1;
                } else if substr.starts_with("two") {
                    num_extracted = 2;
                } else if substr.starts_with("three") {
                    num_extracted = 3;
                } else if substr.starts_with("four") {
                    num_extracted = 4;
                } else if substr.starts_with("five") {
                    num_extracted = 5;
                } else if substr.starts_with("six") {
                    num_extracted = 6;
                } else if substr.starts_with("seven") {
                    num_extracted = 7;
                } else if substr.starts_with("eight") {
                    num_extracted = 8;
                } else if substr.starts_with("nine") {
                    num_extracted = 9;
                } else if substr.starts_with("zero") {
                    num_extracted = 0;
                }
            }

            if num_extracted != -1 {
                if first_num == -1 {
                    first_num = num_extracted;
                }
                second_num = num_extracted;
            }

            index = index + 1;
        }
        res += first_num * 10 + second_num;
    }
    println!("{}", res)
}

// fn tutorial() {
//     println!("Guess the number");
//     let secret_number = rand::thread_rng().gen_range(1..=100);

//     loop {
//         println!("Enter your guess");

//         let mut guess = String::new();

//         io::stdin()
//             .read_line(&mut guess)
//             .expect("Failed to read line");

//         let guess: u32 = match guess.trim().parse() {
//             Ok(num) => num,
//             Err(_) => continue,
//         };

//         println!("Your guess: {guess}");

//         match guess.cmp(&secret_number) {
//             Ordering::Less => println!("Too small!"),
//             Ordering::Greater => println!("Too big!"),
//             Ordering::Equal => {
//                 println!("You win!");
//                 break;
//             }
//         }
//     }
// }
