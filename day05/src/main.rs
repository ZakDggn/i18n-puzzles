use std::fs;

fn solution(input: &str) -> u32 {
    let cols = input.lines().next().unwrap().chars().count();
    let mut count = 0;
    let mut col = 0;
    for row in input.lines() {
        if row.chars().nth(col).unwrap() == '💩' {
            count += 1;
        }
        col = (col + 2) % cols;
    }
    count
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
        assert_eq!(solution(&input), 2);
    }
}
