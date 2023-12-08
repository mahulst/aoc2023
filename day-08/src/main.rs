use std::collections::HashMap;

use nom::{
    bytes::complete::tag,
    character::{
        self,
        complete::{alpha1, alphanumeric1, char, digit1, line_ending},
    },
    combinator::map,
    multi::separated_list1,
    sequence::{delimited, separated_pair, tuple},
    IResult,
};

fn main() {
    use Directions::{L, R};
    let directions = vec![
        L, L, R, L, L, R, L, R, L, R, R, R, L, L, R, R, R, L, R, R, L, R, L, R, L, R, L, R, L, R,
        L, R, R, L, R, R, R, L, R, L, L, R, R, L, R, R, L, R, R, R, L, L, R, L, L, R, R, L, L, R,
        R, R, L, L, L, R, L, R, R, R, L, L, L, L, R, R, R, L, L, R, R, R, L, R, R, L, R, L, L, R,
        L, R, L, R, R, R, L, R, R, R, L, R, R, L, R, R, L, R, R, L, R, L, L, R, R, R, L, R, R, L,
        R, R, R, L, L, R, R, R, L, R, L, R, R, L, L, R, R, L, L, R, L, R, L, R, R, L, R, R, L, L,
        R, L, L, R, R, L, R, L, L, R, R, R, L, L, R, R, R, L, R, R, L, L, R, R, L, R, R, R, L, R,
        L, R, R, R, L, R, R, L, L, L, R, L, L, R, L, L, R, R, R, L, R, L, R, L, R, L, R, R, L, R,
        R, R, L, L, L, R, R, R, L, R, R, R, L, R, R, R, L, R, R, L, R, L, R, L, R, L, R, R, R, L,
        R, R, L, L, R, L, R, R, R, L, R, L, R, L, R, R, L, L, L, R, R, R, R,
    ];
    let answer_1 = part_1(include_str!("./input.txt"), directions.clone());
    let answer_2 = part_2(include_str!("./input.txt"), directions.clone());

    println!("part_1 {}", answer_1);
    println!("part_2 {}", answer_2);
}
fn lr(input: &str) -> IResult<&str, (&str, &str)> {
    delimited(
        char('('),
        map(
            tuple((alphanumeric1, tag(", "), alphanumeric1)),
            |(first, _, second)| (first, second),
        ),
        char(')'),
    )(input)
}
fn node(input: &str) -> IResult<&str, (&str, (&str, &str))> {
    let (input, a) = separated_pair(alphanumeric1, tag(" = "), lr)(input)?;

    Ok((input, a))
}

type Nodes<'a> = HashMap<&'a str, (&'a str, &'a str)>;
fn nodes(input: &str) -> IResult<&str, Nodes> {
    let (input, g) = separated_list1(line_ending, node)(input)?;

    Ok((input, g.into_iter().collect()))
}

#[derive(Clone, Copy)]
enum Directions {
    R,
    L,
}
fn part_1(input: &str, directions: Vec<Directions>) -> usize {
    let (_, n) = nodes(input).unwrap();
    let mut dir = directions.iter().cycle();
    let mut location = "AAA";
    let mut count = 0;
    while let Some(num) = dir.next() {
        let branches = n.get(location).unwrap();
        location = match num {
            Directions::R => branches.1,
            Directions::L => branches.0,
        };
        count += 1;
        if location == "ZZZ" {
            return count;
        }
    }
    0
}
fn part_2(input: &str, directions: Vec<Directions>) -> usize {
    let (_, n) = nodes(input).unwrap();
    let mut dir = directions.iter().cycle();
    let mut starts = n
        .iter()
        .filter(|(name, _)| name.chars().last().unwrap() == 'A')
        .enumerate()
        .map(|(index, (name, _))| (index, name.clone()))
        .collect::<HashMap<usize, &str>>();
    dbg!(&starts);
    let mut counts: HashMap<usize, usize> = HashMap::new();
    let mut count = 0;

    while let Some(num) = dir.next() {
        for (index, mut location) in starts.iter_mut() {
            let branches = n.get(location).unwrap();
            let next = match num {
                Directions::R => branches.1,
                Directions::L => branches.0,
            };
            *location = next;
        }

        count += 1;

        starts
            .iter()
            .filter(|(i, location)| location.chars().last().unwrap() == 'Z')
            .for_each(|(i, _)| {
                if !counts.contains_key(i) {
                    counts.insert(*i, count);
                }
            });
        let finished = counts.len();
        if finished == starts.iter().count() {
            let l: Vec<usize> = counts.into_values().collect();
            return least_common_in_list(&l);
        }
    }
    1
}
fn greatest_common_divisor(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        greatest_common_divisor(b, a % b)
    }
}

fn least_common_multiple(a: usize, b: usize) -> usize {
    a / greatest_common_divisor(a, b) * b
}

fn least_common_in_list(numbers: &[usize]) -> usize {
    numbers
        .iter()
        .fold(1, |acc, next| least_common_multiple(acc, *next))
}
#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    #[test]
    fn parse() {
        let input = "AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

        let result = nodes(input);
        let mut nodes: Nodes = HashMap::new();

        nodes.insert("CCC", ("ZZZ", "GGG"));
        nodes.insert("DDD", ("DDD", "DDD"));
        nodes.insert("EEE", ("EEE", "EEE"));
        nodes.insert("GGG", ("GGG", "GGG"));
        nodes.insert("AAA", ("BBB", "CCC"));
        nodes.insert("BBB", ("DDD", "EEE"));
        nodes.insert("ZZZ", ("ZZZ", "ZZZ"));

        assert_eq!(result.unwrap().1, nodes);
    }

    #[test]
    fn test_day_1() {
        let input = "AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

        let result = part_1(input, vec![Directions::R, Directions::L]);

        assert_eq!(result, 2);
    }
    #[test]
    fn test_day_2() {
        let input = "11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

        let result = part_2(input, vec![Directions::L, Directions::R]);

        assert_eq!(result, 6);
    }
}
