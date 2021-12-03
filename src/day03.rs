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

#[derive(Debug, PartialEq, Eq)]
pub enum Day03Error {
    EmptyData,
    ParseFailure(String),
    NoOxygenRate,
    NoCO2Rate,
}

impl std::fmt::Display for Day03Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &*self {
            Day03Error::ParseFailure(why) => {
                write!(f, "Failed to parse {} as a binary number", why)
            }
            Day03Error::EmptyData => {
                write!(f, "Failed to parse empty data")
            }
            Day03Error::NoCO2Rate => {
                write!(f, "Unable to find a CO2 rate")
            }
            Day03Error::NoOxygenRate => {
                write!(f, "Unable to find an Oxygen rate")
            }
        }
    }
}

pub fn parse_binary(line: &str) -> Result<u32, Day03Error> {
    u32::from_str_radix(line, 2)
        .ok()
        .ok_or(Day03Error::ParseFailure(line.to_string()))
}

pub fn parse(data: &str) -> Result<(Vec<u32>, u32), Day03Error> {
    let num_bits = data
        .lines()
        .next()
        .map(|n| n.chars().count())
        .ok_or(Day03Error::EmptyData)? as u32;
    let numbers = data
        .lines()
        .map(parse_binary)
        .collect::<Result<Vec<u32>, Day03Error>>()?;

    return Ok((numbers, num_bits));
}

pub fn common_bit(diagnostic: &Vec<u32>, bit_index: u32) -> u32 {
    let (zeros, ones): (Vec<u32>, Vec<u32>) = diagnostic
        .iter()
        .map(|&n| n & 1 << bit_index)
        .partition(|&n| n == 0);

    if zeros.len() > ones.len() {
        return 0;
    }

    return 1;
}

pub fn gamma_rate(diagnostic: &Vec<u32>, num_bits: u32) -> u32 {
    (0..num_bits)
        .map(|bit_index| common_bit(diagnostic, bit_index) * 1 << bit_index)
        .sum()
}

pub fn epsilon_rate(diagnostic: &Vec<u32>, num_bits: u32) -> u32 {
    (0..num_bits)
        .map(|bit_index| (common_bit(diagnostic, bit_index) ^ 1) * 1 << bit_index)
        .sum()
}

pub fn solution1(diagnostic: &Vec<u32>, num_bits: u32) -> String {
    let answer = gamma_rate(diagnostic, num_bits) * epsilon_rate(diagnostic, num_bits);
    return answer.to_string();
}

pub fn reduce_diagnostics(
    diagnostic: &Vec<u32>,
    num_bits: u32,
    criteria: &dyn Fn(u32, u32, u32) -> bool,
) -> Option<u32> {
    let mut numbers = diagnostic.clone();

    for n in (0..num_bits).rev() {
        let common = common_bit(&numbers, n);
        numbers = numbers
            .iter()
            .filter(|&&num| criteria(num, n, common))
            .map(|&x| x)
            .collect::<Vec<u32>>();

        if numbers.len() == 1 {
            return Some(numbers[0]);
        }
    }

    return None;
}

pub fn oxygen_generator_rate_criteria(num: u32, index: u32, common_bit: u32) -> bool {
    (num & 1 << index) == common_bit << index
}

pub fn oxygen_generator_rate(diagnostic: &Vec<u32>, num_bits: u32) -> Option<u32> {
    reduce_diagnostics(diagnostic, num_bits, &oxygen_generator_rate_criteria)
}

pub fn co2_scrubber_rate_criteria(num: u32, index: u32, common_bit: u32) -> bool {
    (num & 1 << index) != common_bit << index
}

pub fn co2_scrubber_rate(diagnostic: &Vec<u32>, num_bits: u32) -> Option<u32> {
    reduce_diagnostics(diagnostic, num_bits, &co2_scrubber_rate_criteria)
}

pub fn solution2(diagnostic: &Vec<u32>, num_bits: u32) -> Result<String, Day03Error> {
    let oxy = oxygen_generator_rate(diagnostic, num_bits).ok_or(Day03Error::NoOxygenRate)?;
    let co2 = co2_scrubber_rate(diagnostic, num_bits).ok_or(Day03Error::NoCO2Rate)?;

    return Ok((oxy * co2).to_string());
}

pub fn solutions(data: &str) -> Result<(String, String), String> {
    let (diagnostic, num_bits) = parse(data).map_err(|e| e.to_string())?;
    let soln1 = solution1(&diagnostic, num_bits);
    let soln2 = match solution2(&diagnostic, num_bits) {
        Ok(ans) => ans,
        Err(why) => why.to_string(),
    };

    return Ok((soln1, soln2));
}
