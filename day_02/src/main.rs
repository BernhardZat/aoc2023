fn main() {
    let input = std::fs::read_to_string("input").unwrap();

    let result: i32 = input
        .lines()
        .map(|line| {
            let mut words = line.split_whitespace().skip(1);
            let game_id = String::from(words.next().unwrap().trim_end_matches(':'))
                .parse::<i32>()
                .unwrap();
            let mut current_value = 0;
            let mut is_valid_game = 1;
            while let Some(word) = words.next() {
                if let Ok(number) = String::from(word).parse::<i32>() {
                    current_value = number;
                } else {
                    let color = word
                        .trim_end_matches(|char| match char {
                            ',' => true,
                            ';' => true,
                            _ => false,
                        });
                    match color {
                        "red" => {
                            if current_value > 12 {
                                is_valid_game = 0;
                            }
                        }
                        "green" => {
                            if current_value > 13 {
                                is_valid_game = 0;
                            }
                        }
                        "blue" => {
                            if current_value > 14 {
                                is_valid_game = 0;
                            }
                        }
                        _ => unreachable!(),
                    }
                }
            }
            game_id * is_valid_game
        })
        .sum();
    println!("{result}")
}
