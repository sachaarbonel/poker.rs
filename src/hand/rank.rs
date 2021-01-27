//use std::cmp::Ordering;

//https://www.fgbradleys.com/et_poker.asp

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Rank {
    HighCard,
    Pair,
    TwoPairs,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
    RoyalFlush,
}

#[cfg(test)]
mod tests {
    use crate::hand;

    use super::*;

    // #[test]
    // fn it_works() {
    //     let mut player = hand::Hand::from_vec(vec!["2h", "3h", "4h", "5h", "6h"]);
    //     let player_rank = player.rank();
    //     // assert_eq!(player_rank, Rank::Flush);
    //     let mut opponent = hand::Hand::from_vec(vec!["Ks", "As", "10s", "Qs", "Js"]);
    //     let opponent_rank = opponent.rank();
    //     assert_eq!(opponent_rank  < player_rank, false);
    // }

    #[test]
    fn test_sort_ranks() {
        let mut ranks = vec![
            Rank::RoyalFlush,
            Rank::StraightFlush,
            Rank::FourOfAKind,
            Rank::FullHouse,
            Rank::Flush,
            Rank::Straight,
            Rank::ThreeOfAKind,
            Rank::TwoPairs,
            Rank::Pair,
            Rank::HighCard,
        ];
        ranks.sort();
        assert_eq!(
            ranks,
            vec![
                Rank::HighCard,
                Rank::Pair,
                Rank::TwoPairs,
                Rank::ThreeOfAKind,
                Rank::Straight,
                Rank::Flush,
                Rank::FullHouse,
                Rank::FourOfAKind,
                Rank::StraightFlush,
                Rank::RoyalFlush,
            ]
        )
    }
}
