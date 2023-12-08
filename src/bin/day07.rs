use aoc2023::prelude::*;

const INPUT: &str = include_str!("../../inputs/day07.txt");

#[derive(Debug)]
struct Game {
    hand: Hand,
    bid: u64,
}

impl std::str::FromStr for Game {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (hand, bid) = s.split_once(' ').unwrap();
        let hand = str::parse(hand).unwrap();
        let bid = str::parse(bid).unwrap();

        Ok(Self { hand, bid })
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Hand {
    cards: [Card; 5],
}

impl Hand {
    fn hand_type(&self) -> HandType {
        let counts: HashMap<Card, usize> =
            self.cards
                .iter()
                .fold(HashMap::<Card, usize>::new(), |mut counts, card| {
                    *counts.entry(*card).or_default() += 1;
                    counts
                });

        let mut counts: Vec<_> = counts.into_values().collect();
        counts.sort();

        let mut counts = counts.into_iter().rev();

        let counts: [usize; 5] = [
            counts.next().unwrap_or_default(),
            counts.next().unwrap_or_default(),
            counts.next().unwrap_or_default(),
            counts.next().unwrap_or_default(),
            counts.next().unwrap_or_default(),
        ];

        match counts {
            [5, ..] => HandType::FiveOfAKind,
            [4, ..] => HandType::FourOfAKind,
            [3, 2, ..] => HandType::FullHouse,
            [3, 1, ..] => HandType::ThreeOfAKind,
            [2, 2, ..] => HandType::TwoPair,
            [2, 1, ..] => HandType::OnePair,
            [1, ..] => HandType::HighCard,
            _ => unreachable!(),
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        use std::cmp::Ordering::*;

        let hand_type_ord = self.hand_type().cmp(&other.hand_type());

        match hand_type_ord {
            Less | Greater => hand_type_ord,
            Equal => self.cards.cmp(&other.cards),
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::str::FromStr for Hand {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cards_iter = s.chars().map(Card::from_char);

        Ok(Self {
            cards: [
                cards_iter.next().unwrap(),
                cards_iter.next().unwrap(),
                cards_iter.next().unwrap(),
                cards_iter.next().unwrap(),
                cards_iter.next().unwrap(),
            ],
        })
    }
}

const CARD_CHAR_MAPPPING: [(Card, char); 13] = [
    (Card::Two, '2'),
    (Card::Three, '3'),
    (Card::Four, '4'),
    (Card::Five, '5'),
    (Card::Six, '6'),
    (Card::Seven, '7'),
    (Card::Eight, '8'),
    (Card::Nine, '9'),
    (Card::Ten, 'T'),
    (Card::Jack, 'J'),
    (Card::Queen, 'Q'),
    (Card::King, 'K'),
    (Card::Ace, 'A'),
];

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Card {
    fn from_char(c: char) -> Self {
        CARD_CHAR_MAPPPING
            .iter()
            .find(|(_, ch)| c == *ch)
            .unwrap()
            .0
    }

    // fn to_char(&self) -> char {
    //     CARD_CHAR_MAPPPING
    //         .iter()
    //         .find(|(card, _)| self == card)
    //         .unwrap()
    //         .1
    // }
}

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn solve(input: &str) -> u64 {
    let mut games: Vec<Game> = input
        .lines()
        .map(|line| str::parse(line).unwrap())
        .collect();

    games.sort_by(|a, b| a.hand.cmp(&b.hand));

    games
        .into_iter()
        .enumerate()
        .rev()
        .map(|(rank, game)| game.bid * (rank as u64 + 1))
        .sum()
}

fn part1(input: &str) -> u64 {
    solve(input)
}

fn part2(input: &str) -> u64 {
    solve(input)
}

fn main() {
    println!("part1: {}", part1(INPUT));
    println!("part2: {}", part2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() {
        let input = include_str!("../../inputs/day07p1_example.txt");
        assert_eq!(part1(input), 6440);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 255048101);
    }

    // #[test]
    // fn test_part2_example() {
    //     let input = include_str!("../../inputs/day07p2_example.txt");
    //     assert_eq!(part2(input), 5905);
    // }

    // #[test]
    // fn test_part2() {
    //     let input = include_str!("../../inputs/day07.txt");
    //     assert_eq!(part2(input), 71585);
    // }
}
