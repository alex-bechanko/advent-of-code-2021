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

#[cfg(test)]
mod tests {

    use super::*;

    const EXAMPLE: &str = "199\n200\n208\n210\n200\n207\n240\n269\n260\n263";

    #[test]
    fn test_parse_depth() {
        assert_eq!(parse_depth("4"), Ok(4));
        assert_eq!(parse_depth("5"), Ok(5));
        assert_eq!(parse_depth("6"), Ok(6));
        assert_eq!(
            parse_depth("hi"),
            Err(Day01Error::ParseFailure("hi".to_string()))
        );
    }

    #[test]
    fn test_parse() {
        assert_eq!(parse("4\n5\n6"), Ok(vec![4, 5, 6]));
        assert_eq!(
            parse("4\nhi\n6"),
            Err(Day01Error::ParseFailure("hi".to_string()))
        );
    }

    #[test]
    fn test_solution1() {
        let data = &parse(EXAMPLE).unwrap();
        assert_eq!(solution1(data), "7");
    }

    #[test]
    fn test_solution2() {
        let data = &parse(EXAMPLE).unwrap();
        assert_eq!(solution2(data), "5")
    }

    #[test]
    fn test_solutions() {
        assert_eq!(solutions(EXAMPLE), Ok(("7".to_string(), "5".to_string())));
        assert_eq!(
            solutions("invalid"),
            Err("Failed to parse invalid as a mass value".to_string())
        );
    }
}
