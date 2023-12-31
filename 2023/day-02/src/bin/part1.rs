use std::str::Split;

fn main() {
    let day_info = include_str!("./input/day_info.txt");
    let input = include_str!("./input/part1.txt");
    let result = calculate(input);
    print!("{0}, part 1 result: '{1}'", day_info, result)
}

struct Game {
    id: usize,
    draws: Vec<Draw>,
}

impl Game {
    fn new() -> Self {
        Game {
            id: 0,
            draws: Vec::new(),
        }
    }

    fn with_id(&self, id: usize) -> Self {
        Game {
            id,
            draws: self.draws.clone(),
        }
    }

    fn with_draws(&self, draws: Vec<Draw>) -> Self {
        Game { draws, ..*self }
    }
}

#[derive(Clone, Copy)]
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
        .filter(|game| {
            game.draws
                .iter()
                .all(|draw| draw.red <= 12 && draw.green <= 13 && draw.blue <= 14)
        })
        .map(|game| game.id)
        .sum()
}

fn make_game(line: &str) -> Game {
    let mut parts = line.split(": ");

    let id = parts
        .next()
        .unwrap()
        .strip_prefix("Game ")
        .unwrap()
        .parse::<usize>()
        .unwrap();

    let draws = parts
        .next()
        .map(|draws| draws.split("; ").map(|draw| make_draw(draw)))
        .unwrap()
        .collect();

    Game::new().with_id(id).with_draws(draws)
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
        let input = include_str!("./input/part1_sample_input.txt");
        let result = calculate(input);
        assert_eq!(result, 8)
    }

    #[test]
    fn input_should_be_correct() {
        let input = include_str!("./input/part1.txt");
        let result = calculate(input);
        assert_eq!(result, 2727)
    }
}
