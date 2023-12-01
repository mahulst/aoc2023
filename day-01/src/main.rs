use std::collections::HashMap;

fn main() {
    let answer_1 = part_1(include_str!("./input.txt"));
    let answer_2 = part_2(include_str!("./input.txt"));

    print!("part_1 {}", answer_1);
    print!("part_2 {}", answer_2);
}

fn part_1(input: &str) -> usize {
    input
        .lines()
        .into_iter()
        .map(|line| {
            line.chars()
                .filter_map(|c| c.to_digit(10))
                .collect::<Vec<u32>>()
        })
        .map(|nums| {
            let min = nums.first().unwrap();
            let max = nums.last().unwrap();
            let joined = format!("{}{}", min, max);
            joined.parse::<usize>().unwrap()
        })
        .sum()
}
fn line_mapper(line: &str) -> usize {
    let matches: HashMap<&str, usize> = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
    ]);

    let mut string_matches = matches
        .iter()
        .filter_map(|(key, val)| line.find(key).map(|index| (index, *val)))
        .collect::<Vec<(usize, usize)>>();
    string_matches.sort_by(|(index, _), (index2, _)| index.cmp(index2));

    let mut last_string_matches = matches
        .iter()
        .filter_map(|(key, val)| line.rfind(key).map(|index| (index, *val)))
        .collect::<Vec<(usize, usize)>>();
    last_string_matches.sort_by(|(index, _), (index2, _)| index.cmp(index2));

    let min = string_matches.first().map(|(_, i)| i).unwrap();
    let max = last_string_matches.last().map(|(_, i)| i).unwrap();
    let joined = format!("{}{}", min, max);

    joined.parse::<usize>().unwrap()
}
fn part_2(input: &str) -> usize {
    input.lines().into_iter().map(line_mapper).sum()
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_day_1() {
        let result = part_1(
            "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet",
        );
        assert_eq!(result, 142);
    }
    #[test]
    fn test_day_2() {
        let result = part_2(
            "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen",
        );
        assert_eq!(result, 281);
    }
}
