use regex::Regex;

fn main() {
    let number_regex =
        Regex::new("zero|one|two|three|four|five|six|seven|eight|nine|0|1|2|3|4|5|6|7|8|9")
            .unwrap();
    let input = include_str!("input");
    let sum: usize = input
        .lines()
        .map(|line| {
            let mut matches = number_regex.find_iter(line);

            let first_number = matches.next().unwrap().as_str();

            let mut last_match = None;
            for i in 1..=line.len() {
                if let Some(offset) = number_regex.find_at(line, line.len() - i) {
                    last_match = Some(offset.as_str());
                    // last_match = Some(line.len() - i);
                    break;
                }
            }
            let last_number = last_match.unwrap();

            parse_digit_or_number_string(first_number) * 10
                + parse_digit_or_number_string(last_number)
        })
        .sum();

    println!("{sum}");
}

fn parse_digit_or_number_string(first_number: &str) -> usize {
    match first_number {
        "1" | "one" => 1,
        "2" | "two" => 2,
        "3" | "three" => 3,
        "4" | "four" => 4,
        "5" | "five" => 5,
        "6" | "six" => 6,
        "7" | "seven" => 7,
        "8" | "eight" => 8,
        "9" | "nine" => 9,
        unexpected => todo!("{unexpected} not handled yet"),
    }
}
