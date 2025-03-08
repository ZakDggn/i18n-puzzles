use std::fs;

fn is_valid_sms(msg: &str) -> bool {
    msg.len() <= 160
}

fn is_valid_tweet(msg: &str) -> bool {
    msg.chars().count() <= 140
}

fn cost(msg: &str) -> u32 {
    let valid_sms = is_valid_sms(msg);
    let valid_tweet = is_valid_tweet(msg);
    if valid_sms && valid_tweet {
        13
    } else if valid_sms {
        11
    } else if valid_tweet {
        7
    } else {
        0
    }
}

fn solution(input: &str) -> u32 {
    input.lines().map(cost).sum()
}

fn main() {
    let input = fs::read_to_string("input").unwrap();
    println!("{}", solution(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = fs::read_to_string("test-input").unwrap();
        assert_eq!(solution(&input), 31);
    }
}
