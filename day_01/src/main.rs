const NUMWORDS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn evaluate_string(input: &String) -> Option<i32> {
    for (i, word) in NUMWORDS.iter().enumerate() {
        if input.contains(word) {
            return Some((i + 1) as i32);
        }
    }
    None
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let result: i32 = input
        .lines()
        .map(|line| {
            let mut calibration_value = 0;
            let mut prefix = String::new();
            let mut suffix = String::new();
            let mut chars = line.chars();
            while let Some(char) = chars.next() {
                if char.is_numeric() {
                    calibration_value = 10 * String::from(char).parse::<i32>().unwrap();
                    break;
                }
                prefix.push(char);
                if let Some(parsed_value) = evaluate_string(&prefix) {
                    calibration_value = 10 * parsed_value;
                    break;
                }
            }
            let mut chars = line.chars();
            while let Some(char) = chars.next_back() {
                if char.is_numeric() {
                    calibration_value += String::from(char).parse::<i32>().unwrap();
                    break;
                }
                suffix.insert(0, char);
                if let Some(parsed_value) = evaluate_string(&suffix) {
                    calibration_value += parsed_value;
                    break;
                }
            }
            calibration_value
        })
        .sum();

    println!("{result}");
    assert_eq!(result, 54019);
}
