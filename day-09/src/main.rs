use std::collections::HashMap;

use nom::{
    bytes::complete::tag,
    character::{
        self,
        complete::{alpha1, alphanumeric1, char, digit1, line_ending},
    },
    combinator::{map, map_res, opt, recognize},
    multi::separated_list1,
    sequence::{delimited, preceded, separated_pair, tuple},
    IResult,
};

fn main() {
    let answer_1 = part_1(include_str!("./input.txt"));
    let answer_2 = part_2(include_str!("./input.txt"));

    println!("part_1 {}", answer_1);
    println!("part_2 {}", answer_2);
}
fn oasis(input: &str) -> IResult<&str, Vec<isize>> {
    let (input, g) = separated_list1(tag(" "), parse_isize)(input)?;
    Ok((input, g.into_iter().collect()))
}
fn oasis_list(input: &str) -> IResult<&str, Vec<Vec<isize>>> {
    let (input, g) = separated_list1(line_ending, oasis)(input)?;

    Ok((input, g.into_iter().collect()))
}
fn parse_isize(input: &str) -> IResult<&str, isize> {
    let (i, number) = map_res(recognize(preceded(opt(tag("-")), digit1)), |s| {
        isize::from_str_radix(s, 10)
    })(input)?;

    Ok((i, number))
}
fn calc_score_2(input: Vec<Vec<isize>>) -> isize {
    input.iter().rev().fold(0, |acc, i| {
        let a = i.first().unwrap();
        a - acc
    })
}
fn calc_score(input: Vec<Vec<isize>>) -> isize {
    input.iter().rev().fold(0, |acc, i| {
        let a = i.last().unwrap_or(&0);
        a + acc
    })
}
fn part_1(input: &str) -> isize {
    let (_, oasises) = oasis_list(input).unwrap();

    oasises
        .into_iter()
        .map(|oasis| {
            let mut finished = false;
            let mut changes = vec![oasis.clone()];
            while (!finished) {
                let last = changes.last().unwrap();
                let mut acc = vec![];
                let result = last.windows(2).fold(acc, |mut acc, i| {
                    acc.push(i[1] - i[0]);
                    acc
                });
                if result.iter().all(|&i| i == 0) {
                    finished = true
                }
                changes.push(result);
            }
            let i = calc_score(changes);

            i
        })
        .sum()
}
fn part_2(input: &str) -> isize {
    let (_, oasises) = oasis_list(input).unwrap();

    oasises
        .into_iter()
        .map(|oasis| {
            let mut finished = false;
            let mut changes = vec![oasis.clone()];
            while (!finished) {
                let last = changes.last().unwrap();
                let mut acc = vec![];
                let result = last.windows(2).fold(acc, |mut acc, i| {
                    acc.push(i[1] - i[0]);
                    acc
                });

                if result.iter().all(|&i| i == 0) {
                    finished = true
                }

                changes.push(result);
            }
            let i = calc_score_2(changes);
            i
        })
        .sum()
}
#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    #[test]
    fn parse() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 -30 45";

        let result = oasis_list(input);
        let expected = vec![
            vec![0, 3, 6, 9, 12, 15],
            vec![1, 3, 6, 10, 15, 21],
            vec![10, 13, 16, 21, -30, 45],
        ];
        assert_eq!(result.unwrap().1, expected);
    }

    #[test]
    fn test_day_1() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

        let result = part_1(input);

        assert_eq!(result, 114);
    }
    #[test]
    fn testa() {
        let input =
            "3 9 23 45 75 113 159 213 275 345 423 509 603 705 815 933 1059 1193 1335 1485 1643";

        let result = part_2(input);
        assert_eq!(result, 5);
    }

    #[test]
    fn test_day_2() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

        let result = part_2(input);

        assert_eq!(result, 2);
    }
}
