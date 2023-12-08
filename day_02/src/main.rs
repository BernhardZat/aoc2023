fn first_part(input: &String) -> i32 {
    input
        .lines()
        .map(|line| {
            let mut words = line.split_whitespace().skip(1);
            let game_id = String::from(words.next().unwrap().trim_end_matches(':'))
                .parse::<i32>()
                .unwrap();
            let mut value = 0;
            let mut is_valid = 1;
            while let Some(word) = words.next() {
                if let Ok(number) = String::from(word).parse::<i32>() {
                    value = number;
                } else {
                    let color = word.trim_end_matches(|c| c == ',' || c == ';');
                    match color {
                        "red" => {
                            if value > 12 {
                                is_valid = 0;
                            }
                        }
                        "green" => {
                            if value > 13 {
                                is_valid = 0;
                            }
                        }
                        "blue" => {
                            if value > 14 {
                                is_valid = 0;
                            }
                        }
                        _ => unreachable!(),
                    }
                }
            }
            game_id * is_valid
        })
        .sum()
}

fn second_part(input: &String) -> i32 {
    input
        .lines()
        .map(|line| {
            let mut red = vec![0];
            let mut green = vec![0];
            let mut blue = vec![0];
            let rounds = line.split(|char| char == ':' || char == ';').skip(1);
            for round in rounds {
                let mut words = round.split_whitespace();
                let mut value = 0;
                while let Some(word) = words.next() {
                    if let Ok(number) = String::from(word).parse::<i32>() {
                        value = number;
                    } else {
                        let color = word.trim_end_matches(',');
                        match color {
                            "red" => red.push(value),
                            "green" => green.push(value),
                            "blue" => blue.push(value),
                            _ => unreachable!(),
                        }
                    }
                }
            }
            red.iter().max().unwrap() * green.iter().max().unwrap() * blue.iter().max().unwrap()
        })
        .sum()
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let first_answer = first_part(&input);
    let second_answer = second_part(&input);
    assert_eq!(first_answer, 2727);
    assert_eq!(second_answer, 56580);
}
