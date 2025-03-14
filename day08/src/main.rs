use std::collections::HashSet;
use std::fs;
use unicode_normalization::UnicodeNormalization;

const VOWELS: &str = "aeiou";
const CONSONANTS: &str = "bcdfghjklmnpqrstvwxyz";

fn normalise(pwd: &str) -> String {
    pwd.nfkd()
        .filter(char::is_ascii)
        .map(|c| c.to_ascii_lowercase())
        .collect()
}

fn all_unique(pwd: &str) -> bool {
    let mut uniq = HashSet::new();
    pwd.chars().all(|c| uniq.insert(c))
}

fn is_valid(pwd: &str) -> bool {
    let normalised = normalise(pwd);
    (4..=12).contains(&normalised.len())
        && normalised.chars().any(|c| c.is_ascii_digit())
        && normalised.chars().any(|c| VOWELS.contains(c))
        && normalised.chars().any(|c| CONSONANTS.contains(c))
        && all_unique(&normalised)
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
    fn test_normalise() {
        assert_eq!(normalise("iS0"), "is0");
        assert_eq!(normalise("V8AeC1S7KhP4Ļu"), "v8aec1s7khp4lu");
        assert_eq!(normalise("pD9Ĉ*jXh"), "pd9c*jxh");
        assert_eq!(normalise("E1-0"), "e1-0");
        assert_eq!(normalise("ĕnz2cymE"), "enz2cyme");
        assert_eq!(normalise("tqd~üō"), "tqd~uo");
        assert_eq!(normalise("IgwQúPtd9"), "igwquptd9");
        assert_eq!(normalise("k2lp79ąqV"), "k2lp79aqv");
    }

    #[test]
    fn example() {
        let input = fs::read_to_string("test-input").unwrap();
        assert_eq!(solution(&input), 2);
    }
}
