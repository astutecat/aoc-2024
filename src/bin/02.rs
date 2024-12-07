use nom::{
    character::complete::{digit1, line_ending, space1},
    combinator::map_res,
    multi::{separated_list0, separated_list1},
    IResult,
};

advent_of_code::solution!(2);

type Report = Vec<u32>;

#[derive(Eq, PartialEq)]
enum Status {
    Safe,
    Unsafe,
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Direction {
    Up,
    Down,
}

fn parse_int(input: &str) -> IResult<&str, u32> {
    map_res(digit1, |s: &str| s.parse::<u32>())(input)
}

fn parse_report(input: &str) -> IResult<&str, Report> {
    separated_list1(space1, parse_int)(input)
}

fn parse_input(input: &str) -> IResult<&str, Vec<Report>> {
    separated_list0(line_ending, parse_report)(input)
}

fn abs_diff_ok(a: u32, b: u32) -> bool {
    a.abs_diff(b) > 0 && a.abs_diff(b) <= 3
}

fn direction_ok(direction: Direction, a: u32, b: u32) -> bool {
    match direction {
        Direction::Up if a < b => true,
        Direction::Down if a > b => true,
        _ => false,
    }
}

fn interpret_report(report: &Report) -> Status {
    if report.len() < 2 {
        return Status::Unsafe;
    }

    let direction = match report[..] {
        [a, b, ..] if a < b => Direction::Up,
        [a, b, ..] if a > b => Direction::Down,
        _ => return Status::Unsafe,
    };

    let is_safe = report.windows(2).all(|window| match window {
        [a, b] => direction_ok(direction, *a, *b) && abs_diff_ok(*a, *b),
        _ => false,
    });

    if is_safe {
        Status::Safe
    } else {
        Status::Unsafe
    }
}

fn interpret_report_with_dampen(report: &Report) -> Status {
    if interpret_report(report) == Status::Safe {
        return Status::Safe;
    }

    for i in 0..report.len() {
        let mut reduced_report = report.clone();
        reduced_report.remove(i);
        if interpret_report(&reduced_report) == Status::Safe {
            return Status::Safe;
        }
    }

    Status::Unsafe
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, reports) = parse_input(input).expect("failed to parse input");
    let safe_reports = reports
        .iter()
        .map(|x| interpret_report(x))
        .filter(|o| match o {
            Status::Safe => true,
            _ => false,
        })
        .count();

    Some(safe_reports.try_into().expect("oh no!"))
}

pub fn part_two(input: &str) -> Option<u32> {
    let (_, reports) = parse_input(input).expect("failed to parse input");
    let safe_reports = reports
        .iter()
        .map(|x| interpret_report_with_dampen(x))
        .filter(|o| match o {
            Status::Safe => true,
            _ => false,
        })
        .count();

    Some(safe_reports.try_into().expect("oh no!"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
