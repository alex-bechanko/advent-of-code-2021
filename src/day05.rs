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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Line {
    start_x: i32,
    start_y: i32,
    end_x: i32,
    end_y: i32,
}

impl Line {
    pub fn new(start_x: i32, start_y: i32, end_x: i32, end_y: i32) -> Option<Line> {
        let l = Line {
            start_x,
            start_y,
            end_x,
            end_y,
        };

        if !is_vertical(&l) && !is_horizontal(&l) && !is_diagonal(&l) {
            None
        } else {
            Some(l)
        }
    }
}

impl std::str::FromStr for Line {
    type Err = Day05Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s
            .trim()
            .split_once("->")
            .ok_or(Day05Error::ParseLineFailure(s.to_string()))?;
        let (start_x, start_y) =
            start
                .trim()
                .split_once(",")
                .ok_or(Day05Error::ParseCoordFailure(
                    s.to_string(),
                    start.to_string(),
                ))?;

        let start_x = start_x
            .trim()
            .parse()
            .ok()
            .ok_or(Day05Error::ParseNumberFailure(
                s.to_string(),
                start.to_string(),
                start_x.to_string(),
            ))?;
        let start_y = start_y
            .trim()
            .parse()
            .ok()
            .ok_or(Day05Error::ParseNumberFailure(
                s.to_string(),
                start.to_string(),
                start_y.to_string(),
            ))?;

        let (end_x, end_y) = end
            .trim()
            .split_once(",")
            .ok_or(Day05Error::ParseCoordFailure(
                s.to_string(),
                end.to_string(),
            ))?;
        let end_x = end_x
            .trim()
            .parse()
            .ok()
            .ok_or(Day05Error::ParseNumberFailure(
                s.to_string(),
                end.to_string(),
                end_x.to_string(),
            ))?;
        let end_y = end_y
            .trim()
            .parse()
            .ok()
            .ok_or(Day05Error::ParseNumberFailure(
                s.to_string(),
                end.to_string(),
                end_y.to_string(),
            ))?;

        return Line::new(start_x, start_y, end_x, end_y)
            .ok_or(Day05Error::InvalidLine(s.to_string()));
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Day05Error {
    ParseLineFailure(String),
    ParseCoordFailure(String, String),
    ParseNumberFailure(String, String, String),
    InvalidLine(String),
}

impl std::fmt::Display for Day05Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &*self {
            Day05Error::ParseLineFailure(line) => {
                write!(f, "Failed to parse line: {}", line)
            }
            Day05Error::ParseCoordFailure(line, coord) => {
                write!(
                    f,
                    "Failed to parse coordinate '{}' in line: {}",
                    coord, line
                )
            }
            Day05Error::ParseNumberFailure(line, coord, number) => {
                write!(
                    f,
                    "Failed to parse number '{}' in coord '{}' in line: {}",
                    number, coord, line
                )
            }
            Day05Error::InvalidLine(line) => {
                write!(
                    f,
                    "Failed to construct line, possibly invalid orientation: {}",
                    line
                )
            }
        }
    }
}

pub fn parse(data: &str) -> Result<Vec<Line>, Day05Error> {
    data.lines()
        .map(|s| s.parse())
        .collect::<Result<Vec<Line>, Day05Error>>()
}

pub fn is_vertical(line: &Line) -> bool {
    line.start_x == line.end_x
}

pub fn is_horizontal(line: &Line) -> bool {
    line.start_y == line.end_y
}

pub fn is_diagonal(line: &Line) -> bool {
    !is_horizontal(line) && !is_vertical(line)
}

pub fn line_to_points(line: &Line) -> std::collections::HashSet<(i32, i32)> {
    let xs: Box<dyn Iterator<Item = i32>> = if line.start_x < line.end_x {
        Box::new((line.start_x..=line.end_x).into_iter())
    } else if line.start_x > line.end_x {
        Box::new((line.end_x..=line.start_x).rev().into_iter())
    } else {
        Box::new(std::iter::repeat(line.start_x))
    };

    let ys: Box<dyn Iterator<Item = i32>> = if line.start_y < line.end_y {
        Box::new((line.start_y..=line.end_y).into_iter())
    } else if line.start_y > line.end_y {
        Box::new((line.end_y..=line.start_y).rev().into_iter())
    } else {
        Box::new(std::iter::repeat(line.start_y))
    };

    xs.zip(ys).collect()
}

pub fn solution1(data: &Vec<Line>) -> String {
    let pt_sets = data
        .iter()
        .filter(|&l| is_horizontal(l) || is_vertical(l))
        .map(|l| line_to_points(&l));

    let mut pts = std::collections::HashMap::new();
    for pt_set in pt_sets {
        for pt in pt_set {
            pts.insert(pt, pts.get(&pt).unwrap_or(&0) + 1);
        }
    }

    let soln = pts
        .into_iter()
        .filter(|&(_, total)| total > 1)
        .map(|(_, total)| total)
        .count();

    return soln.to_string();
}

pub fn solution2(data: &Vec<Line>) -> String {
    let pt_sets = data
        .iter()
        .filter(|&l| is_horizontal(l) || is_vertical(l) || is_diagonal(l))
        .map(|l| line_to_points(&l));

    let mut pts = std::collections::HashMap::new();
    for pt_set in pt_sets {
        for pt in pt_set {
            pts.insert(pt, pts.get(&pt).unwrap_or(&0) + 1);
        }
    }

    let soln = pts
        .into_iter()
        .filter(|&(_, total)| total > 1)
        .map(|(_, total)| total)
        .count();

    return soln.to_string();
}

pub fn solutions(data: &str) -> Result<(String, String), String> {
    let day = parse(data).map_err(|e| e.to_string())?;
    let soln1 = solution1(&day);
    let soln2 = solution2(&day);

    return Ok((soln1, soln2));
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;
    const EXAMPLE: &str = include_str!("../examples/day05.txt");

    #[test]
    fn test_solution2() {
        let test = parse(EXAMPLE).unwrap();
        assert_eq!(solution2(&test), "12")
    }

    #[test]
    fn test_solution1() {
        let test = parse("0,9 -> 5,9\n0,9 -> 2,9").unwrap();
        assert_eq!(solution1(&test), "3");

        let test = parse(EXAMPLE).unwrap();
        assert_eq!(solution1(&test), "5")
    }

    #[test]
    fn test_line_to_points() {
        let tests: Vec<(&str, Vec<(i32, i32)>)> = vec![
            (
                "0,9 -> 5,9",
                vec![(0, 9), (1, 9), (2, 9), (3, 9), (4, 9), (5, 9)],
            ),
            (
                "8,0 -> 0,8",
                vec![
                    (8, 0),
                    (7, 1),
                    (6, 2),
                    (5, 3),
                    (4, 4),
                    (3, 5),
                    (2, 6),
                    (1, 7),
                    (0, 8),
                ],
            ),
            (
                "9,4 -> 3,4",
                vec![(9, 4), (8, 4), (7, 4), (6, 4), (5, 4), (4, 4), (3, 4)],
            ),
            ("2,2 -> 2,1", vec![(2, 2), (2, 1)]),
            ("7,0 -> 7,4", vec![(7, 0), (7, 1), (7, 2), (7, 3), (7, 4)]),
            ("6,4 -> 2,0", vec![(6, 4), (5, 3), (4, 2), (3, 1), (2, 0)]),
            ("0,9 -> 2,9", vec![(0, 9), (1, 9), (2, 9)]),
            ("3,4 -> 1,4", vec![(3, 4), (2, 4), (1, 4)]),
            (
                "0,0 -> 8,8",
                vec![
                    (0, 0),
                    (1, 1),
                    (2, 2),
                    (3, 3),
                    (4, 4),
                    (5, 5),
                    (6, 6),
                    (7, 7),
                    (8, 8),
                ],
            ),
            ("5,5 -> 8,2", vec![(5, 5), (6, 4), (7, 3), (8, 2)]),
        ];

        for (teststr, expected) in tests {
            let test = Line::from_str(teststr).ok().unwrap();
            let expected = std::collections::HashSet::from_iter(expected);
            assert_eq!(
                line_to_points(&test),
                expected,
                "'{}' did not return correct points",
                teststr
            );
        }
    }

    #[test]
    fn test_parse() {
        let expected = Ok(vec![
            Line {
                start_x: 0,
                start_y: 9,
                end_x: 5,
                end_y: 9,
            },
            Line {
                start_x: 8,
                start_y: 0,
                end_x: 0,
                end_y: 8,
            },
            Line {
                start_x: 9,
                start_y: 4,
                end_x: 3,
                end_y: 4,
            },
            Line {
                start_x: 2,
                start_y: 2,
                end_x: 2,
                end_y: 1,
            },
            Line {
                start_x: 7,
                start_y: 0,
                end_x: 7,
                end_y: 4,
            },
            Line {
                start_x: 6,
                start_y: 4,
                end_x: 2,
                end_y: 0,
            },
            Line {
                start_x: 0,
                start_y: 9,
                end_x: 2,
                end_y: 9,
            },
            Line {
                start_x: 3,
                start_y: 4,
                end_x: 1,
                end_y: 4,
            },
            Line {
                start_x: 0,
                start_y: 0,
                end_x: 8,
                end_y: 8,
            },
            Line {
                start_x: 5,
                start_y: 5,
                end_x: 8,
                end_y: 2,
            },
        ]);
        assert_eq!(parse(EXAMPLE), expected);
    }

    #[test]
    fn test_fromstr_line() {
        let test = "242,601 -> 242,18";
        let expected = Ok(Line {
            start_x: 242,
            start_y: 601,
            end_x: 242,
            end_y: 18,
        });
        assert_eq!(test.parse::<Line>(), expected);
    }

    #[test]
    fn test_is_vertical() {
        assert_eq!(
            is_vertical(&Line {
                start_x: 0,
                start_y: 0,
                end_x: 0,
                end_y: 1
            }),
            true
        );
        assert_eq!(
            is_vertical(&Line {
                start_x: 1,
                start_y: 0,
                end_x: 1,
                end_y: 1
            }),
            true
        );
        assert_eq!(
            is_vertical(&Line {
                start_x: 1,
                start_y: 1,
                end_x: 0,
                end_y: 1
            }),
            false
        );
        assert_eq!(
            is_vertical(&Line {
                start_x: 3,
                start_y: 0,
                end_x: 2,
                end_y: 1
            }),
            false
        );
    }

    #[test]
    fn test_is_horizontal() {
        assert_eq!(
            is_horizontal(&Line {
                start_x: 0,
                start_y: 1,
                end_x: 1,
                end_y: 1
            }),
            true
        );
        assert_eq!(
            is_horizontal(&Line {
                start_x: 0,
                start_y: 2,
                end_x: 1,
                end_y: 2
            }),
            true
        );
        assert_eq!(
            is_horizontal(&Line {
                start_x: 0,
                start_y: 2,
                end_x: 1,
                end_y: 1
            }),
            false
        );
        assert_eq!(
            is_horizontal(&Line {
                start_x: 0,
                start_y: 2,
                end_x: 1,
                end_y: 3
            }),
            false
        );
    }
}
