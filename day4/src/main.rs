use std::iter;
use std::str::FromStr;

fn main() {
    const input: &str = "246540-787419";

    let n = parse_range(input)
        .map(|n| get_digits(n))
        .filter(|n| part_1(n))
        .count();
    println!("Part 1 = {:?}", n);

    let n = parse_range(input)
        .map(|n| get_digits(n))
        .filter(|n| part_2(n))
        .count();
    println!("Part 2 = {:?}", n);
}

fn part_1(digits: &Vec<u32>) -> bool {
    let mut has_double = false;

    for i in 1..digits.len() {
        if digits[i] < digits[i - 1] {
            return false;
        }
        has_double = has_double || digits[i] == digits[i - 1];
    }

    has_double
}

fn part_2(digits: &Vec<u32>) -> bool {
    let mut double_without_triple = false;

    let mut last_number: u32 = digits[0];
    let mut last_number_count = 1;

    for i in 1..digits.len() {
        if digits[i] < digits[i - 1] {
            return false;
        }

        // 2nd number like the 1st and not like 3rd
        if digits[i] == digits[i - 1] {
            last_number_count += 1;
        }

        if digits[i] != digits[i - 1] || i == digits.len() - 1 {
            if last_number_count == 2 {
                double_without_triple = true;
            }
            last_number = digits[i];
            last_number_count = 1;
        }
    }

    double_without_triple
}

fn parse_range(range: &str) -> std::ops::RangeInclusive<u32> {
    let mut it = range.split("-").map(|s| u32::from_str(s).unwrap());
    it.next().unwrap()..=it.next().unwrap()
}

fn get_digits(number: u32) -> Vec<u32> {
    vec![
        (number / 100000) % 10,
        (number / 10000) % 10,
        (number / 1000) % 10,
        (number / 100) % 10,
        (number / 10) % 10,
        number % 10,
    ]
}
