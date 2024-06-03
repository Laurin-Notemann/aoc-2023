use std::fs;

fn day_3_input() -> Vec<Vec<Vec<u32>>> {
    let filepath = "src/inputs/day-4";

    let content = fs::read_to_string(filepath).expect("Should read file");

    content
        .lines()
        .map(|line| line.split(":").collect::<Vec<&str>>()[1])
        .map(|line| line.split("|").collect::<Vec<&str>>())
        .map(|lines| {
            lines
                .into_iter()
                .map(|line| {
                    line.split_whitespace()
                        .map(|str_num| str_num.parse::<u32>().expect("Should be a number"))
                        .collect::<Vec<u32>>()
                })
                .collect::<Vec<Vec<u32>>>()
        })
        .collect::<Vec<Vec<Vec<u32>>>>()
}

pub fn execute_day_four_one() -> u32 {
    let vec_con = day_3_input();

    vec_con
        .iter()
        .map(|line| {
            let winning_num = &line[0];
            let personal_num = &line[1];

            let matching_nums = find_matching_nums(winning_num, personal_num);

            let two: usize = 2;
            let vec_len = matching_nums.len() as isize - 1;
            if vec_len > -1 as isize {
                let pow = two.pow(vec_len as u32);
                println!("{}", pow);
                return pow;
            } else {
                return 0;
            }
        })
        .sum::<usize>() as u32
}

#[derive(Debug)]
struct CardAmounts {
    amount: u32,
}

pub fn execute_day_four_two() -> u32 {
    let content = day_3_input();

    let mut card_amounts: Vec<CardAmounts> = vec![];

    for _ in 1..=content.len() {
        card_amounts.push(CardAmounts {
            amount: 1,
        })
    }

    let mut empty_row_found = false;

    let sum = content
        .iter()
        .enumerate()
        .map(|(line_num, line)| {
            let winning_num = &line[0];
            let personal_num = &line[1];

            let mut all_match_sums: Vec<Vec<&u32>> = vec![];
            for _ in 0..card_amounts[line_num].amount {
                all_match_sums.push(find_matching_nums(winning_num, personal_num));
            }

            if card_amounts[line_num].amount < 2 && all_match_sums.len() == 0 || empty_row_found {
                empty_row_found = true;
                return 0;
            }

            for matching_nums in all_match_sums {
                for i in 1..=matching_nums.len() {
                    let temp_card_idx = line_num + i;
                    card_amounts[temp_card_idx].amount += 1;
                }
            }

            return card_amounts[line_num].amount;
        })
        .sum::<u32>();

    return sum;
}

fn find_matching_nums<'a>(winning_num: &'a Vec<u32>, personal_num: &'a Vec<u32>) -> Vec<&'a u32> {
    personal_num
        .into_iter()
        .filter(|number| winning_num.contains(*number))
        .collect::<Vec<&u32>>()
}
