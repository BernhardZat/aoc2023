fn get_check_positions(line_len: i32, input_len: i32, num_pos: i32, num_len: i32) -> Vec<i32> {
    let mut positions = Vec::new();
    positions.push(num_pos - 1);
    positions.push(num_pos + num_len);
    for pos in num_pos - line_len - 1..num_pos - line_len + num_len + 1 {
        positions.push(pos);
    }
    for pos in num_pos + line_len - 1..num_pos + line_len + num_len + 1 {
        positions.push(pos);
    }
    positions.retain(|pos| pos >= &0 && pos < &input_len);
    positions
}

fn first_part(input: &String) -> i32 {
    let mut all_numbers = Vec::new();
    let mut valid_numbers = Vec::new();
    let mut num_str = String::new();
    let mut num_pos = input.find(|c: char| c.is_numeric()).unwrap();
    let mut old_pos = num_pos - 1;
    let mut num_chars = input.char_indices().filter(|(_, c)| c.is_numeric());
    while let Some((pos, char)) = num_chars.next() {
        if pos == old_pos + 1 {
            num_str.push(char);
        } else {
            all_numbers.push((num_pos, num_str.clone()));
            num_str.clear();
            num_str.push(char);
            num_pos = pos;
        }
        old_pos = pos;
    }
    for (pos, numstr) in all_numbers {
        let positions = get_check_positions(
            (input.lines().next().unwrap().len() + 1) as i32,
            input.len() as i32,
            pos as i32,
            numstr.len() as i32,
        );
        for pos in positions {
            let char = input.chars().nth(pos as usize).unwrap();
            if !(char == '.' || char.is_numeric() || char == '\n') {
                valid_numbers.push(numstr.parse::<i32>().unwrap());
                break;
            }
        }
    }
    valid_numbers.iter().sum()
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let first_answer = first_part(&input);
    assert_eq!(first_answer, 527144);
}
