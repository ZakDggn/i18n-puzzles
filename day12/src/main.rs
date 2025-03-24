use std::{collections::HashMap, fs};
use unicode_normalization::UnicodeNormalization;

fn sort_english(mut lines: Vec<String>) -> Vec<String> {
    lines.sort_by_key(|line| {
        line.chars()
            .filter(|c| c.is_alphabetic() || *c == ',')
            .map(|c| deunicode::deunicode_char(c).unwrap().to_lowercase())
            .collect::<String>()
    });
    lines
}

fn sort_swedish(mut lines: Vec<String>) -> Vec<String> {
    let char_to_int: HashMap<char, u8> = ",abcdefghijklmnopqrstuvwxyzåäö"
        .chars()
        .enumerate()
        .map(|(i, c)| (c, i as u8))
        .collect();
    lines.sort_by_key(|line| {
        line.to_lowercase()
            .chars()
            .map(|c| match c {
                'æ' => 'ä',
                'ø' => 'ö',
                c if char_to_int.contains_key(&c) => c,
                c => c.nfd().next().unwrap(),
            })
            .filter_map(|c| char_to_int.get(&c).copied())
            .collect::<Vec<u8>>()
    });
    lines
}

fn move_dutch_infix(line: &str) -> String {
    let surname_start = line.chars().position(char::is_uppercase).unwrap();
    if surname_start == 0 {
        line.to_owned()
    } else {
        let name_end = line.find(':').unwrap();
        format!(
            "{} {}{}",
            &line[surname_start..name_end],
            &line[0..surname_start - 1],
            &line[name_end..]
        )
    }
}

fn sort_dutch(mut lines: Vec<String>) -> Vec<String> {
    for line in &mut lines {
        *line = move_dutch_infix(line);
    }
    sort_english(lines)
}

fn middle_phone_number(lines: &[String]) -> u64 {
    let line = &lines[lines.len() / 2];
    let phone_number = line.split_whitespace().last().unwrap();
    phone_number.parse().unwrap()
}

fn solution(input: &str) -> u64 {
    let lines: Vec<String> = input.lines().map(ToOwned::to_owned).collect();
    let english = sort_english(lines.clone());
    let swedish = sort_swedish(lines.clone());
    let dutch = sort_dutch(lines);
    middle_phone_number(&english) * middle_phone_number(&swedish) * middle_phone_number(&dutch)
}

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let answer = solution(&input);
    println!("{answer}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test-input");

    #[test]
    fn test_sort_english() {
        assert_eq!(
            sort_english(TEST_INPUT.lines().map(ToOwned::to_owned).collect()),
            &[
                "Aalto, Alvar: 0192872",
                "Åberg, Rosa-Maria: 0110966",
                "Æbelø, Aurora: 0113267",
                "Ämtler, Lorena: 0112717",
                "Navarrete Ortiz, Dolores: 0119411",
                "Ñíguez Peña, María de los Ángeles: 0151605",
                "Olofsson, Mikael: 0103652",
                "O'Neill, Cara: 0109551",
                "Østergård, Magnus: 0113959",
                "Özaydın, Zeynep: 0185292",
                "van den Heyden, Harm: 0168131",
                "Vandersteen, Willy: 0120659",
                "van Leeuw, Floor: 0144158",
                "van Leeuwen, Joke: 0172199",
                "Zondervan, Jan Peter: 0103008",
            ]
        )
    }

    #[test]
    fn test_sort_swedish() {
        assert_eq!(
            sort_swedish(TEST_INPUT.lines().map(ToOwned::to_owned).collect()),
            &[
                "Aalto, Alvar: 0192872",
                "Navarrete Ortiz, Dolores: 0119411",
                "Ñíguez Peña, María de los Ángeles: 0151605",
                "Olofsson, Mikael: 0103652",
                "O'Neill, Cara: 0109551",
                "van den Heyden, Harm: 0168131",
                "Vandersteen, Willy: 0120659",
                "van Leeuw, Floor: 0144158",
                "van Leeuwen, Joke: 0172199",
                "Zondervan, Jan Peter: 0103008",
                "Åberg, Rosa-Maria: 0110966",
                "Æbelø, Aurora: 0113267",
                "Ämtler, Lorena: 0112717",
                "Østergård, Magnus: 0113959",
                "Özaydın, Zeynep: 0185292",
            ]
        )
    }

    #[test]
    fn test_move_dutch_infix() {
        assert_eq!(
            move_dutch_infix("Aalto, Alvar: 0192872"),
            "Aalto, Alvar: 0192872"
        );
        assert_eq!(
            move_dutch_infix("van Leeuw, Floor: 0144158"),
            "Leeuw, Floor van: 0144158"
        );
        assert_eq!(
            move_dutch_infix("van den Heyden, Harm: 0168131"),
            "Heyden, Harm van den: 0168131"
        );
    }

    #[test]
    fn test_sort_dutch() {
        assert_eq!(
            sort_dutch(TEST_INPUT.lines().map(ToOwned::to_owned).collect()),
            &[
                "Aalto, Alvar: 0192872",
                "Åberg, Rosa-Maria: 0110966",
                "Æbelø, Aurora: 0113267",
                "Ämtler, Lorena: 0112717",
                "Heyden, Harm van den: 0168131",
                "Leeuw, Floor van: 0144158",
                "Leeuwen, Joke van: 0172199",
                "Navarrete Ortiz, Dolores: 0119411",
                "Ñíguez Peña, María de los Ángeles: 0151605",
                "Olofsson, Mikael: 0103652",
                "O'Neill, Cara: 0109551",
                "Østergård, Magnus: 0113959",
                "Özaydın, Zeynep: 0185292",
                "Vandersteen, Willy: 0120659",
                "Zondervan, Jan Peter: 0103008",
            ]
        )
    }

    #[test]
    fn example() {
        let input = fs::read_to_string("test-input").unwrap();
        assert_eq!(solution(&input), 1885816494308838);
    }
}
