mod part_01;
mod part_01a;
mod part_02;
mod utils;

// https://adventofcode.com/2023

fn main() {
    let current: i32 = 3;

    match current {
        1 => part_01::part_01(),
        2 => part_01a::part_01a(),
        3 => part_02::part_02(),
        _ => {
            println!("unknown part {}", current);
        }
    }
}
