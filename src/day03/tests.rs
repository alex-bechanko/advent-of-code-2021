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

use super::*;

const EXAMPLE: &str =
    "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010";
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
