use std::fs;

type FromUtf16Result = Result<String, std::string::FromUtf16Error>;
const MAX_LATIN_CODE: u16 = 0x1eff; // End of "Latin Extended Additional" block

enum Endianness {
    BE,
    LE,
}

fn decode_utf16(bytes: &[u8], endianness: &Endianness) -> FromUtf16Result {
    let u16s: Vec<u16> = bytes
        .chunks_exact(2)
        .map(|chunk| {
            let (hi, lo) = match endianness {
                Endianness::BE => (0, 1),
                Endianness::LE => (1, 0),
            };
            u16::from(chunk[hi]) * 256 + u16::from(chunk[lo])
        })
        .collect();
    String::from_utf16(&u16s)
}

fn decode(bytes: &[u8]) -> String {
    if bytes[..2] == [0xfe, 0xff] {
        return decode_utf16(&bytes[2..], &Endianness::BE).unwrap();
    } else if bytes[..2] == [0xff, 0xfe] {
        return decode_utf16(&bytes[2..], &Endianness::LE).unwrap();
    } else if bytes[..3] == [0xef, 0xbb, 0xbf] {
        return std::str::from_utf8(&bytes[3..]).unwrap().to_owned();
    }
    if let Ok(utf8) = String::from_utf8(bytes.to_owned()) {
        return utf8;
    }
    for endianness in [Endianness::BE, Endianness::LE] {
        if let Ok(utf16) = decode_utf16(bytes, &endianness) {
            if utf16
                .chars()
                .all(|c| c.is_alphabetic() && (c as u16) <= MAX_LATIN_CODE)
            {
                return utf16;
            }
        }
    }
    let latin1: String = bytes.iter().map(|&b| char::from(b)).collect();
    assert!(latin1.chars().all(char::is_alphabetic));
    latin1
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
    let (dictionary, crossword) = input.split_once("\n\n").unwrap();
    let words: Vec<String> = dictionary
        .lines()
        .map(|line| {
            let bytes: Vec<u8> = (0..line.len() - 1)
                .step_by(2)
                .map(|i| u8::from_str_radix(&line[i..i + 2], 16).unwrap())
                .collect();
            decode(&bytes)
        })
        .collect();
    let crossword: Vec<&str> = crossword.lines().map(str::trim).collect();
    let mut sum = 0;
    for blank in &crossword {
        sum += find_match(blank, &words).unwrap();
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
        assert_eq!(solution(&input), 47);
    }
}
