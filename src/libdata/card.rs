use std::{char, fmt};
use std::from_str::FromStr;

#[deriving(Eq, PartialEq)]
pub enum Suit {
    Dummy, Spade, Heart, Dia, Club
}

impl fmt::Show for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            &Dummy => "_",
            &Spade => "S",
            &Heart => "H",
            &Dia   => "D",
            &Club  => "C"
        };

        write!(f, "{}", s)
    }
}

impl FromStr for Suit {
    fn from_str(s: &str) -> Option<Suit> {
        if s.len() != 1 { return None; }
        return match s {
            "S" => Some(Spade),
            "H" => Some(Heart),
            "D" => Some(Dia),
            "C" => Some(Club),
            _   => None
        };
    }
}

#[deriving(Eq, PartialEq)]
pub struct Card {
    pub num: uint,
    pub suit: Suit
}

impl fmt::Show for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let n = match self.num {
            10 => "T".to_string(),
            11 => "J".to_string(),
            12 => "Q".to_string(),
            13 => "K".to_string(),
            1  => "A".to_string(),
            0  => "_".to_string(),
            n  => n.to_str()
        };

        write!(f, "{}{}", n, self.suit)
    }
}

impl FromStr for Card {
    fn from_str(s: &str) -> Option<Card> {
        if s.len() != 2 { return None; }
        let suit = FromStr::from_str(s.slice(1, 2));
        let num = match s.char_at(0) {
            'A' => 1,
            'T' => 10,
            'J' => 11,
            'Q' => 12,
            'K' => 13,
            d if char::is_digit(d) => char::to_digit(d, 10).unwrap(),
            _   => return None
        };
        return suit.map(|s| Card { suit: s, num: num} );
    }
}

impl Card {
    pub fn new(num: uint, suit: Suit) -> Card { Card { num: num, suit: suit } }
    pub fn dummy() -> Card { Card::new(0, Dummy) }
}

#[cfg(test)]
mod tests {
    use super::{Suit, Dummy, Spade, Heart, Dia, Club, Card};

    #[test]
    fn show_suit() {
        assert_eq!("_".to_string(), format!("{}", Dummy));

        fn check_pair(s: String, suite: Suit) {
            assert_eq!(s, format!("{}", suite));
            assert_eq!(Some(suite), from_str(s.as_slice()));
        }
        check_pair("S".to_string(), Spade);
        check_pair("H".to_string(), Heart);
        check_pair("D".to_string(), Dia);
        check_pair("C".to_string(), Club);
    }

    #[test]
    fn show_card() {
        assert_eq!("__".to_string(), format!("{}", Card::dummy()));

        fn check_pair(s: String, card: Card) {
            assert_eq!(s, format!("{}", card));
            assert_eq!(Some(card), from_str(s.as_slice()));
        }
        check_pair("AH".to_string(), Card::new(1, Heart));
        check_pair("2C".to_string(), Card::new(2, Club));
        check_pair("9D".to_string(), Card::new(9, Dia));
        check_pair("TS".to_string(), Card::new(10, Spade));
        check_pair("JH".to_string(), Card::new(11, Heart));
        check_pair("QC".to_string(), Card::new(12, Club));
        check_pair("KD".to_string(), Card::new(13, Dia));
    }
}
