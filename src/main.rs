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
mod day01;
mod day02;

use std::io::Read;
use std::usize;

#[derive(Debug)]
enum ApplicationError {
    InvalidDay,
    InvalidDataFile(String),
    OutOfRangeDay(usize),
    SolutionParseError(String),
}

fn cli_args() -> Result<(usize, String), ApplicationError> {
    let day_arg = clap::Arg::new("day")
        .about("Which day's solutions to compute")
        .short('d')
        .long("day")
        .value_name("DAY")
        .required(true);

    let input_arg = clap::Arg::new("input")
        .about("What input data to use when computing the day's solution")
        .short('i')
        .long("input")
        .value_name("FILE")
        .required(false);

    let matches = clap::App::new("Advent of Code 2021")
        .author("Alex Bechanko")
        .about("Compute solutions to Advent of Code 2021")
        .arg(day_arg)
        .arg(input_arg)
        .get_matches();

    let day = matches
        .value_of("day")
        .and_then(|ds| ds.parse::<usize>().ok())
        .ok_or(ApplicationError::InvalidDay)?;

    let default_path = format!("./inputs/day{:02}.txt", day);
    let path = matches
        .value_of("input")
        .map(|inp| inp.to_string())
        .unwrap_or(default_path);

    let data = read_file(path)?;

    return Ok((day, data));
}

fn read_file(path: String) -> Result<String, ApplicationError> {
    let mut file = std::fs::File::open(&path)
        .ok()
        .ok_or(ApplicationError::InvalidDataFile(path.clone()))?;

    let mut data = String::new();
    file.read_to_string(&mut data)
        .ok()
        .ok_or(ApplicationError::InvalidDataFile(path.clone()))?;

    return Ok(data);
}

fn advent_day_solution((day, data): (usize, String)) -> Result<(String, String), ApplicationError> {
    let solutions: Vec<&dyn Fn(&str) -> Result<(String, String), String>> =
        vec![&day01::solutions, &day02::solutions];

    let solution = solutions
        .get(day - 1)
        .ok_or(ApplicationError::OutOfRangeDay(day))?;

    return solution(&data).map_err(|why| ApplicationError::SolutionParseError(why));
}

fn main() {
    match cli_args().and_then(advent_day_solution) {
        Ok((soln1, soln2)) => {
            println!("Solution 1 answer: {}", soln1);
            println!("Solution 2 answer: {}", soln2);
        }
        Err(ApplicationError::SolutionParseError(why)) => {
            println!("{}", why);
        }
        Err(ApplicationError::InvalidDay) => {
            println!("An invalid day was passed as an argument. Day values must be a number between 1 and 25");
        }
        Err(ApplicationError::InvalidDataFile(name)) => {
            println!("An error occurred trying to read from file {}. Please make sure the file exists and is readable.", name);
        }
        Err(ApplicationError::OutOfRangeDay(day)) => {
            println!("The day {} is currently not implemented. Please try a day closer to the beginning.", day);
        }
    }
}
