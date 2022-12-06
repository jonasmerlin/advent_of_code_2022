use std::fs;

static LEN_START_OF_PACKET_MARKER: usize = 4;
static LEN_START_OF_MESSAGE_MARKER: usize = 14;

fn part_1(input: &str) {
    let mut last_chars: Vec<char> = Vec::new();
    for (index, character) in input.chars().enumerate() {
        if !last_chars.contains(&character) {
            last_chars.push(character);
            if last_chars.len() == LEN_START_OF_PACKET_MARKER {
                // I don't get why I don't need to add 1 here ...
                println!("{}", index);
                break;
            }
        } else {
            last_chars.clear();
        }
    }
}

fn part_2(input: &str) {
    let mut last_chars: Vec<char> = Vec::new();
    for (index, character) in input.chars().enumerate() {
        if !last_chars.contains(&character) {
            last_chars.push(character);
            if last_chars.len() == LEN_START_OF_MESSAGE_MARKER {
                // ... but need it here?
                println!("{}", index + 1);
                break;
            }
        } else {
            last_chars.clear();
        }
    }
}

fn main() {
    let input = fs::read_to_string("./input/input_day06.txt").expect("Couldn't find file.");

    part_1(&input);
    part_2(&input);
}

// #[test]
// fn do_ranges_overlap_works() {
//     assert_eq!(true, do_ranges_overlap(&(1..2), &(1..3)));
//     assert_eq!(false, do_ranges_overlap(&(1..2), &(2..3)));
//     assert_eq!(true, do_ranges_overlap(&(1..6), &(2..3)));
// }
