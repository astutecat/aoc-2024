use std::collections::HashMap;

use nom::{
    character::complete::{digit1, line_ending, space1},
    combinator::map_res,
    multi::separated_list0,
    sequence::separated_pair,
    IResult,
};

advent_of_code::solution!(1);

fn parse_int(input: &str) -> IResult<&str, i64> {
    map_res(digit1, |s: &str| s.parse::<i64>())(input)
}

fn parse_input(input: &str) -> IResult<&str, (Vec<i64>, Vec<i64>)> {
    let (input, pairs) = separated_list0(line_ending, parse_pair)(input)?;
    Ok((input, pairs.into_iter().unzip()))
}

fn parse_pair(input: &str) -> IResult<&str, (i64, i64)> {
    let (input, (left, right)) = separated_pair(parse_int, space1, parse_int)(input)?;
    Ok((input, (left, right)))
}

pub fn part_one(input: &str) -> Option<i64> {
    let (_, (mut left, mut right)) = parse_input(input).unwrap();
    left.sort();
    right.sort();

    let result = left
        .iter()
        .zip(right.iter())
        .fold(0, |acc, (a, b)| acc + (a - b).abs());

    Some(result)
}

pub fn part_two(input: &str) -> Option<i64> {
    let (_, (left, right)) = parse_input(input).unwrap();
    let right = right.iter().fold(HashMap::new(), |mut acc, a| {
        acc.entry(a).and_modify(|entry| *entry += 1).or_insert(1);
        acc
    });

    let result = left
        .iter()
        .fold(0, |acc, a| acc + (right.get(a).unwrap_or(&{ 0 }) * a));

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
