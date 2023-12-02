fn main() {
    let day_info = include_str!("./input/day_info.txt");
    let input = include_str!("./input/part1.txt");
    let result = calculate(input);
    print!("{0} part 1 result: '{1}'", day_info, result)
}

fn calculate(input: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn sample_input_should_be_correct() {
        let input = include_str!("./input/part1_sample_input.txt");
        let result = calculate(input);
        assert_eq!(result, 42);
    }
}
