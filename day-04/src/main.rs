use std::{
    collections::{HashMap, HashSet},
    io::Write,
};

use nom::{
    bytes::complete::tag,
    character::{
        self,
        complete::{alpha1, char, digit1, line_ending, space0, space1},
    },
    multi::{fold_many1, many1, separated_list1},
    sequence::{delimited, separated_pair, terminated, tuple},
    IResult,
};

fn main() {
    let answer_1 = part_1(include_str!("./input.txt"));
    let answer_2 = part_2(include_str!("./input.txt"));

    println!("part_1 {}", answer_1);
    println!("part_2 {}", answer_2);
}

fn part_1(input: &str) -> usize {
    let (_, tickets) = games(input).unwrap();

    tickets
        .iter()
        .map(|ticket| {
            let val = ticket.winners.intersection(&ticket.numbers).count();
            if val > 0 {
                (2 as u32).pow((val - 1) as u32) as usize
            } else {
                0 as usize
            }
        })
        .sum()
}
fn part_2(input: &str) -> usize {
    let (_, tickets) = games(input).unwrap();

    let matches = tickets
        .iter()
        .map(|ticket| {
            (
                ticket.id,
                ticket.winners.intersection(&ticket.numbers).count(),
            )
        })
        .collect::<Vec<(usize, usize)>>();
    let map: HashMap<usize, usize> = matches.iter().fold(HashMap::new(), |mut acc, (id, _)| {
        acc.insert(*id, 1);
        acc
    });

    matches
        .into_iter()
        .fold(map, |mut acc, (id, lucky_numbers)| {
            let count = acc.get(&id).unwrap().clone();
            for i in id..(id + lucky_numbers) {
                let mut ticket_value = acc.entry(i + 1).or_insert(0);
                *ticket_value += count;
            }
            acc
        })
        .values()
        .sum()
}
#[derive(Eq, PartialEq, Debug)]
struct Ticket {
    id: usize,
    winners: HashSet<usize>,
    numbers: HashSet<usize>,
}

fn nums(input: &str) -> IResult<&str, HashSet<usize>> {
    fold_many1(
        terminated(character::complete::u32, space0),
        HashSet::new,
        |mut acc: HashSet<_>, item| {
            acc.insert(item as usize);
            acc
        },
    )(input)
}

fn card(input: &str) -> IResult<&str, Ticket> {
    let (input, num) = strip_card(input)?;
    let (input, (winners, numbers)) = separated_pair(nums, tuple((tag("|"), space1)), nums)(input)?;

    Ok((
        input,
        Ticket {
            winners,
            id: num,
            numbers,
        },
    ))
}
fn strip_card(input: &str) -> IResult<&str, usize> {
    let (input, num) = delimited(
        tuple((tag("Card"), space1)),
        character::complete::u32,
        tuple((tag(":"), space1)),
    )(input)?;

    Ok((input, num as usize))
}
fn games(input: &str) -> IResult<&str, Vec<Ticket>> {
    let (input, g) = separated_list1(line_ending, card)(input)?;
    Ok((input, g))
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    #[test]
    fn parse() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        let (input, result) = games(input).unwrap();
        let result = result.first().unwrap();
        assert_eq!(
            result,
            &Ticket {
                id: 1,
                winners: [48, 86, 83, 41, 17].into_iter().collect(),
                numbers: [17, 31, 53, 6, 86, 9, 48, 83].into_iter().collect(),
            }
        );
    }

    #[test]
    fn test_day_1() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        let result = part_1(input);
        assert_eq!(result, 13);
    }
    #[test]
    fn test_day_2() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        let result = part_2(input);
        assert_eq!(result, 30);
    }
}
