use std::collections::HashSet;

use nom::{
    bytes::complete::tag,
    character::complete,
    multi::{many1, separated_list1},
    sequence::preceded,
    IResult,
};

pub struct Card {
    pub winning: Vec<i32>,
    pub numbers: Vec<i32>,
}

impl Card {
    pub fn match_count(&self) -> usize {
        let winning: HashSet<i32> = HashSet::from_iter(self.winning.iter().cloned());
        let numbers: HashSet<i32> = HashSet::from_iter(self.numbers.iter().cloned());
        winning.intersection(&numbers).count()
    }

    fn card_parser(input: &str) -> IResult<&str, Card> {
        let (input, _) = tag("Card")(input)?;
        let (input, _) = preceded(many1(tag(" ")), complete::i32)(input)?;
        let (input, _) = tag(":")(input)?;
        let (input, _) = many1(tag(" "))(input)?;
        let (input, winning) = separated_list1(many1(tag(" ")), complete::i32)(input)?;
        let (input, _) = tag(" |")(input)?;
        let (input, _) = many1(tag(" "))(input)?;
        let (input, numbers) = separated_list1(many1(tag(" ")), complete::i32)(input)?;

        assert!(input.is_empty());

        Ok((input, Card { winning, numbers }))
    }

    pub fn parse(input: &str) -> Card {
        Card::card_parser(input).unwrap().1
    }
}
