fn main() {
    let day_info = include_str!("./input/day_info.txt");
    let input = include_str!("./input/part1.txt");
    let result = calculate(input);
    print!("{0}, part 1 result: '{1}'", day_info, result)
}

fn calculate(input: &str) -> usize {
    input
        .split("\n")
        .into_iter()
        .map(|line| {
            let first_digit = line
                .chars()
                .find(|character| character.is_numeric())
                .expect("There should be at least one digit.");
            let last_digit = line
                .chars()
                .rev()
                .find(|character| character.is_numeric())
                .expect("There should be at least one digit.");
            format!("{}{}", first_digit, last_digit)
        })
        .into_iter()
        .map(|value| value.parse::<usize>().unwrap_or_else(|_| 0))
        .sum()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn sample_input_should_be_correct() {
        let input = include_str!("./input/part1_sample_input.txt");
        let result = calculate(input);
        assert_eq!(result, 142)
    }
}
