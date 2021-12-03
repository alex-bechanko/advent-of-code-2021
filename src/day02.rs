/*
Advent of Code 2021 solutions
Copyright (C) 2021  Alex Bechanko

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Day02Error {
    ParseFailure(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up(u32),
    Down(u32),
    Forward(u32),
}

pub trait Submarine {
    fn move_in_direction(&self, direction: Direction) -> Self;
}
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Solution1Submarine {
    x: u32,
    y: u32,
}

impl Submarine for Solution1Submarine {
    fn move_in_direction(&self, direction: Direction) -> Self {
        let mut submarine = *self;
        match direction {
            Direction::Up(v) => {
                submarine.y -= v;
            }
            Direction::Down(v) => {
                submarine.y += v;
            }
            Direction::Forward(v) => {
                submarine.x += v;
            }
        }

        return submarine;
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Solution2Submarine {
    x: u32,
    y: u32,
    aim: u32,
}

impl Submarine for Solution2Submarine {
    fn move_in_direction(&self, direction: Direction) -> Self {
        let mut submarine = *self;

        match direction {
            Direction::Up(v) => {
                submarine.aim -= v;
            }
            Direction::Down(v) => {
                submarine.aim += v;
            }
            Direction::Forward(v) => {
                submarine.x += v;
                submarine.y += v * submarine.aim;
            }
        }

        return submarine;
    }
}

pub fn parse_direction(line: &str) -> Result<Direction, Day02Error> {
    let data = line.split(" ").collect::<Vec<&str>>();

    let mag = data
        .get(1)
        .and_then(|s| s.parse::<u32>().ok())
        .ok_or(Day02Error::ParseFailure(line.to_string()))?;

    match data.get(0).map(|&s| s) {
        Some("forward") => Ok(Direction::Forward(mag)),
        Some("up") => Ok(Direction::Up(mag)),
        Some("down") => Ok(Direction::Down(mag)),
        _ => Err(Day02Error::ParseFailure(line.to_string())),
    }
}

pub fn parse(data: &str) -> Result<Vec<Direction>, Day02Error> {
    data.lines()
        .map(parse_direction)
        .collect::<Result<Vec<Direction>, Day02Error>>()
}

pub fn solution1(directions: &Vec<Direction>) -> String {
    let submarine = directions
        .iter()
        .fold(Solution1Submarine::default(), |submarine, &d| {
            submarine.move_in_direction(d)
        });

    (submarine.x * submarine.y).to_string()
}

pub fn solution2(directions: &Vec<Direction>) -> String {
    let submarine = directions
        .iter()
        .fold(Solution2Submarine::default(), |submarine, &d| {
            submarine.move_in_direction(d)
        });

    (submarine.x * submarine.y).to_string()
}

pub fn solutions(data: &str) -> Result<(String, String), String> {
    let directions = parse(data).map_err(|Day02Error::ParseFailure(d)| {
        format!("Failed to parse {} as a direction value", d)
    })?;

    return Ok((solution1(&directions), solution2(&directions)));
}

#[cfg(test)]
mod tests {

    use super::*;

    const EXAMPLE: &str = include_str!("../examples/day02.txt");

    #[test]
    fn test_parse_direction() {
        assert_eq!(parse_direction("forward 5"), Ok(Direction::Forward(5)));
        assert_eq!(
            parse_direction("here? 5"),
            Err(Day02Error::ParseFailure("here? 5".to_string()))
        );
    }

    #[test]
    fn test_parse() {
        assert_eq!(
            parse(EXAMPLE),
            Ok(vec![
                Direction::Forward(5),
                Direction::Down(5),
                Direction::Forward(8),
                Direction::Up(3),
                Direction::Down(8),
                Direction::Forward(2)
            ])
        );
        assert_eq!(
            parse("here? 5\nforward 5"),
            Err(Day02Error::ParseFailure("here? 5".to_string()))
        );
    }

    #[test]
    fn test_solution1_submarine_traitsubmarine() {
        let sub = Solution1Submarine::default().move_in_direction(Direction::Down(1));

        assert_eq!(
            sub.move_in_direction(Direction::Forward(1)),
            Solution1Submarine { x: 1, y: 1 }
        );
        assert_eq!(
            sub.move_in_direction(Direction::Up(1)),
            Solution1Submarine { x: 0, y: 0 }
        );
        assert_eq!(
            sub.move_in_direction(Direction::Down(1)),
            Solution1Submarine { x: 0, y: 2 }
        );
    }

    #[test]
    fn test_solution2_submarine_traitsubmarine() {
        let sub = Solution2Submarine::default().move_in_direction(Direction::Down(1));

        assert_eq!(
            sub.move_in_direction(Direction::Forward(1)),
            Solution2Submarine { x: 1, y: 1, aim: 1 }
        );
        assert_eq!(
            sub.move_in_direction(Direction::Up(1)),
            Solution2Submarine { x: 0, y: 0, aim: 0 }
        );
        assert_eq!(
            sub.move_in_direction(Direction::Down(1)),
            Solution2Submarine { x: 0, y: 0, aim: 2 }
        );
    }

    #[test]
    fn test_solution1() {
        assert_eq!(solution1(&parse(EXAMPLE).unwrap()), "150");
    }

    #[test]
    fn test_solution2() {
        assert_eq!(solution2(&parse(EXAMPLE).unwrap()), "900");
    }

    #[test]
    fn test_solutions() {
        assert_eq!(
            solutions(EXAMPLE),
            Ok(("150".to_string(), "900".to_string()))
        );
        assert_eq!(
            solutions("invalidinput"),
            Err("Failed to parse invalidinput as a direction value".to_string())
        );
    }
}
