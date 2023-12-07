use std::collections::HashMap;

use super::card::Card;

#[derive(Eq, PartialEq, Debug, PartialOrd, Ord, Clone, Copy)]
pub enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl HandType {
    pub fn parse(hand: &Vec<Card>) -> Self {
        let mut counts: HashMap<&Card, u8> = HashMap::new();
        for card in hand {
            *counts.entry(card).or_insert(0) += 1;
        }

        let joker_count = counts.get(&Card::CJoker).unwrap_or(&0);

        match (counts.len(), joker_count) {
            (5, 0) => HandType::HighCard,
            (5, 1) => HandType::OnePair,
            (4, 0) => HandType::OnePair,
            (4, _) => HandType::ThreeOfAKind,
            (3, 0) => {
                if counts.iter().any(|(_, v)| *v == 3) {
                    return HandType::ThreeOfAKind;
                }
                HandType::TwoPair
            }
            (3, 1) => {
                if counts.iter().any(|(_, v)| *v == 3) {
                    return HandType::FourOfAKind;
                }
                HandType::FullHouse
            }
            (3, _) => HandType::FourOfAKind,
            (2, 0) => {
                if counts.iter().any(|(_, v)| *v == 4) {
                    return HandType::FourOfAKind;
                }
                HandType::FullHouse
            }
            (2, _) => HandType::FiveOfAKind,
            (1, _) => HandType::FiveOfAKind,
            (_, _) => panic!(
                "Invalid hand type. Hand was {}, with {} jokers.",
                hand.iter().map(|card| card.to_string()).collect::<String>(),
                joker_count
            ),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::challenge::camel_cards::card::Card;

    use super::*;

    fn get_cards(hand: &str) -> Vec<Card> {
        hand.chars().map(|c| Card::from(c)).collect::<Vec<Card>>()
    }

    fn get_cards_joker(hand: &str) -> Vec<Card> {
        hand.chars()
            .map(|c| Card::from_with_joker(c))
            .collect::<Vec<Card>>()
    }

    #[test]
    fn ch07_hand_type_parse() {
        let hand = get_cards("KKTT4");
        assert_eq!(HandType::parse(&hand), HandType::TwoPair);

        let hand = get_cards("23456");
        assert_eq!(HandType::parse(&hand), HandType::HighCard);

        let hand = get_cards("AAAA4");
        assert_eq!(HandType::parse(&hand), HandType::FourOfAKind);

        let hand = get_cards("KKK65");
        assert_eq!(HandType::parse(&hand), HandType::ThreeOfAKind);

        let hand = get_cards("KKJJJ");
        assert_eq!(HandType::parse(&hand), HandType::FullHouse);
    }

    #[test]
    fn ch07_hand_type_ord() {
        assert!(HandType::FourOfAKind > HandType::TwoPair);
        assert!(HandType::FiveOfAKind > HandType::FullHouse);
        assert!(HandType::FullHouse > HandType::HighCard);
        assert!(HandType::ThreeOfAKind > HandType::TwoPair);
        assert!(HandType::FourOfAKind > HandType::OnePair);
    }

    #[test]
    fn ch07_hand_type_parse_with_joker() {
        let hand = get_cards_joker("KKTT4");
        assert_eq!(HandType::parse(&hand), HandType::TwoPair);

        let hand = get_cards_joker("KKJJ4");
        assert_eq!(HandType::parse(&hand), HandType::FourOfAKind);

        let hand = get_cards_joker("AAAAJ");
        assert_eq!(HandType::parse(&hand), HandType::FiveOfAKind);

        let hand = get_cards_joker("KKKJ5");
        assert_eq!(HandType::parse(&hand), HandType::FourOfAKind);

        let hand = get_cards_joker("J2345");
        assert_eq!(HandType::parse(&hand), HandType::OnePair);

        let hand = get_cards_joker("J2245");
        assert_eq!(HandType::parse(&hand), HandType::ThreeOfAKind);

        let hand = get_cards_joker("JJ345");
        assert_eq!(HandType::parse(&hand), HandType::ThreeOfAKind);

        let hand = get_cards_joker("J5335");
        assert_eq!(HandType::parse(&hand), HandType::FullHouse);
    }
}
