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

#[derive(Debug)]
pub enum Day02Error {
    ParseFailure(String)
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Up(u32),
    Down(u32),
    Forward(u32),
}

pub trait Submarine {
    fn move_in_direction(&self, direction:Direction) -> Self;
}
#[derive(Debug, Clone, Copy, Default)]
pub struct Solution1Submarine {
    x :u32,
    y :u32,
}

impl Submarine for Solution1Submarine {
    fn move_in_direction(&self, direction:Direction) -> Self {
        let mut submarine = *self;
        match direction {
            Direction::Up(v) => {
                submarine.y -= v;
            },
            Direction::Down(v) => {
                submarine.y += v;
            },
            Direction::Forward(v) => {
                submarine.x += v;
            }
        }

        return submarine;
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Solution2Submarine {
    x :u32,
    y :u32,
    aim :u32,
}

impl Submarine for Solution2Submarine {
    fn move_in_direction(&self, direction:Direction) -> Self{
        let mut submarine = *self;

        match direction {
            Direction::Up(v) => {
                submarine.aim -= v;
            },
            Direction::Down(v) => {
                submarine.aim += v;
            },
            Direction::Forward(v) => {
                submarine.x += v;
                submarine.y += v * submarine.aim;
            }
        }

        return submarine;
    }
}


pub fn move_submarine(x:u32, y:u32, direction: Direction) -> (u32, u32) {
    match direction {
        Direction::Up(v) => (x, y - v),
        Direction::Down(v) => (x, y + v),
        Direction::Forward(v) => (x + v, y)
    }
}

pub fn parse_direction(line: &str) -> Result<Direction, Day02Error> {
    let data = line.split(" ").collect::<Vec<&str>>();

    let mag = data.get(1)
        .and_then(|s| s.parse::<u32>().ok())
        .ok_or(Day02Error::ParseFailure(line.to_string()))?;

    match data.get(0).map(|&s|s) {
        Some("forward") => Ok(Direction::Forward(mag)),
        Some("up") => Ok(Direction::Up(mag)),
        Some("down") => Ok(Direction::Down(mag)),
        _ => Err(Day02Error::ParseFailure(line.to_string()))
    }
}

pub fn parse(data: &str) -> Result<Vec<Direction>, Day02Error> {
    data.lines()
        .map(parse_direction)
        .collect::<Result<Vec<Direction>, Day02Error>>()
}

pub fn solution1(directions: &Vec<Direction>) -> String {
    let submarine = directions.iter()
        .fold(Solution1Submarine::default(), |submarine, &d| submarine.move_in_direction(d));

    (submarine.x * submarine.y).to_string()
}

pub fn solution2(directions: &Vec<Direction>) -> String {
    let submarine = directions.iter()
        .fold(Solution2Submarine::default(), |submarine, &d| submarine.move_in_direction(d));

    (submarine.x * submarine.y).to_string()
}

pub fn solutions(data: &str) -> Result<(String, String), String> {
    let directions = parse(data)
        .map_err(|Day02Error::ParseFailure(d)| format!("Failed to parse {} as a direction value", d))?;

    return Ok((solution1(&directions),solution2(&directions)));
}