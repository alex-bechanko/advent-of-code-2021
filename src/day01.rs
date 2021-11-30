

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