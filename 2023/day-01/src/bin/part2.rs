fn main() {
    let day_info = include_str!("./input/day_info.txt");
    let input = include_str!("./input/part1.txt");
    let result = calculate(input);
    print!("{0}, part 2 result: '{1}'", day_info, result)
}

#[derive(Eq, PartialEq, Debug, Clone)]
struct Digit {
    index: usize,
    value: usize,
}

static NUMERIC_WORDS: [&'static str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn calculate(input: &str) -> usize {
    input
        .split("\n")
        .into_iter()
        .map(|line| {
            let first_digit =
                get_first_number(line).expect("It should have found the first digit.");
            let last_digit = get_last_number(line).expect("It should have found the last digit");
            format!("{}{}", first_digit, last_digit)
                .parse::<usize>()
                .expect("It should have been a number")
        })
        .into_iter()
        .sum()
}

fn get_first_number(line: &str) -> Option<usize> {
    let digit = get_first_digit(line);
    let numeric_word = get_first_numeric_word(line);

    return digit
        .and_then(|digit| match &numeric_word {
            Some(word) => {
                if word.index < digit.index {
                    return Some(word.value);
                } else {
                    return Some(digit.value);
                }
            }
            None => Some(digit.value),
        })
        .or_else(|| match &numeric_word {
            Some(word) => Some(word.value),
            None => None,
        });
}

fn get_first_digit(line: &str) -> Option<Digit> {
    for (i, ch) in line.chars().enumerate() {
        if ch.is_numeric() {
            return Some(Digit {
                index: i,
                value: ch.to_string().parse::<usize>().unwrap(),
            });
        }
    }

    None
}

fn get_first_numeric_word(line: &str) -> Option<Digit> {
    let mut digit: Option<Digit> = None;

    for (i, number) in NUMERIC_WORDS.iter().enumerate() {
        match line.find(number) {
            Some(idx) => match digit.clone() {
                Some(value) => {
                    if idx < value.index {
                        digit = Some(Digit {
                            index: idx,
                            value: i,
                        });
                    }
                }
                None => {
                    digit = Some(Digit {
                        index: idx,
                        value: i,
                    });
                }
            },
            None => (),
        }
    }
    digit
}

fn get_last_number(line: &str) -> Option<usize> {
    let digit = get_last_digit(line);
    let numeric_word = get_last_numeric_word(line);

    return digit
        .and_then(|digit| match &numeric_word {
            Some(word) => {
                if word.index > digit.index {
                    return Some(word.value);
                } else {
                    return Some(digit.value);
                }
            }
            None => Some(digit.value),
        })
        .or_else(|| match &numeric_word {
            Some(word) => Some(word.value),
            None => None,
        });
}

fn get_last_digit(line: &str) -> Option<Digit> {
    let mut digit: Option<Digit> = None;

    for (i, ch) in line.chars().enumerate() {
        if ch.is_numeric() {
            digit = Some(Digit {
                index: i,
                value: ch.to_string().parse::<usize>().unwrap(),
            })
        }
    }
    digit
}

fn get_last_numeric_word(line: &str) -> Option<Digit> {
    let mut digit: Option<Digit> = None;

    for (i, number) in NUMERIC_WORDS.iter().enumerate() {
        // rfind finds the rightmost instance of the word.
        match line.rfind(number) {
            Some(idx) => match digit {
                Some(value) => {
                    if value.index > idx {
                        digit = Some(value);
                    } else {
                        digit = Some(Digit {
                            index: idx,
                            value: i,
                        });
                    }
                }
                None => {
                    digit = Some(Digit {
                        index: idx,
                        value: i,
                    });
                }
            },
            None => (),
        }
    }

    digit
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn sample_input_should_be_correct() {
        let input = include_str!("./input/part2_sample_input.txt");
        let result = calculate(input);
        assert_eq!(result, 281)
    }

    #[test]
    fn get_first_numeric_word_finds_words() {
        let expected = Some(Digit { index: 2, value: 1 });
        let actual = get_first_numeric_word("a one at position two");
        assert_eq!(expected, actual);
    }

    #[test]
    fn get_first_numeric_word_is_none() {
        let expected = None;
        let actual = get_first_numeric_word("an empty string with no digits");
        assert_eq!(expected, actual);
    }

    #[test]
    fn get_first_digit_finds_digit() {
        let expected = Some(Digit { index: 4, value: 9 });
        let actual = get_first_digit("the 9 is at position 4");
        assert_eq!(expected, actual);
    }

    #[test]
    fn get_first_digit_does_not_find_digit() {
        let expected = None;
        let actual = get_first_digit("there are no digits here");
        assert_eq!(expected, actual);
    }

    #[test]
    fn get_first_number_finds_digit() {
        let expected = Some(1);
        let actual = get_first_number("1 is not two");
        assert_eq!(expected, actual);
    }

    #[test]
    fn get_first_number_finds_word() {
        let expected = Some(1);
        let actual = get_first_number("one is not 2");
        assert_eq!(expected, actual);
    }

    #[test]
    fn get_last_digit_finds_number() {
        let expected = Some(Digit {
            index: 10,
            value: 0,
        });
        let actual = get_last_digit("the value 0 is at index ten");
        assert_eq!(expected, actual);
    }

    #[test]
    fn get_last_numeric_word_should_find_one() {
        let expected = Some(Digit {
            index: 25,
            value: 5,
        });
        let actual = get_last_numeric_word("the value is not one but fiver");
        assert_eq!(expected, actual);
    }

    #[test]
    fn get_last_numeric_word_should_not_find_one() {
        let expected = None;
        let actual = get_last_numeric_word("the value is not found");
        assert_eq!(expected, actual);
    }

    #[test]
    fn get_last_number_finds_digit() {
        let expected = Some(8);
        let actual = get_last_number("the last one should be 8");
        assert_eq!(expected, actual);
    }

    #[test]
    fn get_last_number_finds_word() {
        let expected = Some(8);
        let actual = get_last_number("it shouldn't find 9 but eighty");
        assert_eq!(expected, actual);
    }

    #[test]
    fn get_last_number_finds_single_word() {
        let expected = Some(8);
        let actual = get_last_number("it should find eighty");
        assert_eq!(expected, actual);
    }

    #[test]
    fn get_last_number_finds_single_digit() {
        let expected = Some(9);
        let actual = get_last_number("it should find 9");
        assert_eq!(expected, actual);
    }

    #[test]
    fn get_last_number_finds_none() {
        let expected = None;
        let actual = get_last_number("it shouldn't find anything");
        assert_eq!(expected, actual);
    }

    #[test]
    fn calculate_should_succeed_0001() {
        let expected = 14;
        let actual = calculate("1six7396484");
        assert_eq!(expected, actual);
    }

    #[test]
    fn calculate_should_succeed_0002() {
        let expected = 19;
        let actual = calculate("1ninehgqtjprgnpkchxdkctzk");
        assert_eq!(expected, actual);
    }

    #[test]
    fn calculate_should_sum_correctly() {
        let expected = 19 + 14;
        let actual = calculate(
            "1six7396484
        1ninehgqtjprgnpkchxdkctzk",
        );
        assert_eq!(expected, actual);
    }

    #[test]
    fn calculate_should_succeed_0005() {
        let expected = 99;
        let actual = calculate("9eightctkdnnllnine");
        assert_eq!(expected, actual);
    }

    #[test]
    fn calculate_should_succeed_0017() {
        let expected = 88;
        let actual = calculate("m8t");
        assert_eq!(expected, actual);
    }

    #[test]
    fn calculate_should_succeed_0021() {
        let expected = 65;
        let actual = calculate("bctthnksix4onegmgpjbbxqqrmk5hnfvzcnsgh");
        assert_eq!(expected, actual);
    }

    #[test]
    fn calculate_should_succeed_0347() {
        let expected = 19;
        let actual = calculate("kdkoneightjstgn9");
        assert_eq!(expected, actual);
    }

    #[test]
    fn calculate_should_succeed_0348() {
        let expected = 73;
        let actual = calculate("7bbfsgl6sixdchgpnine3");
        assert_eq!(expected, actual);
    }

    #[test]
    fn calculate_should_succeed_0983() {
        let expected = 44;
        let actual = calculate("vfh4zteightkvbpps4rxhlnctjztjfvdvdxfk");
        assert_eq!(expected, actual);
    }

    #[test]
    fn calculate_should_succeed_0997() {
        let expected = 11;
        let actual = calculate("1dgschj");
        assert_eq!(expected, actual);
    }

    #[test]
    fn calculate_should_succeed_1000() {
        let expected = 71;
        let actual = calculate("seven443six8three31");
        assert_eq!(expected, actual);
    }

    #[test]
    fn calculate_should_find_digits_both_ends() {
        let expected = 11;
        let actual = calculate("onetwothreefourone");
        assert_eq!(expected, actual);
    }
}
