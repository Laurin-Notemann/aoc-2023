use std::fs::{self};
use std::ops::Mul;

#[derive(Debug)]
struct Number {
    value: u32,
    idx: usize,
}

pub fn execute_day_three_one() -> u32 {
    let filepath = "src/inputs/day-3";

    let content = fs::read_to_string(filepath).expect("Should read file");

    let vec_con: Vec<Vec<char>> = content.lines().map(|item| item.chars().collect()).collect();

    let mut all_nums: Vec<u32> = vec![];
    for column_idx in 0..vec_con.len() {
        //let mut number_row_idx: Vec<u32> = vec![];
        let mut found_numbers_temp_idx: Vec<usize> = vec![];
        for row_idx in 0..vec_con[column_idx].len() {
            let mut numbers_found: Vec<Number> = vec![];
            if vec_con[column_idx][row_idx].is_digit(10)
                && !found_numbers_temp_idx.contains(&row_idx)
            {
                let mut temp_row_idx = row_idx;
                while vec_con[column_idx][temp_row_idx].is_digit(10) {
                    numbers_found.push(Number {
                        value: vec_con[column_idx][temp_row_idx]
                            .to_digit(10)
                            .expect("should be number"),
                        idx: temp_row_idx,
                    });
                    found_numbers_temp_idx.push(temp_row_idx);
                    temp_row_idx += 1;
                    if temp_row_idx > vec_con[0].len() - 1 {
                        break;
                    }
                }

                let mut has_symbol_nearby = false;
                for num in &numbers_found {
                    for i in 0..=2 {
                        for j in 0..=2 {
                            let actual_row: isize = num.idx as isize + i - 1;
                            let actual_col: isize = column_idx as isize + j - 1;

                            if actual_col < 0 || actual_row < 0 {
                                continue;
                            }

                            let actual_row = actual_row.try_into().expect("should be above 0");
                            let actual_col = actual_col.try_into().expect("should be above 0");

                            if actual_row == num.idx && actual_col == column_idx {
                                continue;
                            }
                            if is_not_digit_or_period(&vec_con, actual_row, actual_col) {
                                has_symbol_nearby = true;
                                break;
                            }
                        }
                    }
                    if has_symbol_nearby {
                        break;
                    }
                }

                if has_symbol_nearby {
                    let mut string_num = String::from("");
                    for num in &numbers_found {
                        string_num = format!("{}{}", string_num, num.value)
                    }

                    let parsed_num = string_num.parse::<u32>().expect("should be number");
                    all_nums.push(parsed_num)
                }
            };
            if !found_numbers_temp_idx.is_empty() {
                found_numbers_temp_idx.remove(0);
            }
        }
    }
    let sum: u32 = all_nums.iter().sum();
    return sum;
}

fn is_not_digit_or_period(vec_con: &Vec<Vec<char>>, row: usize, col: usize) -> bool {
    if row >= vec_con.len() || col >= vec_con[row].len() {
        return false;
    }
    !vec_con[col][row].is_digit(10) && vec_con[col][row] != '.'
}

pub fn day_three_two() -> u32 {
    let filepath = "src/inputs/day-3";

    let content = fs::read_to_string(filepath).expect("Should read file");

    let two_d_array: Vec<Vec<char>> = content.lines().map(|item| item.chars().collect()).collect();

    let something = two_d_array
        .iter()
        .enumerate()
        .map(|(col_idx, col_el)| {
            col_el
                .iter()
                .enumerate()
                .map(|(row_idx, row_el)| {
                    let mut product: u32 = 0;
                    if row_el == &'*' {
                        let mut digit_one: u32 = 0;
                        let mut digit_two: u32 = 0;
                        for i in 0..=2 {
                            for j in 0..=2 {
                                let row_to_test: isize = row_idx as isize + j - 1;
                                let col_to_test: isize = col_idx as isize + i - 1;

                                if col_to_test < 0 || row_to_test < 0 {
                                    continue;
                                }

                                let col_to_test: usize =
                                    col_to_test.try_into().expect("should be above 0");
                                let row_to_test: usize =
                                    row_to_test.try_into().expect("should be above 0");

                                if col_to_test == col_idx && row_to_test == row_idx {
                                    continue;
                                }

                                if two_d_array[col_to_test][row_to_test].is_digit(10) {
                                    let mut curr_row_to_left = row_to_test;

                                    let mut left_side_digits: Vec<char> = vec![];
                                    while two_d_array[col_to_test][curr_row_to_left].is_digit(10) {
                                        if curr_row_to_left > 0 {
                                            curr_row_to_left -= 1;
                                        } else {
                                            break;
                                        }
                                        if two_d_array[col_to_test][curr_row_to_left].is_digit(10) {
                                            left_side_digits
                                                .push(two_d_array[col_to_test][curr_row_to_left]);
                                        }
                                    }
                                    let mut curr_row_to_right = row_to_test;

                                    let mut right_side_digits: Vec<char> = vec![];
                                    while two_d_array[col_to_test][curr_row_to_right].is_digit(10) {
                                        if curr_row_to_right < two_d_array[col_idx].len() - 1 {
                                            curr_row_to_right += 1;
                                        } else {
                                            break;
                                        }
                                        if two_d_array[col_to_test][curr_row_to_right].is_digit(10)
                                        {
                                            right_side_digits
                                                .push(two_d_array[col_to_test][curr_row_to_right]);
                                        }
                                    }

                                    let mut number_str: String =
                                        format!("{}", two_d_array[col_to_test][row_to_test]);

                                    for digit in left_side_digits {
                                        number_str = format!("{}{}", digit, number_str)
                                    }
                                    for digit in right_side_digits {
                                        number_str = format!("{}{}", number_str, digit)
                                    }

                                    if digit_one == 0 {
                                        digit_one = number_str.parse().expect("Should be a number")
                                    } else if digit_two == 0 {
                                        digit_two = number_str.parse().expect("Should be a number")
                                    }
                                    if two_d_array[col_idx + 1][row_idx] == '.'
                                        && two_d_array[col_idx - 1][row_idx] == '.'
                                    {
                                        continue;
                                    } else {
                                        break;
                                    }
                                }
                            }
                        }

                        if digit_one > 0 && digit_two > 0 {
                            product = digit_one.mul(digit_two);
                        }
                    }
                    return product;
                })
                .sum::<u32>()
        })
        .collect::<Vec<u32>>();

    return something.iter().sum();
}
