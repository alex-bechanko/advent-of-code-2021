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

use std::fmt::Debug;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct LanternFish(usize);

impl LanternFish {
    pub fn new(spawn_day: usize) -> Option<LanternFish> {
        if spawn_day > 8 {
            None
        } else {
            Some(LanternFish(spawn_day))
        }
    }
}

#[derive(Debug, Clone)]
pub struct LanternFishSchool {
    spawn_timer: [u64; 9],
}

const TIMER_NEW_LANTERN_FISH: usize = 8;
const TIMER_OLD_LANTERN_FISH: usize = 6;

impl LanternFishSchool {
    pub fn new() -> Self {
        return LanternFishSchool {
            spawn_timer: [0; 9],
        };
    }

    pub fn add(&mut self, fish: LanternFish) {
        self.spawn_timer[fish.0] += 1;
    }

    pub fn size(&self) -> u64 {
        self.spawn_timer.iter().sum()
    }

    pub fn next_generation(&self) -> LanternFishSchool {
        let mut school = [0; 9];

        for (timer, &num_fish) in self.spawn_timer.iter().enumerate() {
            if timer == 0 {
                school[TIMER_NEW_LANTERN_FISH] += num_fish;
                school[TIMER_OLD_LANTERN_FISH] += num_fish;
            } else {
                school[timer - 1] += num_fish;
            }
        }

        LanternFishSchool {
            spawn_timer: school,
        }
    }
}

impl std::fmt::Display for LanternFishSchool {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.spawn_timer.fmt(f)
    }
}

impl FromIterator<LanternFish> for LanternFishSchool {
    fn from_iter<I: IntoIterator<Item = LanternFish>>(iter: I) -> Self {
        let mut school = LanternFishSchool::new();
        for fish in iter {
            school.add(fish);
        }

        school
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Day06Error {
    LanternFishParseFailure(String),
}

impl std::fmt::Display for Day06Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &*self {
            Day06Error::LanternFishParseFailure(fish) => {
                write!(f, "Unable to parse fish '{}'.", fish)
            }
        }
    }
}

pub fn parse(data: &str) -> Result<Vec<LanternFish>, Day06Error> {
    data.split(",")
        .map(|fish| {
            fish.parse::<usize>()
                .ok()
                .and_then(LanternFish::new)
                .ok_or(Day06Error::LanternFishParseFailure(fish.to_string()))
        })
        .collect::<Result<Vec<LanternFish>, Day06Error>>()
}

pub fn solution1(data: &Vec<LanternFish>) -> String {
    let mut school: LanternFishSchool = data.clone().into_iter().collect();
    for i in 1..=80 {
        school = school.next_generation();
    }

    school.size().to_string()
}

pub fn solution2(data: &Vec<LanternFish>) -> String {
    let mut school: LanternFishSchool = data.clone().into_iter().collect();
    for i in 1..=256 {
        school = school.next_generation();
    }

    school.size().to_string()
}

pub fn solutions(data: &str) -> Result<(String, String), String> {
    let day = parse(data).map_err(|e| e.to_string())?;
    let soln1 = solution1(&day);
    let soln2 = solution2(&day);

    Ok((soln1.to_string(), soln2.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("../examples/day06.txt");

    #[test]
    fn test_solution2() {
        let data = parse(EXAMPLE).unwrap();
        let expected = "26984457539";
        assert_eq!(solution2(&data), expected);
    }

    #[test]
    fn test_solution1() {
        let data = parse(EXAMPLE).unwrap();
        let expected = "5934";
        assert_eq!(solution1(&data), expected);
    }

    #[test]
    fn test_lanterfishschool_next_generation() {
        let mut school: LanternFishSchool = parse(EXAMPLE).unwrap().into_iter().collect();
        let mut ancestor = school.clone();

        let sizes = vec![
            5, 5, 6, 7, 9, 10, 10, 10, 10, 11, 12, 15, 17, 19, 20, 20, 21, 22, 26,
        ];

        for (day, expected_size) in sizes.into_iter().enumerate() {
            assert_eq!(
                school.size(),
                expected_size,
                "Day {} size did not match up. Ancestors: {}, Current: {}",
                day,
                ancestor,
                school
            );

            ancestor = school.clone();
            school = school.next_generation();
        }
    }

    #[test]
    fn test_lanterfishschool_from_iter() {
        let fish = vec![LanternFish(0), LanternFish(0), LanternFish(1)];
        let school = LanternFishSchool::from_iter(fish.into_iter());
        assert_eq!(school.size(), 3)
    }

    #[test]
    fn test_lanterfishschool_size() {
        let mut school = LanternFishSchool::new();
        assert_eq!(school.size(), 0);

        school.add(LanternFish(0));
        school.add(LanternFish(0));
        school.add(LanternFish(0));
        assert_eq!(school.size(), 3);

        school.add(LanternFish(1));
        school.add(LanternFish(1));
        school.add(LanternFish(1));
        assert_eq!(school.size(), 6);
    }

    #[test]
    fn test_lanterfishschool_add() {
        let mut school = LanternFishSchool::new();
        school.add(LanternFish(0));
        assert_eq!(school.spawn_timer.get(0), Some(&1));

        school.add(LanternFish(0));
        assert_eq!(school.spawn_timer.get(0), Some(&2));
    }

    #[test]
    fn test_lanterfishschool_new() {
        let school = LanternFishSchool::new();

        for i in 0..=8 {
            assert_eq!(school.spawn_timer.get(i), Some(&0));
        }

        for i in 9..=50 {
            assert_eq!(school.spawn_timer.get(i), None);
        }
    }

    #[test]
    fn test_lanternfish_new() {
        for i in 0..=8 {
            assert_eq!(LanternFish::new(i), Some(LanternFish(i)));
        }

        for i in 9..=50 {
            assert_eq!(LanternFish::new(i), None);
        }
    }

    #[test]
    fn test_parse() {
        assert_eq!(
            parse(EXAMPLE),
            Ok(vec![3, 4, 3, 1, 2]
                .into_iter()
                .map(|x| LanternFish::new(x).unwrap())
                .collect())
        )
    }
}
