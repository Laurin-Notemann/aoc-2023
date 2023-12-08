use std::fs;

pub fn execute_day_one_one() -> u32 {
    let filepath = "src/inputs/day-1";

    let content = fs::read_to_string(filepath).expect("Should read file");

    content
        .lines()
        .map(|line| {
            let number: Vec<u32> = line
                .chars()
                .filter(|item| item.is_digit(10))
                .map(|item| item.to_digit(10).expect("Should be a number"))
                .collect();

            println!("{:?}", number);
            let temp_str = format!(
                "{}{}",
                number.first().expect("a number"),
                number.last().expect("a number")
            );

            temp_str.parse::<u32>().expect("should be a valid number")
        })
        .sum()
}

pub fn execute_day_one_two() -> u32 {
    let filepath = "src/inputs/day-1-1";

    let content = fs::read_to_string(filepath).expect("Should read file");

    content
        .lines()
        .map(|line| {
            let temp_strr = line;
            let temp_strr = temp_strr.replace("one", "o1e");
            let temp_strr = temp_strr.replace("two", "t2o");
            let temp_strr = temp_strr.replace("three", "t3e");
            let temp_strr = temp_strr.replace("four", "f4r");
            let temp_strr = temp_strr.replace("five", "f5e");
            let temp_strr = temp_strr.replace("six", "s6x");
            let temp_strr = temp_strr.replace("seven", "s7n");
            let temp_strr = temp_strr.replace("eight", "e8t");
            let temp_strr = temp_strr.replace("nine", "n9e");
            let number: Vec<u32> = temp_strr
                .chars()
                .filter(|item| item.is_digit(10))
                .map(|item| item.to_digit(10).expect("Should be a number"))
                .collect();

            println!("{:?}", number);
            let temp_str = format!(
                "{}{}",
                number.first().expect("a number"),
                number.last().expect("a number")
            );

            temp_str.parse::<u32>().expect("should be a valid number")
        })
        .sum()
}
