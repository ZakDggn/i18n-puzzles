use std::fs;

const UPPERCASE: &[char] = &[
    'Α', 'Β', 'Γ', 'Δ', 'Ε', 'Ζ', 'Η', 'Θ', 'Ι', 'Κ', 'Λ', 'Μ', 'Ν', 'Ξ', 'Ο', 'Π', 'Ρ', 'Σ', 'Τ',
    'Υ', 'Φ', 'Χ', 'Ψ', 'Ω',
];
const LOWERCASE: &[char] = &[
    'α', 'β', 'γ', 'δ', 'ε', 'ζ', 'η', 'θ', 'ι', 'κ', 'λ', 'μ', 'ν', 'ξ', 'ο', 'π', 'ρ', 'σ', 'τ',
    'υ', 'φ', 'χ', 'ψ', 'ω',
];
const N_CHARS: usize = 24;
const ODYSSEUS_VARIANTS: &[&str] = &["Οδυσσευς", "Οδυσσεως", "Οδυσσει", "Οδυσσεα", "Οδυσσευ"];

fn rotate(sentence: &str, shift: usize) -> String {
    let mut rotated = String::new();
    let mut chars = sentence
        .chars()
        .map(|c| if c == 'ς' { 'σ' } else { c })
        .peekable();
    while let Some(c) = chars.next() {
        if c.is_uppercase() {
            let n = UPPERCASE.iter().position(|&a| a == c).unwrap();
            rotated.push(UPPERCASE[(n + shift) % N_CHARS]);
        } else if c.is_lowercase() {
            let n = LOWERCASE.iter().position(|&a| a == c).unwrap();
            let c_shifted = match LOWERCASE[(n + shift) % N_CHARS] {
                'σ' => match chars.peek() {
                    Some(next) if !next.is_alphabetic() => 'ς',
                    _ => 'σ',
                },
                c => c,
            };
            rotated.push(c_shifted);
        } else {
            rotated.push(c);
        }
    }
    rotated
}

fn contains_odysseus(sentence: &str) -> bool {
    ODYSSEUS_VARIANTS.iter().any(|var| sentence.contains(var))
}

fn solution(input: &str) -> usize {
    input
        .lines()
        .filter_map(|sentence| {
            (1..N_CHARS).find(|&shift| contains_odysseus(&rotate(sentence, shift)))
        })
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
    fn test_rotate() {
        assert_eq!(
            rotate("σζμ γ' ωοωλδθαξλδμξρ οπξρδυζ οξκτλζσθρ Ξγτρρδτρ.", 1),
            "την δ' απαμειβομενος προσεφη πολυμητις Οδυσσευς."
        );
        assert_eq!(
            rotate("αφτ κ' λαλψφτ ωπφχλρφτ δξησηρζαλψφτ φελο, Φκβωωλβ.", 18),
            "τον δ' ετερον σκοπελον χθαμαλωτερον οψει, Οδυσσευ."
        );
    }

    #[test]
    fn example() {
        let input = fs::read_to_string("test-input").unwrap();
        assert_eq!(solution(&input), 19);
    }
}
