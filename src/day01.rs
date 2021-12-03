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

#[cfg(test)]
mod tests;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Day01Error {
    ParseFailure(String),
}

pub fn parse_depth(line: &str) -> Result<u32, Day01Error> {
    line.parse::<u32>()
        .ok()
        .ok_or(Day01Error::ParseFailure(line.to_string()))
}

pub fn parse(data: &str) -> Result<Vec<u32>, Day01Error> {
    data.lines()
        .map(parse_depth)
        .collect::<Result<Vec<u32>, Day01Error>>()
}

pub fn solution1(data: &Vec<u32>) -> String {
    return data
        .iter()
        .zip(data.iter().skip(1))
        .filter(|(x, y)| x < y)
        .count()
        .to_string();
}

pub fn solution2(data: &Vec<u32>) -> String {
    let slide = data
        .iter()
        .zip(data.iter().skip(1))
        .zip(data.iter().skip(2))
        .map(|((x, y), z)| x + y + z);

    return slide
        .clone()
        .zip(slide.skip(1))
        .filter(|(x, y)| x < y)
        .count()
        .to_string();
}

pub fn solutions(data: &str) -> Result<(String, String), String> {
    let masses = parse(data).map_err(|Day01Error::ParseFailure(mass)| {
        format!("Failed to parse {} as a mass value", mass)
    })?;

    return Ok((solution1(&masses), solution2(&masses)));
}
