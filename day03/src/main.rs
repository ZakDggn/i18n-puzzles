use std::fs;

fn is_valid(pwd: &str) -> bool {
    (4..=12).contains(&pwd.chars().count())
        && pwd.chars().any(|c| c.is_ascii_digit())
        && pwd.chars().any(char::is_uppercase)
        && pwd.chars().any(char::is_lowercase)
        && !pwd.is_ascii()
}

fn solution(input: &str) -> usize {
    input.lines().filter(|pwd| is_valid(pwd)).count()
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
