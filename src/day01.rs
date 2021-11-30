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

pub enum Day01Error {
    ParseFailure(String)
}

pub fn parse_mass(line: &str) -> Result<u32, Day01Error> {
    line.parse::<u32>()
        .ok()
        .ok_or(Day01Error::ParseFailure(line.to_string()))
}

pub fn parse(data: &str) -> Result<Vec<u32>, Day01Error> {
    data.lines()
        .map(parse_mass)
        .collect::<Result<Vec<u32>, Day01Error>>()
}

pub fn solution1(data: &Vec<u32>) -> String {
    return data.iter()
        .map(|m|m/3-2)
        .sum::<u32>()
        .to_string();
}

pub fn calculate_fuel(mass : u32) -> u32 {
    let mut unnaccounted_mass = mass;
    let mut total_fuel = 0;

    while unnaccounted_mass > 0 {
        let fuel = (unnaccounted_mass / 3).checked_sub(2).unwrap_or(0);
        total_fuel += fuel;
        unnaccounted_mass = fuel;
    }

    return total_fuel;
}

pub fn solution2(data: &Vec<u32>) -> String {
    return data.iter()
        .map(|&f|calculate_fuel(f))
        .sum::<u32>()
        .to_string();
}

pub fn solutions(data: &str) -> Result<(String, String), String> {
    let masses = parse(data)
        .map_err(|Day01Error::ParseFailure(mass)| format!("Failed to parse {} as a mass value", mass))?;

    return Ok((solution1(&masses),solution2(&masses)));
}