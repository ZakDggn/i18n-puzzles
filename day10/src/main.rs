use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::{collections::HashMap, fs};
use unicode_normalization::UnicodeNormalization;

fn parse(input: &str) -> (HashMap<&str, &str>, Vec<(&str, &str)>) {
    let (entries, attempts) = input.split_once("\n\n").unwrap();
    (
        entries
            .lines()
            .map(|line| line.split_once(' ').unwrap())
            .collect(),
        attempts
            .lines()
            .map(|line| line.split_once(' ').unwrap())
            .collect(),
    )
}

fn decompositions(composed: &str) -> Vec<String> {
    let mut permutations = vec![String::new()];
    for c in composed.chars() {
        let d = c.nfd().to_string();
        if d.chars().count() == 1 {
            for perm in &mut permutations {
                perm.push(c);
            }
        } else {
            permutations = permutations
                .into_iter()
                .flat_map(|perm| [perm.clone() + &c.to_string(), perm + &d])
                .collect();
        }
    }
    permutations
}

fn solution(input: &str) -> u32 {
    let (hash_db, attempts) = parse(input);
    let mut valid = 0;
    let mut password_cache = HashMap::new();
    for (user, password) in attempts {
        let hash = hash_db.get(user).unwrap();
        let composed = password.nfc().to_string();
        if !password_cache.contains_key(user)
            && decompositions(&composed)
                .par_iter()
                .any(|perm| bcrypt::verify(perm, hash).unwrap())
        {
            password_cache.insert(user, composed.clone());
        }
        if let Some(correct) = password_cache.get(user) {
            if &composed == correct {
                valid += 1;
            }
        }
    }
    valid
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
        assert_eq!(solution(&input), 4);
    }
}
