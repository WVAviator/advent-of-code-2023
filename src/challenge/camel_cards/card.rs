use std::fmt::Display;

#[derive(Debug, Eq, PartialEq, PartialOrd, Hash, Ord, Clone, Copy)]
pub enum Card {
    CJoker,
    C2,
    C3,
    C4,
    C5,
    C6,
    C7,
    C8,
    C9,
    CT,
    CJ,
    CQ,
    CK,
    CA,
}

impl Card {
    pub fn from(ch: char) -> Self {
        match ch {
            '2' => Card::C2,
            '3' => Card::C3,
            '4' => Card::C4,
            '5' => Card::C5,
            '6' => Card::C6,
            '7' => Card::C7,
            '8' => Card::C8,
            '9' => Card::C9,
            'T' => Card::CT,
            'J' => Card::CJ,
            'Q' => Card::CQ,
            'K' => Card::CK,
            'A' => Card::CA,
            _ => panic!("Attempted to parse invalid card."),
        }
    }

    pub fn from_with_joker(ch: char) -> Self {
        if ch == 'J' {
            return Card::CJoker;
        }

        Card::from(ch)
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Card::CJoker => 'J',
            Card::C2 => '2',
            Card::C3 => '3',
            Card::C4 => '4',
            Card::C5 => '5',
            Card::C6 => '6',
            Card::C7 => '7',
            Card::C8 => '8',
            Card::C9 => '9',
            Card::CT => 'T',
            Card::CJ => 'J',
            Card::CQ => 'Q',
            Card::CK => 'K',
            Card::CA => 'A',
        };

        write!(f, "{}", c)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ch07_from() {
        let card = Card::from('A');
        assert_eq!(card, Card::CA)
    }

    #[test]
    fn ch07_ord() {
        let mut cards = vec![
            Card::from('T'),
            Card::from('K'),
            Card::from('3'),
            Card::from('7'),
        ];

        let expected = vec![
            Card::from('3'),
            Card::from('7'),
            Card::from('T'),
            Card::from('K'),
        ];

        cards.sort();

        assert_eq!(cards, expected);

        assert!(Card::from('K') > Card::from('J'));
    }
}
