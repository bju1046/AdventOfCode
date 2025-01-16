use crate::utils::read_lines;
struct GameColors {
    game_number: i32,
    red: i32,
    blue: i32,
    green: i32,
}

pub fn part_02() {
    let lines: Vec<String> =
        read_lines("/Users/brettudvardy/rust_hello/src/resources/02_input.dat");
    let max_game_colors = GameColors {
        game_number: 0,
        red: 12,
        blue: 14,
        green: 13,
    };
    let mut sum_of_impossible_games: i32 = 0;
    for line in lines {
        // println!("processing {}", line);
        let game_number: &str = &line[..line.find(':').unwrap()];
        // println!("game number: {}", game_number);

        let games = &line[line.find(':').unwrap() + 1..];
        // println!("games: {}", games);

        let games_split: std::str::Split<'_, &str> = games.split(";");
        let mut game_colors = GameColors {
            game_number: game_number.parse::<i32>().unwrap(),
            red: 0,
            blue: 0,
            green: 0,
        };
        for game in games_split {
            // println!("game: {}", game);
            let colors: std::str::Split<'_, &str> = game.split(",");

            for color_orig in colors {
                let color = color_orig.trim();
                let count: &str = &color[..color.trim().find(" ").unwrap()];
                let color: &str = &color[color.trim().find(" ").unwrap() + 1..];

                // println!("count: {}, color: {}", count, color);
                if color == "red" {
                    game_colors.red += count.parse::<i32>().unwrap();
                } else if color == "blue" {
                    game_colors.blue += count.parse::<i32>().unwrap();
                } else if color == "green" {
                    game_colors.green += count.parse::<i32>().unwrap();
                }
            }
        }
        // println!(
        //     "game number: {}, red: {}, blue: {}, green: {}",
        //     game_colors.game_number, game_colors.red, game_colors.blue, game_colors.green
        // );

        if max_game_colors.red < game_colors.red
            || max_game_colors.blue < game_colors.blue
            || max_game_colors.green < game_colors.green
        {
            println!(
                "game number: {}, red: {}, blue: {}, green: {}",
                game_colors.game_number, game_colors.red, game_colors.blue, game_colors.green
            );
            println!("game {} is impossible", game_colors.game_number);
        } else {
            sum_of_impossible_games += game_colors.game_number;
        }
    }
    println!("sum of impossible games: {}", sum_of_impossible_games);
    // not 4962
    // 88?
}
