use std::{cmp::max, fs};

struct Fraction(u64, u64);

struct Length {
    value: u64,
    unit: Fraction,
}

struct Area {
    value: u64,
    unit: Fraction,
}

impl std::ops::Mul for Fraction {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0, self.1 * rhs.1)
    }
}

impl std::ops::Mul for Length {
    type Output = Area;

    fn mul(self, rhs: Self) -> Self::Output {
        Area {
            value: self.value * rhs.value,
            unit: self.unit * rhs.unit,
        }
    }
}

impl Area {
    fn to_metric(&self) -> u64 {
        self.value * self.unit.0 * (10 * 10) / self.unit.1 / (33 * 33)
    }
}

fn parse_numeral(c: char) -> u64 {
    match c {
        '一' => 1,
        '二' => 2,
        '三' => 3,
        '四' => 4,
        '五' => 5,
        '六' => 6,
        '七' => 7,
        '八' => 8,
        '九' => 9,
        '十' => 10,
        '百' => 100,
        '千' => 1000,
        '万' => 10_000,
        '億' => 100_000_000,
        _ => panic!("Invalid numeral: {c}"),
    }
}

fn parse_number(number: &str) -> u64 {
    let mut prev = 0;
    let mut acc = 0;
    let mut total = 0;
    for c in number.chars() {
        let value = parse_numeral(c);
        if value >= 10_000 {
            acc += prev;
            total += acc * value;
            acc = 0;
            prev = 0;
        } else if value >= 10 {
            acc += max(prev, 1) * value;
            prev = 0;
        } else {
            prev = value;
        }
    }
    total + acc + prev
}

fn parse_length(length: &str) -> Length {
    let mut chars: Vec<char> = length.chars().collect();
    let unit = chars.pop().unwrap();
    let number: String = chars.into_iter().collect();
    let value = parse_number(&number);
    let unit = match unit {
        '尺' => Fraction(1, 1),
        '間' => Fraction(6, 1),
        '丈' => Fraction(10, 1),
        '町' => Fraction(360, 1),
        '里' => Fraction(12_960, 1),
        '毛' => Fraction(1, 10_000),
        '厘' => Fraction(1, 1000),
        '分' => Fraction(1, 100),
        '寸' => Fraction(1, 10),
        c => panic!("Invalid unit: {c}"),
    };
    Length { value, unit }
}

fn area(dimensions: &str) -> u64 {
    let (width, height) = dimensions.split_once(" × ").unwrap();
    (parse_length(width) * parse_length(height)).to_metric()
}

fn solution(input: &str) -> u64 {
    input.lines().map(area).sum()
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
    fn test_parse_number() {
        assert_eq!(parse_number("十一"), 11);
        assert_eq!(parse_number("十二"), 12);
        assert_eq!(parse_number("二十"), 20);
        assert_eq!(parse_number("四十二"), 42);
        assert_eq!(parse_number("十万"), 100_000);
        assert_eq!(parse_number("百万"), 1_000_000);
        assert_eq!(parse_number("千万"), 10_000_000);
        assert_eq!(parse_number("三百"), 300);
        assert_eq!(parse_number("三百二十一"), 321);
        assert_eq!(parse_number("四千"), 4000);
        assert_eq!(parse_number("五万"), 50_000);
        assert_eq!(parse_number("九万九千九百九十九"), 99_999);
        assert_eq!(parse_number("四十二万四十二"), 420_042);
        assert_eq!(
            parse_number("九億八千七百六十五万四千三百二十一"),
            987_654_321
        );
    }

    #[test]
    fn test_area() {
        assert_eq!(area("二百四十二町 × 三百五十一丈"), 28_080_000);
        assert_eq!(area("七十八寸 × 二十一万七千八百厘"), 156);
        assert_eq!(area("七万二千三百五十八町 × 六百十二分"), 14_639_040);
        assert_eq!(area("六寸 × 三十万七千九十八尺"), 16920);
        assert_eq!(area("九間 × 三万三千百五十四里"), 2_130_624_000);
        assert_eq!(area("六百毛 × 七百四十四万千五百厘"), 41);
        assert_eq!(
            area("七十八億二千八十三万五千毛 × 二十八万八千六百毛"),
            2_072_629
        );
        assert_eq!(
            area("三百七十四万二千五百三十厘 × 六百七十一万七千厘"),
            2_308_409
        );
    }

    #[test]
    fn example() {
        let input = fs::read_to_string("test-input").unwrap();
        assert_eq!(solution(&input), 2_177_741_195);
    }
}
