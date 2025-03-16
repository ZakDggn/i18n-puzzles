use std::collections::HashMap;
use std::fs;

type Date = (u8, u8, u8);

fn parse_date(date: &str) -> Date {
    let nums: Vec<u8> = date.split('-').map(|n| n.parse().unwrap()).collect();
    (nums[0], nums[1], nums[2])
}

fn parse(input: &str) -> HashMap<&str, Vec<Date>> {
    let mut name_to_dates = HashMap::new();
    for line in input.lines() {
        let (date, names) = line.split_once(':').unwrap();
        let date = parse_date(date);
        for name in names.split(',').map(str::trim) {
            name_to_dates.entry(name).or_insert(Vec::new()).push(date);
        }
    }
    name_to_dates
}

fn is_valid_dmy((d, m, y): Date) -> bool {
    d != 0
        && (m == 2 && d <= 29
            || [1, 3, 5, 7, 8, 10, 12].contains(&m) && d <= 31
            || [4, 6, 9, 11].contains(&m) && d <= 30)
        && !(d == 29 && m == 2 && y % 4 != 0)
}

fn is_valid_mdy((m, d, y): Date) -> bool {
    is_valid_dmy((d, m, y))
}

fn is_valid_ymd((y, m, d): Date) -> bool {
    is_valid_dmy((d, m, y))
}

fn is_valid_ydm((y, d, m): Date) -> bool {
    is_valid_dmy((d, m, y))
}

fn solution(input: &str) -> String {
    let name_to_dates = parse(input);
    let mut names = Vec::new();
    for (&name, dates) in &name_to_dates {
        if dates.iter().all(|&date| is_valid_dmy(date)) && dates.contains(&(11, 9, 1))
            || dates.iter().all(|&date| is_valid_mdy(date)) && dates.contains(&(9, 11, 1))
            || dates.iter().all(|&date| is_valid_ymd(date)) && dates.contains(&(1, 9, 11))
            || dates.iter().all(|&date| is_valid_ydm(date)) && dates.contains(&(1, 11, 9))
        {
            names.push(name);
        }
    }
    names.sort_unstable();
    names.join(" ")
}

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let answer = solution(&input);
    println!("{answer}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = fs::read_to_string("test-input").unwrap();
        assert_eq!(solution(&input), "Margot Peter");
    }
}
