use crate::utils::read_lines;

fn string_to_stringnumber(input: &str) -> char {
    let result: char = match input {
        "one" => '1',
        "1" => '1',
        "two" => '2',
        "2" => '2',
        "three" => '3',
        "3" => '3',
        "four" => '4',
        "4" => '4',
        "five" => '5',
        "5" => '5',
        "six" => '6',
        "6" => '6',
        "seven" => '7',
        "7" => '7',
        "eight" => '8',
        "8" => '8',
        "nine" => '9',
        "9" => '9',
        _ => '0',
    };
    result
}

struct PositionNumber {
    position: i32,
    number_string: char,
}

pub fn part_01a() {
    // let lines: Vec<String> = read_lines("/Users/brettudvardy/rust_hello/src/resources/single_test");
    let lines: Vec<String> = read_lines("/Users/brettudvardy/rust_hello/src/resources/01_input");
    let named_numbers: [&str; 18] = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "1", "2", "3", "4",
        "5", "6", "7", "8", "9",
    ];
    let mut line_calibration: Vec<i32> = Vec::new();
    for line in lines {
        // print!("processing {}\t", line);
        let mut found: Vec<PositionNumber> = Vec::new();
        for number in named_numbers {
            let position: Option<usize> = line.find(number);
            if position != None {
                println!("found {} in position {}", number, position.unwrap());
                let position_number = PositionNumber {
                    position: position.unwrap() as i32,
                    number_string: string_to_stringnumber(number),
                };
                found.push(position_number);
            }
            let position2: Option<usize> = line.rfind(number);
            if position2 != None {
                println!("found {} in position {}", number, position2.unwrap());
                let position_number = PositionNumber {
                    position: position2.unwrap() as i32,
                    number_string: string_to_stringnumber(number),
                };
                found.push(position_number);
            }
        }

        found.sort_by(|a, b| a.position.cmp(&b.position));
        let mut string_result = String::from("");
        if found.len() > 0 {
            let last = found.last();
            // let actual_last = last.unwrap();

            string_result.push(found[0].number_string);
            string_result.push(last.unwrap().number_string);
        } else {
            println!("\t\tno number found in {}", line);
        }

        println!("result: {} \t {}", string_result, line);
        if string_result != "" {
            line_calibration.push(string_result.parse::<i32>().unwrap());
        }
    }
    println!("sum: {}", line_calibration.iter().sum::<i32>());
}
