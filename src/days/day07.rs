use crate::prelude::*;
use std::cmp::Ordering;

pub struct Day07;
impl Day for Day07 {
    fn star1(&self, input: String) -> String {
        input
            .parsed_lines::<Hand>()
            .sorted()
            .enumerate()
            .sum_by(|(score, hand)| (score + 1) * hand.bid)
            .to_string()
    }

    fn star2(&self, input: String) -> String {
        input
            .parsed_lines::<Hand>()
            .map(|hand| hand.switch_to_wild_type())
            .sorted()
            .enumerate()
            .sum_by(|(score, hand)| (score + 1) * hand.bid)
            .to_string()
    }
}

#[derive(PartialEq, Eq)]
struct Hand {
    cards: Vec<Card>,
    bid: usize,
    hand_type: HandType,
}

impl FromStr for Hand {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (cards, bid) = s.split_once(' ').unwrap();
        let cards = cards.chars().map(Card::from).collect_vec();
        let bid = bid.parse().unwrap();
        let hand_type = Self::get_type(&cards);
        Ok(Self {
            cards,
            bid,
            hand_type,
        })
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.hand_type.cmp(&other.hand_type) {
            Ordering::Equal => self
                .cards
                .iter()
                .zip(other.cards.iter())
                .map(|(card, other_card)| card.cmp(other_card))
                .find(|ord| ord != &Ordering::Equal)
                .unwrap(),
            x => x,
        }
    }
}

impl Hand {
    fn get_type(cards: &[Card]) -> HandType {
        let mut num_cards = cards
            .iter()
            .counts_by(|card| card.strength)
            .into_values()
            .sorted()
            .rev();

        match (num_cards.next(), num_cards.next()) {
            (Some(5), _) => HandType::FiveOfaKind,
            (Some(4), _) => HandType::FourOfaKind,
            (Some(3), Some(2)) => HandType::FullHouse,
            (Some(3), _) => HandType::ThreeOfaKind,
            (Some(2), Some(2)) => HandType::TwoPair,
            (Some(2), _) => HandType::OnePair,
            _ => HandType::HighCard,
        }
    }

    fn switch_to_wild_type(mut self) -> Self {
        let num_jokers = self
            .cards
            .iter_mut()
            .filter(|card| card.strength == 11)
            .map(|card| card.strength = 1)
            .count();

        let mut num_cards = self
            .cards
            .iter()
            .filter(|card| card.strength != 1)
            .counts_by(|card| card.strength)
            .into_values()
            .sorted()
            .rev();

        let most_card = num_cards.next();
        let second_most_card = num_cards.next();

        let most_card = most_card.map_or(num_jokers, |card| card + num_jokers);

        let new_type = match (most_card, second_most_card) {
            (5, _) => HandType::FiveOfaKind,
            (4, _) => HandType::FourOfaKind,
            (3, Some(2)) => HandType::FullHouse,
            (3, _) => HandType::ThreeOfaKind,
            (2, Some(2)) => HandType::TwoPair,
            (2, _) => HandType::OnePair,
            _ => HandType::HighCard,
        };
        self.hand_type = new_type;
        self
    }
}

// Confirm this derivation works like how I want it to
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfaKind,
    FullHouse,
    FourOfaKind,
    FiveOfaKind,
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug)]
struct Card {
    strength: usize,
}

impl From<char> for Card {
    fn from(value: char) -> Self {
        let strength = match value {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 11,
            'T' => 10,
            x => x.to_digit(10).unwrap() as usize,
        };
        Self { strength }
    }
}

#[test]
fn ordering() {
    use HandType::*;
    assert!(FiveOfaKind > HighCard);
    assert!(FullHouse > TwoPair);
    assert!(OnePair < FourOfaKind);
}
