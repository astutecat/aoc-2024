use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{anychar, char, digit1},
    combinator::map_res,
    multi::{many1, many_till},
    sequence::separated_pair,
    IResult,
};

advent_of_code::solution!(3);

#[derive(Clone, Copy)]
enum T {
    Mult(u32, u32),
    Enable,
    Disable,
}

fn parse_int(input: &str) -> IResult<&str, u32> {
    map_res(digit1, |s: &str| s.parse::<u32>())(input)
}

fn parse_pair(input: &str) -> IResult<&str, (u32, u32)> {
    separated_pair(parse_int, char(','), parse_int)(input)
}

fn parse_mult(input: &str) -> IResult<&str, T> {
    let (input, _) = tag("mul(")(input)?;
    let (input, (a, b)) = parse_pair(input)?;
    let (input, _) = char(')')(input)?;
    Ok((input, T::Mult(a, b)))
}

fn parse_enable(input: &str) -> IResult<&str, T> {
    let (input, _) = tag("do()")(input)?;
    Ok((input, T::Enable))
}

fn parse_disable(input: &str) -> IResult<&str, T> {
    let (input, _) = tag("don't()")(input)?;
    Ok((input, T::Disable))
}

fn parse_input(input: &str) -> Vec<T> {
    let (_, results) = many1(many_till(anychar, parse_mult))(input).unwrap();
    results.iter().map(|m| m.1).collect()
}

fn parse_input_with_conditionals(input: &str) -> Vec<T> {
    let maybe_t = (parse_mult, parse_enable, parse_disable);
    let (_, result) = many1(many_till(anychar, alt(maybe_t)))(input).unwrap();
    result.iter().map(|m| m.1).collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let parsed = parse_input(input);
    let result = parsed.iter().fold(0, |acc, x| match x {
        T::Mult(a, b) => acc + (a * b),
        _ => acc,
    });

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (_, result) =
        parse_input_with_conditionals(input)
            .iter()
            .fold((true, 0), |(enabled, acc), x| match x {
                T::Enable => (true, acc),
                T::Disable => (false, acc),
                T::Mult(a, b) if enabled => (enabled, acc + (a * b)),
                _ => (enabled, acc),
            });
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(48));
    }
}
