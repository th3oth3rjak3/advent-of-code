use std::str::Split;

fn main() {
    let day_info = include_str!("./input/day_info.txt");
    let input = include_str!("./input/part2.txt");
    let result = calculate(input);
    print!("{0}, part 2 result: '{1}'", day_info, result)
}

struct Game {
    draws: Vec<Draw>,
    power: usize,
}

impl Game {
    fn new() -> Self {
        Game {
            draws: Vec::new(),
            power: 0,
        }
    }

    fn with_draws(&self, draws: Vec<Draw>) -> Self {
        Game { draws, ..*self }
    }
}

struct Draw {
    red: usize,
    green: usize,
    blue: usize,
}

impl Draw {
    fn new() -> Self {
        Draw {
            red: 0,
            green: 0,
            blue: 0,
        }
    }

    fn with_red(&self, red: usize) -> Self {
        Draw { red, ..*self }
    }

    fn with_green(&self, green: usize) -> Self {
        Draw { green, ..*self }
    }

    fn with_blue(&self, blue: usize) -> Draw {
        Draw { blue, ..*self }
    }
}

fn calculate(input: &str) -> usize {
    input
        .lines()
        .map(|line| make_game(line))
        .map(|game| {
            let red = game.draws.iter().map(|draw| draw.red).max().unwrap();
            let green = game.draws.iter().map(|draw| draw.green).max().unwrap();
            let blue = game.draws.iter().map(|draw| draw.blue).max().unwrap();

            Game {
                power: red * green * blue,
                ..game
            }
        })
        .map(|game| game.power)
        .sum()
}

fn make_game(line: &str) -> Game {
    let draws = line
        .split(": ")
        .skip(1)
        .next()
        .map(|draws| draws.split("; ").map(|draw| make_draw(draw)))
        .unwrap()
        .collect();

    Game::new().with_draws(draws)
}

fn make_draw(line: &str) -> Draw {
    line.split(", ")
        .map(|number_and_color| number_and_color.split(' '))
        .fold(Draw::new(), |acc, mut split| parse_draw(acc, &mut split))
}

fn parse_draw<'a>(draw: Draw, split: &mut Split<'a, char>) -> Draw {
    let count = split.next().unwrap().parse::<usize>().unwrap();
    match split.next() {
        Some("red") => draw.with_red(count),
        Some("green") => draw.with_green(count),
        Some("blue") => draw.with_blue(count),
        _ => draw,
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn sample_input_should_be_correct() {
        let input = include_str!("./input/part2_sample_input.txt");
        let result = calculate(input);
        assert_eq!(result, 2286)
    }

    #[test]
    fn input_should_be_correct() {
        let input = include_str!("./input/part2.txt");
        let result = calculate(input);
        assert_eq!(result, 56580)
    }
}
