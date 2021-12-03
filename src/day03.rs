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

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../examples/day03.txt");
    const EXAMPLE_BITS: u32 = 5;

    #[test]
    fn test_parse_binary() {
        assert_eq!(parse_binary("00100"), Ok(4));
        assert_eq!(
            parse_binary("notbinary"),
            Err(Day03Error::ParseFailure("notbinary".to_string()))
        );
    }

    #[test]
    fn test_parse() {
        assert_eq!(
            parse(EXAMPLE),
            Ok((vec![4, 30, 22, 23, 21, 15, 7, 28, 16, 25, 2, 10], 5))
        );
        assert_eq!(
            parse("notbinary\n00100"),
            Err(Day03Error::ParseFailure("notbinary".to_string()))
        );
    }

    #[test]
    fn test_common_bit() {
        assert_eq!(common_bit(&parse(EXAMPLE).unwrap().0, 0), 0);
        assert_eq!(common_bit(&parse(EXAMPLE).unwrap().0, 1), 1);
        assert_eq!(common_bit(&parse(EXAMPLE).unwrap().0, 2), 1);
        assert_eq!(common_bit(&parse(EXAMPLE).unwrap().0, 3), 0);
        assert_eq!(common_bit(&parse(EXAMPLE).unwrap().0, 4), 1);
    }

    #[test]
    fn test_gamma_rate() {
        assert_eq!(gamma_rate(&parse(EXAMPLE).unwrap().0, EXAMPLE_BITS), 22);
    }

    #[test]
    fn test_epsilon_rate() {
        assert_eq!(epsilon_rate(&parse(EXAMPLE).unwrap().0, EXAMPLE_BITS), 9);
    }

    #[test]
    fn test_solution1() {
        assert_eq!(solution1(&parse(EXAMPLE).unwrap().0, EXAMPLE_BITS), "198")
    }

    #[test]
    fn test_oxygen_generator_rate_criteria() {
        assert_eq!(oxygen_generator_rate_criteria(0b001, 0, 1), true);
        assert_eq!(oxygen_generator_rate_criteria(0b010, 0, 0), true);
        assert_eq!(oxygen_generator_rate_criteria(0b010, 1, 1), true);
        assert_eq!(oxygen_generator_rate_criteria(0b100, 1, 0), true);

        assert_eq!(oxygen_generator_rate_criteria(0b001, 1, 1), false);
        assert_eq!(oxygen_generator_rate_criteria(0b010, 1, 0), false);
        assert_eq!(oxygen_generator_rate_criteria(0b010, 2, 1), false);
        assert_eq!(oxygen_generator_rate_criteria(0b100, 2, 0), false);
    }

    #[test]
    fn test_oxygen_generator_rate() {
        assert_eq!(
            oxygen_generator_rate(&parse(EXAMPLE).unwrap().0, EXAMPLE_BITS),
            Some(23)
        );
    }

    #[test]
    fn test_co2_scrubber_rate_criteria() {
        assert_eq!(co2_scrubber_rate_criteria(0b001, 0, 1), false);
        assert_eq!(co2_scrubber_rate_criteria(0b010, 0, 0), false);
        assert_eq!(co2_scrubber_rate_criteria(0b010, 1, 1), false);
        assert_eq!(co2_scrubber_rate_criteria(0b100, 1, 0), false);

        assert_eq!(co2_scrubber_rate_criteria(0b001, 1, 1), true);
        assert_eq!(co2_scrubber_rate_criteria(0b010, 1, 0), true);
        assert_eq!(co2_scrubber_rate_criteria(0b010, 2, 1), true);
        assert_eq!(co2_scrubber_rate_criteria(0b100, 2, 0), true);
    }

    #[test]
    fn test_co2_scrubber_rate() {
        assert_eq!(
            co2_scrubber_rate(&parse(EXAMPLE).unwrap().0, EXAMPLE_BITS),
            Some(10)
        );
    }

    #[test]
    fn test_solution2() {
        assert_eq!(
            solution2(&parse(EXAMPLE).unwrap().0, EXAMPLE_BITS),
            Ok("230".to_string())
        )
    }

    #[test]
    fn test_solutions() {
        assert_eq!(
            solutions(EXAMPLE),
            Ok(("198".to_string(), "230".to_string()))
        );
    }
}
