use chrono::{DateTime, NaiveDateTime, Utc};
use chrono_tz::Tz;
use std::fs;

fn get_utc(timezone: &str, local_datetime: &str) -> DateTime<Utc> {
    let fmt = "%b %d, %Y, %H:%M";
    let tz: Tz = timezone.parse().unwrap();
    NaiveDateTime::parse_from_str(local_datetime, fmt)
        .unwrap()
        .and_local_timezone(tz)
        .single()
        .unwrap()
        .to_utc()
}

fn parse(input: &str) -> Vec<(DateTime<Utc>, DateTime<Utc>)> {
    input
        .split("\n\n")
        .map(|journey| {
            let dts: Vec<DateTime<Utc>> = journey
                .lines()
                .map(|line| {
                    let words: Vec<&str> = line.split_whitespace().collect();
                    let timezone = words[1];
                    let datetime = words[2..].join(" ");
                    get_utc(timezone, &datetime)
                })
                .collect();
            (dts[0], dts[1])
        })
        .collect()
}

fn solution(input: &str) -> i64 {
    parse(input)
        .iter()
        .map(|(dep, arr)| arr.signed_duration_since(dep).num_minutes())
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
        assert_eq!(solution(&input), 3143);
    }
}
