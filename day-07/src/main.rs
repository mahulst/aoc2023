use std::{
    cmp::{Ordering, Reverse},
    collections::HashMap,
    io::Write,
    vec,
};

use nom::{
    bytes::complete::{self, tag},
    character::{
        self,
        complete::{alpha1, alphanumeric1, anychar, char, digit1, line_ending},
    },
    multi::{many1, separated_list1},
    sequence::{delimited, separated_pair, terminated},
    IResult,
};

fn main() {
    let answer_1 = part_1(include_str!("./input.txt"));
    let answer_2 = part_2(include_str!("./input.txt"));

    println!("part_1 {}", answer_1);
    println!("part_2 {}", answer_2);
}

#[derive(Debug)]
struct Hand {
    cards: Vec<Card>,
    score: usize,
    hand_type: HandTypes,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandTypes {
    FiveOfAKind = 6,
    FourOfAKind = 5,
    FullHouse = 4,
    ThreeOfAKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0,
}
impl HandTypes {
    fn score(cards: &Vec<Card>) -> HandTypes {
        let mut counts = HashMap::new();

        for variant in cards {
            *counts.entry(variant).or_insert(0) += 1;
        }
        let mut counts_vec: Vec<_> = counts.into_iter().collect();
        counts_vec.sort_by_key(|&(_, count)| Reverse(count));
        let largest_two = counts_vec.into_iter().take(2).collect::<Vec<_>>();
        let largest = largest_two[0];
        let second_largest = largest_two.get(1).unwrap_or(&(&Card::T, 0));

        match (largest.1, second_largest.1) {
            (5, 0) => HandTypes::FiveOfAKind,
            (4, 1) => HandTypes::FourOfAKind,
            (3, 2) => HandTypes::FullHouse,
            (3, _) => HandTypes::ThreeOfAKind,
            (2, 2) => HandTypes::TwoPair,
            (2, _) => HandTypes::OnePair,
            _ => HandTypes::HighCard,
        }
    }
    fn score2(cards: &Vec<Card>) -> HandTypes {
        let mut counts = HashMap::new();

        for variant in cards {
            *counts.entry(variant).or_insert(0) += 1;
        }
        let jokers = counts.get(&Card::S).unwrap_or(&0).clone();
        let mut counts_vec: Vec<_> = counts.into_iter().collect();
        counts_vec.sort_by_key(|&(_, count)| Reverse(count));
        let largest_two = counts_vec
            .into_iter()
            .filter(|(c, _)| c != &&Card::S)
            .collect::<Vec<_>>();

        let largest = largest_two.get(0).unwrap_or(&(&Card::T, 0));

        let second_largest = largest_two.get(1).unwrap_or(&(&Card::T, 0));

        match (largest.1, second_largest.1, jokers) {
            (5, 0, 0) => HandTypes::FiveOfAKind,
            (4, 0, 1) => HandTypes::FiveOfAKind,
            (4, 1, 0) => HandTypes::FourOfAKind,
            (3, 2, 0) => HandTypes::FullHouse,
            (3, 1, 1) => HandTypes::FourOfAKind,
            (3, 1, 0) => HandTypes::ThreeOfAKind,
            (3, 0, 2) => HandTypes::FiveOfAKind,
            (2, 2, 1) => HandTypes::FullHouse,
            (2, 1, 2) => HandTypes::FourOfAKind,
            (2, 2, 0) => HandTypes::TwoPair,
            (2, 1, 0) => HandTypes::OnePair,
            (2, 0, 3) => HandTypes::FiveOfAKind,
            (2, 1, 1) => HandTypes::ThreeOfAKind,
            (1, 1, 3) => HandTypes::FourOfAKind,
            (1, 1, 2) => HandTypes::ThreeOfAKind,
            (1, 0, 4) => HandTypes::FiveOfAKind,
            (0, 0, 5) => HandTypes::FiveOfAKind,
            (1, _, 1) => HandTypes::OnePair,
            (1, _, 0) => HandTypes::HighCard,
            a => {
                dbg!(a);
                panic!("halp")
            }
        }
    }
}
impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.hand_type == other.hand_type
    }
}

impl Eq for Hand {}
impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.hand_type != other.hand_type {
            return self.hand_type.cmp(&other.hand_type);
        }
        self.cards
            .iter()
            .zip(other.cards.iter())
            .map(|(a, b)| a.cmp(b))
            .filter(|ord| ord != &Ordering::Equal)
            .next()
            .unwrap_or(Ordering::Equal)
    }
}
#[derive(Hash, Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Card {
    V2 = 1,
    V3 = 2,
    V4 = 3,
    V5 = 4,
    V6 = 5,
    V7 = 6,
    V8 = 7,
    V9 = 8,
    T = 9,
    J = 10,
    Q = 11,
    K = 12,
    A = 13,
    S = 0,
}
impl Card {
    fn from_char(char: char) -> Card {
        match char {
            '2' => Card::V2,
            '3' => Card::V3,
            '4' => Card::V4,
            '5' => Card::V5,
            '6' => Card::V6,
            '7' => Card::V7,
            '8' => Card::V8,
            '9' => Card::V9,
            'T' => Card::T,
            'J' => Card::J,
            'Q' => Card::Q,
            'K' => Card::K,
            'A' => Card::A,
            _ => Card::T,
        }
    }
    fn from_char2(char: char) -> Card {
        match char {
            '2' => Card::V2,
            '3' => Card::V3,
            '4' => Card::V4,
            '5' => Card::V5,
            '6' => Card::V6,
            '7' => Card::V7,
            '8' => Card::V8,
            '9' => Card::V9,
            'T' => Card::T,
            'J' => Card::S,
            'Q' => Card::Q,
            'K' => Card::K,
            'A' => Card::A,
            _ => Card::T,
        }
    }
}

fn hand(input: &str) -> IResult<&str, Hand> {
    let (input, (cards, val)) =
        separated_pair(alphanumeric1, char(' '), character::complete::u32)(input)?;

    let chars = cards.chars().map(|c| Card::from_char2(c)).collect();
    let hand = Hand {
        hand_type: HandTypes::score2(&chars),
        score: val as usize,
        cards: chars,
    };
    Ok((input, hand))
}
fn hands(input: &str) -> IResult<&str, Vec<Hand>> {
    let (input, g) = separated_list1(line_ending, hand)(input)?;
    Ok((input, g))
}

fn part_1(input: &str) -> usize {
    let (_, mut hands) = hands(input).unwrap();
    hands.sort();
    hands
        .iter()
        .enumerate()
        .map(|(index, hand)| hand.score * (index + 1))
        .sum()
}
fn part_2(input: &str) -> usize {
    let (_, mut hands) = hands(input).unwrap();
    hands.sort();
    hands
        .iter()
        .enumerate()
        .map(|(index, hand)| hand.score * (index + 1))
        .sum()
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    #[test]
    fn compare() {
        let a = Hand {
            cards: vec![Card::A, Card::A, Card::A, Card::A, Card::A],
            hand_type: HandTypes::FiveOfAKind,
            score: 0,
        };
        let b = Hand {
            cards: vec![Card::K, Card::A, Card::A, Card::A, Card::A],
            hand_type: HandTypes::FourOfAKind,
            score: 0,
        };
        assert_eq!(a.cmp(&b), Ordering::Greater);

        let a = Hand {
            cards: vec![Card::A, Card::A, Card::A, Card::A, Card::A],
            hand_type: HandTypes::FiveOfAKind,
            score: 0,
        };
        let b = Hand {
            cards: vec![Card::K, Card::K, Card::K, Card::K, Card::A],
            hand_type: HandTypes::FiveOfAKind,
            score: 0,
        };
        assert_eq!(b.cmp(&a), Ordering::Less);
    }

    #[test]
    fn parse() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
J463A 363
JJJJJ 905
3T33T 702
QQQJA 483";
        let (_, result) = hands(input).unwrap();
        assert_eq!(
            result.first().unwrap(),
            &Hand {
                hand_type: HandTypes::OnePair,
                cards: vec![Card::V3, Card::V2, Card::T, Card::V3, Card::K],
                score: 765
            }
        )
    }

    #[test]
    fn test_day_1() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        let result = part_1(input);
        assert_eq!(result, 6440);
    }
    #[test]
    fn test_day_2() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        let result = part_2(input);
        assert_eq!(result, 5905);
    }
}
