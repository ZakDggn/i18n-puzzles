use chrono::{Offset, Timelike};
use chrono_tz::America::{Halifax, Santiago};
use std::fs;

fn solution(input: &str) -> usize {
    input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            let mut words = line.split_whitespace();
            let timestamp = words.next().unwrap();
            let correct_minutes = words.next().unwrap().parse().unwrap();
            let wrong_minutes = words.next().unwrap().parse().unwrap();
            let dt_fixed = chrono::DateTime::parse_from_rfc3339(timestamp).unwrap();
            let dt_halifax = dt_fixed.with_timezone(&Halifax);
            let dt_santiago = dt_fixed.with_timezone(&Santiago);
            let mut dt = if *dt_fixed.offset() == dt_halifax.offset().fix() {
                dt_halifax
            } else {
                dt_santiago
            };
            dt -= chrono::TimeDelta::minutes(wrong_minutes);
            dt += chrono::TimeDelta::minutes(correct_minutes);
            dt.time().hour() as usize * (i + 1)
        })
        .sum()
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
        assert_eq!(solution(&input), 866);
    }
}
