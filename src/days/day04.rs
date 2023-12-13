use crate::prelude::*;

pub struct Day04;
impl Day for Day04 {
    fn star1(&self, input: String) -> String {
        input
            .parsed_lines()
            .sum_by(|card: Card| card.score())
            .to_string()
    }

    fn star2(&self, input: String) -> String {
        input.parse::<CardSheet>().unwrap().score().to_string()
    }
}

struct Card {
    numbers: Vec<usize>,
    // I started with HashSet here for constant time lookup but it turns out vec is faster
    // I guess since the vec is small the constant time factors on hashing are too big.
    // interesting...
    //
    // try to build your own hashset for this use case. We know we always have two digit numbers
    // so the hashset could just be vec![false; 100];
    //
    // Update: I wrote the DigitSet. It is about 30% faster than the vec for this use case
    // 2x faster than the HashSet
    winning_numbers: DigitSet,
    // winning_numbers: std::collections::HashSet<usize>,
    // winning_numbers: Vec<usize>,
}

impl FromStr for Card {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, nums) = s.split_once(": ").unwrap();
        let (winning, numbers) = nums.split_once(" | ").unwrap();
        let numbers = numbers.split_ascii_whitespace().parse_each().collect();
        let winning_numbers = winning.split_ascii_whitespace().parse_each().collect();
        Ok(Self {
            winning_numbers,
            numbers,
        })
    }
}

impl Card {
    fn score(&self) -> usize {
        match self.num_matches() {
            0 => 0,
            x => 2_usize.pow((x - 1) as u32),
        }
    }

    fn num_matches(&self) -> usize {
        self.numbers
            .iter()
            .filter(|num| self.winning_numbers.contains(num))
            .count()
    }
}

struct CardSheet {
    cards: Vec<Card>,
    // lmao adding this takes the runtime from 30s to 2ms
    // we love a little memoizatoin
    card_scores: Vec<Option<usize>>,
}

impl FromStr for CardSheet {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cards = s.parsed_lines().collect_vec();
        Ok(Self {
            card_scores: vec![None; cards.len()],
            cards,
        })
    }
}

impl CardSheet {
    fn score(&mut self) -> usize {
        (0..self.cards.len())
            .map(|i| self.tally_scratch_card(i))
            .sum()
    }

    /// panics if called on numbers out of bounds. Do bounds checking before calling
    fn tally_scratch_card(&mut self, card_num: usize) -> usize {
        if let Some(score) = self.card_scores[card_num] {
            return score;
        }
        let card = &self.cards[card_num];
        if card_num == self.cards.len() - 1 {
            self.card_scores[card_num] = Some(1);
            return 1;
        }
        let num_matches = card.num_matches();
        let range_start = card_num + 1;
        let range_end = usize::min(self.cards.len() - 1, card_num + num_matches);
        let score = 1
            + (range_start..=range_end)
                .map(|i| self.tally_scratch_card(i))
                .sum::<usize>();
        self.card_scores[card_num] = Some(score);
        score
    }
}
