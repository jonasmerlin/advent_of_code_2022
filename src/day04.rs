use std::fs;

/*
All this would've been much better done without converting the sections into
ranges I feel.
*/

fn get_sections_as_range(sections: &str) -> std::ops::Range<i32> {
    let range_ends: Vec<i32> = sections
        .split("-")
        .map(|end| str::parse::<i32>(end).unwrap())
        .collect();

    // My answers were too low for a long time because I forgot the `+ 1` here.
    range_ends[0]..range_ends[1] + 1
}

fn is_range_contained_in_other(
    possibly_containing_range: &std::ops::Range<i32>,
    possibly_contained_range: &std::ops::Range<i32>,
) -> bool {
    // All the cloning here makes me nervous. I don't like!
    possibly_containing_range.clone().min() <= possibly_contained_range.clone().min()
        && possibly_containing_range.clone().max() >= possibly_contained_range.clone().max()
}

fn do_ranges_overlap(range_1: &std::ops::Range<i32>, range_2: &std::ops::Range<i32>) -> bool {
    let range_1_min = range_1.clone().min().unwrap();
    let range_1_max = range_1.clone().max().unwrap();

    let range_2_min = range_2.clone().min().unwrap();
    let range_2_max = range_2.clone().max().unwrap();

    let range_1_vec: Vec<i32> = range_1.clone().collect();
    let range_2_vec: Vec<i32> = range_2.clone().collect();

    (range_1_vec.contains(&range_2_min) || range_1_vec.contains(&range_2_max))
        || (range_2_vec.contains(&range_1_min) || range_2_vec.contains(&range_1_max))
}

fn part_1(input: &str) {
    let mut sum = 0;
    for line in input.lines() {
        if line != "" {
            println!("{}", line);

            let parts: Vec<&str> = line.split(",").collect();

            let sections_elf_1 = parts[0];
            let sections_elf_2 = parts[1];

            let sections_elf_1_range = get_sections_as_range(sections_elf_1);
            let sections_elf_2_range = get_sections_as_range(sections_elf_2);

            println!("{:?}", sections_elf_1_range);
            println!("{:?}", sections_elf_2_range);

            if is_range_contained_in_other(&sections_elf_1_range, &sections_elf_2_range) {
                sum += 1;
            } else if is_range_contained_in_other(&sections_elf_2_range, &sections_elf_1_range) {
                sum += 1;
            }
        }
    }

    println!("{}", sum);
}

fn part_2(input: &str) {
    let mut sum = 0;
    for line in input.lines() {
        if line != "" {
            println!("{}", line);

            let parts: Vec<&str> = line.split(",").collect();

            let sections_elf_1 = parts[0];
            let sections_elf_2 = parts[1];

            let sections_elf_1_range = get_sections_as_range(sections_elf_1);
            let sections_elf_2_range = get_sections_as_range(sections_elf_2);

            println!("{:?}", sections_elf_1_range);
            println!("{:?}", sections_elf_2_range);

            if do_ranges_overlap(&sections_elf_1_range, &sections_elf_2_range) {
                sum += 1;
            }
        }
    }

    println!("{}", sum);
}

fn main() {
    let input = fs::read_to_string("./input/input_day04.txt").expect("Couldn't find file.");

    println!("{}", input.lines().collect::<Vec<&str>>().len());

    part_1(&input);
    part_2(&input);
}

#[test]
fn do_ranges_overlap_works() {
    assert_eq!(true, do_ranges_overlap(&(1..2), &(1..3)));
    assert_eq!(false, do_ranges_overlap(&(1..2), &(2..3)));
    assert_eq!(true, do_ranges_overlap(&(1..6), &(2..3)));
}
