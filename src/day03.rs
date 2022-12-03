use std::fs;

static LOWERCASE_LOWER_BOUND: i32 = 'a' as i32;
static UPPERCASE_LOWER_BOUND: i32 = 'A' as i32;

static LOWERCASE_VALUE_TO_SUBSTRACT: i32 = LOWERCASE_LOWER_BOUND - 1;
static UPPERCASE_VALUE_TO_SUBSTRACT: i32 = UPPERCASE_LOWER_BOUND - 27;

fn get_score_for_char(character: char) -> i32 {
    if character.is_lowercase() {
        return character as i32 - LOWERCASE_VALUE_TO_SUBSTRACT;
    } else {
        return character as i32 - UPPERCASE_VALUE_TO_SUBSTRACT;
    }
}

fn find_common_char(str_1: &str, str_2: &str) -> Option<char> {
    for character in str_1.chars() {
        if str_2.contains(character) {
            return Some(character);
        }
    }

    None
}

// Can be replaced by <slice>.split_at()
fn get_splits(string: &str) -> (&str, &str) {
    let middle_index = string.len() / 2;

    (&string[..middle_index], &string[middle_index..])
}

fn find_common_char_between_three(str_1: &str, str_2: &str, str_3: &str) -> Option<char> {
    for character in str_1.chars() {
        if str_2.contains(character) {
            if str_3.contains(character) {
                return Some(character);
            }
        }
    }

    None
}

fn part_1(input: &str) {
    let mut sum = 0;
    for line in input.lines() {
        let (split_1, split_2) = line.split_at(line.len() / 2);
        let common_char = find_common_char(split_1, split_2);
        match common_char {
            Some(character) => sum += get_score_for_char(character),
            None => panic!("No common character found!"),
        }
    }

    println!("{}", sum);
}

fn part_2(input: &str) {
    let mut sum = 0;
    for lines in input.lines().collect::<Vec<&str>>().chunks(3) {
        let common_char = find_common_char_between_three(lines[0], lines[1], lines[2]);
        match common_char {
            Some(character) => sum += get_score_for_char(character),
            None => panic!("No common character found!"),
        }
    }

    println!("{}", sum);
}

fn main() {
    let input = fs::read_to_string("./input_day03.txt").expect("Couldn't find file.");

    println!("{}", input.lines().collect::<Vec<&str>>().len());

    part_1(&input);
    part_2(&input);
}

#[test]
fn get_score_for_char_works_for_boundaries() {
    assert_eq!(1, get_score_for_char('a'));
    assert_eq!(26, get_score_for_char('z'));
    assert_eq!(27, get_score_for_char('A'));
    assert_eq!(52, get_score_for_char('Z'));
}
