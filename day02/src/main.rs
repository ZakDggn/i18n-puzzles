use std::collections::HashMap;
use std::fs;

fn solution(input: &str) -> String {
    let mut counts = HashMap::new();
    for line in input.lines() {
        let dt = chrono::DateTime::parse_from_rfc3339(line).unwrap().to_utc();
        *counts.entry(dt).or_insert(0) += 1;
    }
    let (dt, _n) = counts.iter().max_by_key(|(_dt, n)| **n).unwrap();
    dt.to_rfc3339()
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
    fn test() {
        let input = fs::read_to_string("test-input").unwrap();
        assert_eq!(solution(&input), "2019-06-05T12:15:00+00:00");
    }
}
