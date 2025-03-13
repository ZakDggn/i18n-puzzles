use std::fs;

fn unmash(word: &str) -> String {
    let bytes = word.chars().map(|c| u8::try_from(c).unwrap()).collect();
    String::from_utf8(bytes).unwrap()
}

fn fix_words(words: &str) -> Vec<String> {
    let mut fixed_words = Vec::new();
    for (i, word) in words.lines().enumerate() {
        if (i + 1) % 15 == 0 {
            fixed_words.push(unmash(&unmash(word)));
        } else if (i + 1) % 3 == 0 || (i + 1) % 5 == 0 {
            fixed_words.push(unmash(word));
        } else {
            fixed_words.push(word.to_owned());
        }
    }
    fixed_words
}

fn find_match(blank: &str, words: &[String]) -> Option<usize> {
    let length = blank.chars().count();
    let index = blank.chars().position(|c| c != '.').unwrap();
    let letter = blank.chars().nth(index);
    for (i, word) in words.iter().enumerate() {
        if word.chars().count() == length && word.chars().nth(index) == letter {
            return Some(i + 1);
        }
    }
    None
}

fn solution(input: &str) -> usize {
    let (words, crossword) = input.split_once("\n\n").unwrap();
    let words = fix_words(words);
    let mut sum = 0;
    for blank in crossword.lines().map(str::trim) {
        let line_number = find_match(blank, &words).unwrap();
        sum += line_number;
    }
    sum
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
        assert_eq!(solution(&input), 50);
    }
}
