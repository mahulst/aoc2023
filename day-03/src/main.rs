use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::{self, tag, take_while},
    character::{
        self,
        complete::{alpha1, char, digit1, line_ending},
    },
    multi::{many0, many1, separated_list1},
    sequence::{delimited, separated_pair, terminated},
    IResult,
};

fn main() {
    let answer_1 = part_1(include_str!("./input.txt"));
    let answer_2 = part_2(include_str!("./input.txt"));

    println!("part_1 {}", answer_1);
    println!("part_2 {}", answer_2);
}

fn part_1(input: &str) -> usize {
    let mut symbols: HashMap<(isize, isize), Symbols> = HashMap::new();
    let mut parts: HashMap<(isize, isize), usize> = HashMap::new();
    parse(input, &mut parts, &mut symbols);
    parts
        .iter()
        .filter(|((x, y), num)| {
            let len = num.to_string().len() as isize;
            println!("running {}", num);
            for i in x - 1..x + len + 1 {
                for j in y - 1..y + 1 {
                    println!("checking {},{}", i, j);
                    if symbols.get(&(i, j)).is_some() {
                        return true;
                    }
                }
            }
            false
        })
        .map(|(_, val)| val)
        .sum()
}
fn part_2(input: &str) -> usize {
    0
}
#[derive(PartialEq, Eq, Debug)]
enum Symbols {
    Symbol,
    NoSymbol,
}
fn dot(input: &str) -> IResult<&str, usize> {
    let (result, _) = many0(tag("."))(input)?;
    Ok((result, input.len() - result.len()))
}
fn symbol(input: &str) -> IResult<&str, Symbols> {
    let (result, _) = alt((
        char('*'),
        char('+'),
        char('#'),
        char('/'),
        char('$'),
        char('%'),
        char('='),
        char('@'),
        char('&'),
        char('-'),
        char('\\'),
    ))(input)?;

    Ok((input, Symbols::Symbol))
}

fn part(input: &str) -> IResult<&str, (usize, usize)> {
    let (result, num) = digit1(input)?;

    Ok((result, (num.parse().unwrap(), input.len() - result.len())))
}

fn parse(
    input: &str,
    parts: &mut HashMap<(isize, isize), usize>,
    symbols: &mut HashMap<(isize, isize), Symbols>,
) {
    let mut y = 0;
    input.lines().for_each(|line| {
        let mut line = line.to_string();
        let mut count = 0;
        while line.len() > 0 {
            match dot(&line) {
                Ok((_, snipped)) => {
                    count += snipped;
                    line.drain(0..snipped);
                }
                Err(_) => {}
            }
            match part(&line) {
                Ok((_, (num, snipped))) => {
                    line.drain(0..snipped);
                    parts.insert((count as isize, y), num);
                    count += snipped;
                }
                Err(_) => {}
            }
            match symbol(&line) {
                Ok((_, sym)) => {
                    let snipped = 1;
                    line.drain(0..snipped);
                    symbols.insert((count as isize, y), sym);
                    count += snipped;
                }
                Err(_) => {}
            }
        }
        y += 1;
    });
}
#[cfg(test)]
mod tests {
    use std::{io::Write, vec};

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_dtos() {
        let input = ".....114.#.";

        let (parts, symbols_snipped) = dot(input).unwrap();
        assert_eq!(symbols_snipped, 5);

        let (rest, (num, snipped)) = part(parts).unwrap();
        assert_eq!(num, 114);
        assert_eq!(snipped, 3);
        let (rest, symbols_snipped) = dot(rest).unwrap();
        assert_eq!(symbols_snipped, 1);

        let (rest, sym) = symbol(rest).unwrap();

        assert_eq!(sym, Symbols::Symbol);
    }

    #[test]
    fn test_parse() {
        let input = "467..114.+.";
        let mut symbols: HashMap<(isize, isize), Symbols> = HashMap::new();
        let mut parts: HashMap<(isize, isize), usize> = HashMap::new();

        parse(input, &mut parts, &mut symbols);
        assert_eq!(symbols.get(&(9, 0)).unwrap(), &Symbols::Symbol);
        assert_eq!(parts, HashMap::from([((0, 0), 467), ((5, 0), 114)]));
    }

    #[test]
    fn test_day_1() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        let result = part_1(input);
        assert_eq!(result, 4361);
    }

    #[test]
    fn test_day_2() {}
}
