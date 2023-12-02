use std::collections::HashMap;

use nom::{
    bytes::complete::{self, tag},
    character::{
        self,
        complete::{alpha1, char, digit1, line_ending},
    },
    multi::{many1, separated_list1},
    sequence::{delimited, separated_pair, terminated},
    IResult,
};

fn main() {
    let answer_1 = part_1(
        include_str!("./input.txt"),
        HashMap::from([("red", 12), ("green", 13), ("blue", 14)]),
    );
    let answer_2 = part_2(include_str!("./input.txt"));

    println!("part_1 {}", answer_1);
    println!("part_2 {}", answer_2);
}

type Round<'a> = HashMap<&'a str, usize>;
#[derive(PartialEq, Eq, Debug)]
struct Game<'a> {
    num: usize,
    rounds: Vec<Round<'a>>,
}
fn cube(input: &str) -> IResult<&str, (&str, usize)> {
    let (input, digits) = digit1(input)?;
    let (input, _) = char(' ')(input)?;
    let (input, string_part) = alpha1(input)?;
    Ok((input, (string_part, digits.parse::<usize>().unwrap())))
}
fn round(input: &str) -> IResult<&str, Round> {
    let (input, rounds) = separated_list1(tag(", "), cube)(input)?;

    Ok((input, rounds.into_iter().collect()))
}
fn game(input: &str) -> IResult<&str, Game> {
    let (input, num) = strip_game(input)?;
    let (input, rounds) = separated_list1(tag("; "), round)(input)?;

    Ok((input, Game { rounds, num }))
}
fn strip_game(input: &str) -> IResult<&str, usize> {
    let (input, num) = delimited(tag("Game "), character::complete::u32, tag(": "))(input)?;

    Ok((input, num as usize))
}
fn games(input: &str) -> IResult<&str, Vec<Game>> {
    let (input, g) = separated_list1(line_ending, game)(input)?;
    Ok((input, g))
}

fn part_1(input: &str, expected: HashMap<&str, usize>) -> usize {
    let (input, games) = games(input).unwrap();
    let games_possible = games.iter().filter(|game| {
        let f = game.rounds.iter().all(|rounds| {
            rounds.iter().all(|round| {
                let max = expected.get(round.0).unwrap_or(&999);
                max >= round.1
            })
        });

        f
    });
    games_possible.map(|game| game.num).sum()
}
fn part_2(input: &str) -> usize {
    let (input, games) = games(input).unwrap();
    games
        .iter()
        .map(|game| {
            let max_per_color = game.rounds.iter().fold(
                HashMap::from([("blue", 0), ("red", 0), ("green", 0)]),
                |mut acc, next| {
                    next.into_iter().for_each(|(key, val)| {
                        acc.entry(key).and_modify(|e| *e = std::cmp::max(*e, *val));
                    });
                    acc
                },
            );

            let result: usize = max_per_color.values().product();
            result
        })
        .sum()
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    #[test]
    fn parse() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
";
        let result = games(input).unwrap();

        assert_eq!(
            result.1,
            vec![Game {
                rounds: vec![
                    HashMap::from([("blue", 3), ("red", 4),]),
                    HashMap::from([("red", 1), ("green", 2), ("blue", 6),]),
                    HashMap::from([("green", 2),])
                ],
                num: 1
            }]
        );
    }

    #[test]
    fn test_day_1() {
        let result = part_1(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
            HashMap::from([("red", 12), ("green", 13), ("blue", 14)]),
        );
        assert_eq!(result, 8);
    }
    #[test]
    fn test_day_2() {
        let result = part_2(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        );
        assert_eq!(result, 2286);
    }
}
