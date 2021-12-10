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
pub enum Day04Error {
    ParseBoardFailure(String, String),
    ParseLotsFailure(String),
    MissingInputData(String),
}

impl std::fmt::Display for Day04Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &*self {
            Day04Error::ParseBoardFailure(board, piece) => {
                write!(
                    f,
                    "Unable to parse board '{}'. Failed to parse '{}'",
                    board, piece
                )
            }
            Day04Error::ParseLotsFailure(number) => {
                write!(
                    f,
                    "Failed to parse number from the pulled numbers input data: {}",
                    number
                )
            }
            Day04Error::MissingInputData(why) => {
                write!(f, "Missing input data: {}", why)
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Day04 {
    numbers: Vec<u32>,
    boards: Vec<Vec<u32>>,
}

pub fn parse_bingo_board(data: &str) -> Result<Vec<u32>, Day04Error> {
    data.trim()
        .split(&[' ', '\n'][..])
        .filter(|&s| s != "")
        .map(|elem| {
            elem.trim()
                .parse()
                .ok()
                .ok_or(Day04Error::ParseBoardFailure(
                    data.to_string(),
                    elem.to_string(),
                ))
        })
        .collect::<Result<Vec<u32>, Day04Error>>()
}

pub fn is_bingo(board: &Vec<u32>, numbers: &std::collections::HashSet<u32>) -> bool {
    // check rows and columns
    for i in 0..5 {
        let mut row = std::collections::HashSet::new();
        let mut col = std::collections::HashSet::new();
        for j in 0..5 {
            row.insert(board[i * 5 + j]);
            col.insert(board[5 * j + i]);
        }

        if row.is_subset(numbers) {
            return true;
        } else if col.is_subset(numbers) {
            return true;
        }
    }

    return false;
}

pub fn parse(data: &str) -> Result<Day04, Day04Error> {
    let (numbers, boards) = data.split_once("\n\n").ok_or(Day04Error::MissingInputData(
        "Unable to find number lots".to_string(),
    ))?;

    let numbers = numbers
        .split(",")
        .map(|elem| {
            elem.trim()
                .parse()
                .ok()
                .ok_or(Day04Error::ParseLotsFailure(elem.to_string()))
        })
        .collect::<Result<Vec<u32>, _>>()?;

    let boards = boards
        .split("\n\n")
        .map(|board| parse_bingo_board(board))
        .collect::<Result<Vec<Vec<u32>>, Day04Error>>()?;

    return Ok(Day04 { numbers, boards });
}

pub fn solution1(data: &Day04) -> String {
    let mut played_numbers = std::collections::HashSet::new();

    for num in data.numbers.iter() {
        played_numbers.insert(*num);

        if let Some(winner) = data
            .boards
            .iter()
            .filter(|&b| is_bingo(b, &played_numbers))
            .next()
        {
            let unmarked: u32 = winner
                .clone()
                .into_iter()
                .collect::<std::collections::HashSet<u32>>()
                .difference(&played_numbers)
                .sum();

            return (unmarked * num).to_string();
        }
    }

    return "No solution found".to_string();
}

pub fn solution2(data: &Day04) -> String {
    let mut played_numbers = std::collections::HashSet::new();

    let mut winners = std::collections::HashSet::new();

    for num in data.numbers.iter() {
        played_numbers.insert(*num);

        for winner in data.boards.iter().filter(|&b| is_bingo(b, &played_numbers)) {
            if !winners.contains(&winner) && winners.len() + 1 == data.boards.len() {
                let unmarked: u32 = winner
                    .clone()
                    .into_iter()
                    .collect::<std::collections::HashSet<u32>>()
                    .difference(&played_numbers)
                    .sum();

                return (unmarked * num).to_string();
            } else if !winners.contains(&winner) {
                winners.insert(winner);
            }
        }
    }

    return "No solution found".to_string();
}

pub fn solutions(data: &str) -> Result<(String, String), String> {
    let day = parse(data).map_err(|e| e.to_string())?;
    let soln1 = solution1(&day);
    let soln2 = solution2(&day);

    return Ok((soln1, soln2));
}

#[cfg(test)]
mod tests {

    use super::*;
    const EXAMPLE: &str = include_str!("../examples/day04.txt");

    #[test]
    fn test_parse_bingo_board() {
        let data = "22 13 17 11  0\n8  2 23  4 24\n21  9 14 16  7\n6 10  3 18  5\n1 12 20 15 19";
        let expected: Vec<u32> = vec![
            22, 13, 17, 11, 0, 8, 2, 23, 4, 24, 21, 9, 14, 16, 7, 6, 10, 3, 18, 5, 1, 12, 20, 15,
            19,
        ];
        let actual = parse_bingo_board(data);

        assert_eq!(actual, Ok(expected));
    }

    #[test]
    fn test_parse() {
        let expected = Ok(Day04 {
            numbers: vec![
                7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8,
                19, 3, 26, 1,
            ],
            boards: vec![
                vec![
                    22, 13, 17, 11, 0, 8, 2, 23, 4, 24, 21, 9, 14, 16, 7, 6, 10, 3, 18, 5, 1, 12,
                    20, 15, 19,
                ],
                vec![
                    3, 15, 0, 2, 22, 9, 18, 13, 17, 5, 19, 8, 7, 25, 23, 20, 11, 10, 24, 4, 14, 21,
                    16, 12, 6,
                ],
                vec![
                    14, 21, 17, 24, 4, 10, 16, 15, 9, 19, 18, 8, 23, 26, 20, 22, 11, 13, 6, 5, 2,
                    0, 12, 3, 7,
                ],
            ],
        });
        assert_eq!(parse(EXAMPLE), expected);
    }

    #[test]
    fn test_solution1() {
        let data = parse(EXAMPLE).unwrap();
        let expected = "4512";
        assert_eq!(solution1(&data), expected);
    }

    #[test]
    fn test_solution2() {
        let data = parse(EXAMPLE).unwrap();
        let expected = "1924";
        assert_eq!(solution2(&data), expected);
    }

    #[test]
    fn test_is_bingo() {
        let tests: Vec<(Vec<u32>, std::collections::HashSet<u32>, bool)> = vec![
            // valid row bingos
            (
                vec![
                    22, 13, 17, 11, 0, 8, 2, 23, 4, 24, 21, 9, 14, 16, 7, 6, 10, 3, 18, 5, 1, 12,
                    20, 15, 19,
                ],
                vec![22, 13, 17, 11, 0].into_iter().collect(),
                true,
            ),
            (
                vec![
                    22, 13, 17, 11, 0, 8, 2, 23, 4, 24, 21, 9, 14, 16, 7, 6, 10, 3, 18, 5, 1, 12,
                    20, 15, 19,
                ],
                vec![8, 2, 23, 4, 24].into_iter().collect(),
                true,
            ),
            (
                vec![
                    22, 13, 17, 11, 0, 8, 2, 23, 4, 24, 21, 9, 14, 16, 7, 6, 10, 3, 18, 5, 1, 12,
                    20, 15, 19,
                ],
                vec![21, 9, 14, 16, 7].into_iter().collect(),
                true,
            ),
            (
                vec![
                    22, 13, 17, 11, 0, 8, 2, 23, 4, 24, 21, 9, 14, 16, 7, 6, 10, 3, 18, 5, 1, 12,
                    20, 15, 19,
                ],
                vec![6, 10, 3, 18, 5].into_iter().collect(),
                true,
            ),
            (
                vec![
                    22, 13, 17, 11, 0, 8, 2, 23, 4, 24, 21, 9, 14, 16, 7, 6, 10, 3, 18, 5, 1, 12,
                    20, 15, 19,
                ],
                vec![1, 12, 20, 15, 19].into_iter().collect(),
                true,
            ),
            // valid col bingos
            (
                vec![
                    22, 13, 17, 11, 0, 8, 2, 23, 4, 24, 21, 9, 14, 16, 7, 6, 10, 3, 18, 5, 1, 12,
                    20, 15, 19,
                ],
                vec![22, 8, 21, 6, 1].into_iter().collect(),
                true,
            ),
            (
                vec![
                    22, 13, 17, 11, 0, 8, 2, 23, 4, 24, 21, 9, 14, 16, 7, 6, 10, 3, 18, 5, 1, 12,
                    20, 15, 19,
                ],
                vec![13, 2, 9, 10, 12].into_iter().collect(),
                true,
            ),
            (
                vec![
                    22, 13, 17, 11, 0, 8, 2, 23, 4, 24, 21, 9, 14, 16, 7, 6, 10, 3, 18, 5, 1, 12,
                    20, 15, 19,
                ],
                vec![17, 23, 14, 3, 20].into_iter().collect(),
                true,
            ),
            (
                vec![
                    22, 13, 17, 11, 0, 8, 2, 23, 4, 24, 21, 9, 14, 16, 7, 6, 10, 3, 18, 5, 1, 12,
                    20, 15, 19,
                ],
                vec![11, 4, 16, 18, 15].into_iter().collect(),
                true,
            ),
            (
                vec![
                    22, 13, 17, 11, 0, 8, 2, 23, 4, 24, 21, 9, 14, 16, 7, 6, 10, 3, 18, 5, 1, 12,
                    20, 15, 19,
                ],
                vec![0, 24, 7, 5, 19].into_iter().collect(),
                true,
            ),
            // valid diagonal bingos don't count
            (
                vec![
                    22, 13, 17, 11, 0, 8, 2, 23, 4, 24, 21, 9, 14, 16, 7, 6, 10, 3, 18, 5, 1, 12,
                    20, 15, 19,
                ],
                vec![22, 2, 14, 18, 19].into_iter().collect(),
                false,
            ),
            (
                vec![
                    22, 13, 17, 11, 0, 8, 2, 23, 4, 24, 21, 9, 14, 16, 7, 6, 10, 3, 18, 5, 1, 12,
                    20, 15, 19,
                ],
                vec![0, 4, 14, 10, 1].into_iter().collect(),
                false,
            ),
            (
                vec![
                    14, 21, 17, 24, 4, 10, 16, 15, 9, 19, 18, 8, 23, 26, 20, 22, 11, 13, 6, 5, 2,
                    0, 12, 3, 7,
                ],
                vec![4, 2, 7, 9, 5, 23, 11, 17].into_iter().collect(),
                false,
            ),
        ];

        for (board, numbers, expected) in tests {
            assert_eq!(
                is_bingo(&board, &numbers),
                expected,
                "board={:?} and numbers={:?}",
                board,
                numbers
            )
        }
    }
}
