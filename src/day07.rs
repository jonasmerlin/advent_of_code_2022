use std::{collections::HashMap, fs};

#[derive(Debug)]
struct File {
    name: String,
    size: u32,
}

#[derive(Debug)]
struct FileSystemNode {
    name: String,
    parent: Option<usize>,
    sub_folders: HashMap<String, usize>,
    files: Vec<File>,
}

fn sum_sizes(nodes: &Vec<FileSystemNode>, starting_index: usize, running_sum: &mut u32) -> u32 {
    let current_node = &nodes[starting_index];
    // println!("{:?}", current_node);

    let mut size = 0;
    for file in current_node.files.iter() {
        size += file.size
    }

    for (name, index) in current_node.sub_folders.iter() {
        size += sum_sizes(nodes, *index, running_sum);
    }

    if size <= 100000 {
        *running_sum += size;
    }

    size
}

fn collect_sizes(
    nodes: &Vec<FileSystemNode>,
    starting_index: usize,
    sizes: &mut HashMap<String, u32>,
) -> u32 {
    let current_node = &nodes[starting_index];
    // println!("{:?}", current_node);

    let mut size = 0;
    for file in current_node.files.iter() {
        size += file.size
    }

    for (name, index) in current_node.sub_folders.iter() {
        size += collect_sizes(nodes, *index, sizes);
    }

    sizes.insert(String::from(&current_node.name), size);

    size
}

fn part_1(input: &str) {
    let mut line_iter = input.lines().peekable();

    let mut file_system: Vec<FileSystemNode> = Vec::new();
    let mut current_node_index: Option<usize> = None;

    while let Some(line) = line_iter.next() {
        let line_split: Vec<&str> = line.split(" ").collect();

        if line_split[0] == "$" {
            let command = line_split[1];

            match command {
                "cd" => {
                    let folder = line_split[2];

                    if folder == ".." {
                        // go up
                        match current_node_index {
                            Some(index) => {
                                current_node_index = file_system[index].parent;
                            }
                            None => (),
                        }
                    } else {
                        // go down
                        let new_node = FileSystemNode {
                            name: String::from(folder),
                            parent: current_node_index,
                            sub_folders: HashMap::new(),
                            files: Vec::new(),
                        };

                        file_system.push(new_node);

                        let new_node_index = file_system.len() - 1;

                        match current_node_index {
                            Some(index) => {
                                file_system[index]
                                    .sub_folders
                                    .insert(String::from(folder), new_node_index);
                            }
                            None => (),
                        }

                        current_node_index = Some(new_node_index);
                    }
                }
                "ls" => {
                    while let Some(next_line) = line_iter.peek() {
                        if next_line.starts_with("$") {
                            break;
                        } else {
                            let next_line_split: Vec<&str> = next_line.split(" ").collect();

                            if next_line_split[0] == "dir" {
                                // dir
                            } else {
                                // file
                                let size = next_line_split[0].parse::<u32>().unwrap();
                                let name = next_line_split[1];

                                match current_node_index {
                                    Some(index) => file_system[index].files.push(File {
                                        name: String::from(name),
                                        size,
                                    }),
                                    None => (),
                                }
                            }

                            line_iter.next();
                        }
                    }
                }
                _ => (),
            }
        }
    }

    let mut sum = 0;

    sum_sizes(&file_system, 0, &mut sum);

    println!("{}", sum);

    let mut sizes = HashMap::new();

    collect_sizes(&file_system, 0, &mut sizes);

    let unused = 70000000 - sizes.get("/").unwrap();
    let needed = 30000000 - unused;

    println!("unused: {}", unused);
    println!("needed: {}", needed);

    let mut min_distance: u32 = u32::MAX;
    for (name, size) in sizes {
        if name != "/" {
            if size >= needed && size < min_distance {
                min_distance = size;
            }
        }
    }

    println!("{}", min_distance);
}

// fn part_2(input: &str) {
//     let mut last_chars: Vec<char> = Vec::new();
//     for (index, character) in input.chars().enumerate() {
//         if !last_chars.contains(&character) {
//             last_chars.push(character);
//             if last_chars.len() == LEN_START_OF_MESSAGE_MARKER {
//                 // ... but need it here?
//                 println!("{}", index + 1);
//                 break;
//             }
//         } else {
//             last_chars.clear();
//         }
//     }
// }

fn main() {
    let input = fs::read_to_string("./input/input_day07.txt").expect("Couldn't find file.");

    part_1(&input);
    // part_2(&input);
}

// #[test]
// fn do_ranges_overlap_works() {
//     assert_eq!(true, do_ranges_overlap(&(1..2), &(1..3)));
//     assert_eq!(false, do_ranges_overlap(&(1..2), &(2..3)));
//     assert_eq!(true, do_ranges_overlap(&(1..6), &(2..3)));
// }
