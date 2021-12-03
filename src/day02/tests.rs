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

const EXAMPLE: &str = "forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2";

#[test]
fn test_parse_direction() {
    assert_eq!(parse_direction("forward 5"), Ok(Direction::Forward(5))); 
    assert_eq!(parse_direction("here? 5"), Err(Day02Error::ParseFailure("here? 5".to_string()))); 
}

#[test]
fn test_parse() {
    assert_eq!(parse(EXAMPLE), Ok(vec![Direction::Forward(5), Direction::Down(5), Direction::Forward(8), Direction::Up(3), Direction::Down(8), Direction::Forward(2)])); 
    assert_eq!(parse("here? 5\nforward 5"), Err(Day02Error::ParseFailure("here? 5".to_string()))); 
}

#[test]
fn test_solution1_submarine_traitsubmarine() {
    let sub = Solution1Submarine::default().move_in_direction(Direction::Down(1));

    assert_eq!(sub.move_in_direction(Direction::Forward(1)), Solution1Submarine{x:1,y:1});
    assert_eq!(sub.move_in_direction(Direction::Up(1)), Solution1Submarine{x:0,y:0});
    assert_eq!(sub.move_in_direction(Direction::Down(1)), Solution1Submarine{x:0,y:2});
}

#[test]
fn test_solution2_submarine_traitsubmarine() {
    let sub = Solution2Submarine::default().move_in_direction(Direction::Down(1));

    assert_eq!(sub.move_in_direction(Direction::Forward(1)), Solution2Submarine{x:1,y:1,aim:1});
    assert_eq!(sub.move_in_direction(Direction::Up(1)), Solution2Submarine{x:0,y:0,aim:0});
    assert_eq!(sub.move_in_direction(Direction::Down(1)), Solution2Submarine{x:0,y:0,aim:2});
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
    assert_eq!(solutions(EXAMPLE), Ok(("150".to_string(), "900".to_string())));
    assert_eq!(solutions("invalidinput"), Err("Failed to parse invalidinput as a direction value".to_string()));

}