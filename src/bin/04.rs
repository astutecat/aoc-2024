use pathfinding::prelude::Grid;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha0, line_ending},
    multi::{many0_count, separated_list1},
    IResult,
};

advent_of_code::solution!(4);

pub fn parse_input_to_lines(input: &str) -> IResult<&str, Vec<&str>> {
    separated_list1(line_ending, alpha0)(input)
}

pub fn count_instances_horizontal(input: &str) -> IResult<&str, usize> {
    let maybe_xmas = (tag("XMAS"), tag("SAMX"));
    many0_count(alt(maybe_xmas))(input)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, lines) = parse_input_to_lines(input).unwrap();

    let total_lines = lines.iter().count();
    let line_length = lines.first().unwrap().chars().count();

    let mut g = Grid::new(total_lines, line_length);
    g.enable_diagonal_mode();
    g.fill();
    g.iter().for_each(|v| println!("{:?}", g.neighbours(v)));

    None
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
