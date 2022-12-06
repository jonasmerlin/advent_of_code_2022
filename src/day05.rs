use std::{collections::HashMap, fs};

#[derive(Debug)]
struct Move {
    num_crates: u32,
    from: u32,
    to: u32,
}

fn read_initial_stacks(lines: &[&str]) -> Vec<Vec<char>> {
    let stack_lines = &lines[0..8];
    let stack_num_line = lines[8];

    let mut index_in_line_to_stack_num = HashMap::<usize, u32>::new();
    for (index, character) in stack_num_line.chars().enumerate() {
        if character >= '0' && character <= '9' {
            if let Some(i) = character.to_digit(10) {
                index_in_line_to_stack_num.insert(index, i);
            }
        }
    }

    let mut stacks: Vec<Vec<char>> = vec![vec![]; index_in_line_to_stack_num.len()];
    for line in stack_lines.into_iter().rev() {
        for (index, character) in line.chars().enumerate() {
            // Since we know that crates are only designated with capital letters ...
            if character >= 'A' && character <= 'Z' {
                let stack_num_at_index = index_in_line_to_stack_num.get(&index).unwrap() - 1;
                let stack_option = stacks.get_mut(stack_num_at_index as usize);

                match stack_option {
                    Some(stack) => stack.push(character),
                    None => stacks[stack_num_at_index as usize] = vec![character],
                }
            }
        }
    }

    stacks
}

fn read_moves(lines: &[&str]) -> Vec<Move> {
    let move_lines = &lines[10..];

    let mut move_nums: Vec<u32> = Vec::new();
    for move_line in move_lines {
        let line_split: Vec<&str> = (**move_line).split_ascii_whitespace().collect();

        move_nums.push(line_split[1].parse::<u32>().unwrap());
        move_nums.push(line_split[3].parse::<u32>().unwrap());
        move_nums.push(line_split[5].parse::<u32>().unwrap());
    }

    let mut moves: Vec<Move> = Vec::new();
    for move_chunk in move_nums.chunks(3) {
        moves.push(Move {
            num_crates: move_chunk[0],
            from: move_chunk[1],
            to: move_chunk[2],
        });
    }

    moves
}

fn part_1(input: &str) {
    let lines: Vec<&str> = input.lines().collect();

    let mut stacks = read_initial_stacks(&lines);
    let moves = read_moves(&lines);

    for crate_move in moves {
        for _ in 0..crate_move.num_crates {
            let value = stacks[(crate_move.from - 1) as usize].pop().unwrap();

            stacks[(crate_move.to - 1) as usize].push(value);
        }
    }

    for stack in stacks {
        print!("{}", stack.last().unwrap());
    }

    println!();
}

fn part_2(input: &str) {
    let lines: Vec<&str> = input.lines().collect();

    let mut stacks = read_initial_stacks(&lines);
    let moves = read_moves(&lines);

    for crate_move in moves {
        let stack_from = &mut stacks[(crate_move.from - 1) as usize];
        let last_n_elements =
            &mut stack_from.split_off(stack_from.len() - crate_move.num_crates as usize);

        stacks[(crate_move.to - 1) as usize].append(last_n_elements);
    }

    for stack in stacks {
        print!("{}", stack.last().unwrap());
    }

    println!();
}

fn main() {
    let input = fs::read_to_string("./input/input_day05.txt").expect("Couldn't find file.");

    println!("{}", input.lines().collect::<Vec<&str>>().len());

    part_1(&input);
    part_2(&input);
}

// #[test]
// fn do_ranges_overlap_works() {
//     assert_eq!(true, do_ranges_overlap(&(1..2), &(1..3)));
//     assert_eq!(false, do_ranges_overlap(&(1..2), &(2..3)));
//     assert_eq!(true, do_ranges_overlap(&(1..6), &(2..3)));
// }
