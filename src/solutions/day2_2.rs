use std::fs;

pub fn execute_day_two_one() -> u32 {
    let filepath = "src/inputs/day-2-2";

    let content = fs::read_to_string(filepath).expect("Should read file");

    let red_cubes: u32 = 12;
    let green_cubes: u32 = 13;
    let blue_cubes: u32 = 14;

    content
        .lines()
        .map(|line| {
            let game_id_and_content_split: Vec<&str> = line.split(":").collect();
            let id: Vec<&str> = game_id_and_content_split[0].split_whitespace().collect();
            let id = id[1].parse::<u32>().expect("should be a number");
            let cubes: Vec<Vec<Vec<&str>>> = game_id_and_content_split[1]
                .split(";")
                .map(|item| item.split(","))
                .map(|item| {
                    item.map(|item_two| item_two.split_whitespace().collect::<Vec<&str>>())
                        .collect()
                })
                .collect();

            let is_valid = cubes
                .iter()
                .map(|item| {
                    item.into_iter()
                        .map(|an_item| {
                            let num = an_item[0].parse::<u32>().expect("should be number");
                            let colour = an_item[1];
                            if colour == "red" && num <= red_cubes {
                                return true;
                            }
                            if colour == "green" && num <= green_cubes {
                                return true;
                            }
                            if colour == "blue" && num <= blue_cubes {
                                return true;
                            }
                            return false;
                        })
                        .all(|item| item)
                })
                .all(|item| item);

            if is_valid {
                return id;
            }
            return 0;
        })
        .sum()
}

#[derive(Debug)]
struct CurrHighest {
    blue: u32,
    red: u32,
    green: u32,
}

pub fn execute_day_two_two() -> u32 {
    let filepath = "src/inputs/day-2-2";

    let content = fs::read_to_string(filepath).expect("Should read file");

    content
        .lines()
        .map(|line| {
            let game_id_and_content_split: Vec<&str> = line.split(":").collect();
            let id: Vec<&str> = game_id_and_content_split[0].split_whitespace().collect();
            let id = id[1].parse::<u32>().expect("should be a number");
            let cubes: Vec<Vec<Vec<&str>>> = game_id_and_content_split[1]
                .split(";")
                .map(|item| item.split(","))
                .map(|item| {
                    item.map(|item_two| item_two.split_whitespace().collect::<Vec<&str>>())
                        .collect()
                })
                .collect();

            let mut curr_highest = CurrHighest {
                blue: 1,
                red: 1,
                green: 1,
            };
            let is_valid: Vec<u32> = cubes
                .iter()
                .map(|item| {
                    let dongs = item.into_iter()
                        .map(|an_item| {
                            let num = an_item[0].parse::<u32>().expect("should be number");
                            let colour = an_item[1];
                            if colour == "red" && &num > &curr_highest.red {
                                curr_highest.red = num;
                            }
                            if colour == "green" && &num > &curr_highest.green {
                                curr_highest.green = num;
                            }
                            if colour == "blue" && &num > &curr_highest.blue {
                                curr_highest.blue = num;
                            }
                            return 0;
                        })
                        .collect::<Vec<u32>>();
                    return 0;
                })
                .collect();

            let sum = curr_highest.red * curr_highest.blue * curr_highest.green;
            return sum;
        })
        .sum()
}
