use std::fs;

fn day_5_input() -> Vec<Vec<Vec<u64>>> {
    let filepath = "src/inputs/day-5";

    let content = fs::read_to_string(filepath).expect("Should read file");

    content
        .split("\n\n")
        .map(|block| {
            block
                .split(":")
                .map(|line| line.trim())
                .collect::<Vec<&str>>()[1]
                .lines()
                .map(|line| {
                    line.trim()
                        .split_ascii_whitespace()
                        .map(|str_num| str_num.parse::<u64>().expect("should be numb lol"))
                        .collect::<Vec<u64>>()
                })
                .collect::<Vec<Vec<u64>>>()
        })
        .collect()
}

pub fn execute_day_five_one() -> u64 {
    let content = day_5_input();
    let seeds = &content[0][0];
    let length_content = &content.len();

    let somedongs = seeds
        .iter()
        .enumerate()
        .map(|(seed_idx, seed)| {
            let mut curr_result = 0;
            content
                .iter()
                .enumerate()
                .map(move |(map_idx, map)| {
                    let ergebnis = 0;
                    if map.len() != 1 {
                        let mut has_been_found = false;
                        if map_idx == 1 {
                            let correct_num = map
                                .iter()
                                .enumerate()
                                .map(|(line_idx, line)| {
                                    if *seed > line[1] + line[2] - 1 || seed < &line[1] {
                                        return 0;
                                    } else if line_idx == map.len() - 1 && !has_been_found {
                                        return *seed;
                                    } else {
                                        has_been_found = true;
                                        return line[0] + (*seed - line[1]);
                                    }
                                })
                                .sum::<u64>();

                            curr_result = correct_num;
                        } else {
                            has_been_found = false;
                            let correct_num = map
                                .iter()
                                .enumerate()
                                .map(|(line_idx, line)| {
                                    if line_idx == map.len() - 1 && !has_been_found {
                                        return curr_result;
                                    } else if curr_result > line[1] + line[2] - 1
                                        || curr_result < line[1]
                                    {
                                        return 0;
                                    } else {
                                        has_been_found = true;
                                        return line[0] + (curr_result - line[1]);
                                    }
                                })
                                .sum::<u64>();

                            curr_result = correct_num;
                        }

                        if map_idx == *length_content - 1 {
                            return curr_result;
                        }

                        return ergebnis;
                    }
                    return ergebnis;
                })
                .filter(|item| item != &0)
                .collect::<Vec<u64>>()
        })
        .flatten()
        .min()
        .expect("should ben umber");

    println!("{:?}", somedongs);
    return somedongs;
}

pub fn execute_day_five_two() -> u64 {
    let content = day_5_input();
    let seeds: Vec<Vec<u64>> = vec![vec![]];
    let seeds = &content[0][0].iter().f;
    let length_content = &content.len();

    let somedongs = seeds
        .iter()
        .enumerate()
        .map(|(seed_idx, seed)| {
            let mut curr_result = 0;
            content
                .iter()
                .enumerate()
                .map(move |(map_idx, map)| {
                    let ergebnis = 0;
                    if map.len() != 1 {
                        let mut has_been_found = false;
                        if map_idx == 1 {
                            let correct_num = map
                                .iter()
                                .enumerate()
                                .map(|(line_idx, line)| {
                                    if *seed > line[1] + line[2] - 1 || seed < &line[1] {
                                        return 0;
                                    } else if line_idx == map.len() - 1 && !has_been_found {
                                        return *seed;
                                    } else {
                                        has_been_found = true;
                                        return line[0] + (*seed - line[1]);
                                    }
                                })
                                .sum::<u64>();

                            curr_result = correct_num;
                        } else {
                            has_been_found = false;
                            let correct_num = map
                                .iter()
                                .enumerate()
                                .map(|(line_idx, line)| {
                                    if line_idx == map.len() - 1 && !has_been_found {
                                        return curr_result;
                                    } else if curr_result > line[1] + line[2] - 1
                                        || curr_result < line[1]
                                    {
                                        return 0;
                                    } else {
                                        has_been_found = true;
                                        return line[0] + (curr_result - line[1]);
                                    }
                                })
                                .sum::<u64>();

                            curr_result = correct_num;
                        }

                        if map_idx == *length_content - 1 {
                            return curr_result;
                        }

                        return ergebnis;
                    }
                    return ergebnis;
                })
                .filter(|item| item != &0)
                .collect::<Vec<u64>>()
        })
        .flatten()
        .min()
        .expect("should ben umber");

    println!("{:?}", somedongs);
    return somedongs;
}
