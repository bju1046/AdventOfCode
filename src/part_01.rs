use crate::utils::read_lines;

pub fn part_01() {
    let lines: Vec<String> = read_lines("/Users/brettudvardy/rust_hello/src/resources/01_input");

    let mut line_calibration: Vec<i32> = Vec::new();
    for line in lines {
        println!("processing {}", line);
        let mut line_chars: String = String::from("");

        let mut first_char = '_';
        for c in line.chars() {
            if c.is_digit(10) {
                first_char = c;
                line_chars.push(c);
                break;
            }
        }
        let first_match: Option<usize> = line.find(first_char);
        let slice = &line[first_match.unwrap()..];
        let reversed = slice.chars().rev().collect::<String>();

        for c in reversed.chars() {
            if c.is_digit(10) {
                // last_char = c;
                line_chars.push(c);
                break;
            }
        }

        println!("adding {} to line_calibration", line_chars);
        line_calibration.push(line_chars.parse::<i32>().unwrap());
    }

    println!("sum: {}", line_calibration.iter().sum::<i32>());
}
