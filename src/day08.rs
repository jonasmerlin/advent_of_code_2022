use std::fs;

fn build_grid_rows(input: &str) -> Vec<Vec<u32>> {
    let mut grid_rows: Vec<Vec<u32>> = Vec::new();

    for line in input.lines() {
        let row = line
            .chars()
            .map(|character| character.to_digit(10).unwrap())
            .collect::<Vec<u32>>();

        grid_rows.push(row);
    }

    grid_rows
}

fn build_grid_cols(input: &str) -> Vec<Vec<u32>> {
    let mut grid_cols: Vec<Vec<u32>> = Vec::new();

    for line in input.lines() {
        for (index, character) in line.chars().enumerate() {
            let col = grid_cols.get_mut(index);

            if col.is_some() {
                col.unwrap().push(character.to_digit(10).unwrap());
            } else {
                grid_cols.push(vec![character.to_digit(10).unwrap()]);
            };
        }
    }

    grid_cols
}

fn is_visible_from_the_left(tree_x: usize, row: &Vec<u32>) -> bool {
    for x_left in 0..tree_x {
        if row[x_left] >= row[tree_x] {
            return false;
        }
    }

    true
}

fn is_visible_from_the_top(tree_y: usize, col: &Vec<u32>) -> bool {
    for y_top in 0..tree_y {
        if col[y_top] >= col[tree_y] {
            return false;
        }
    }

    true
}

fn is_visible_from_the_right(tree_x: usize, row: &Vec<u32>) -> bool {
    for x_right in tree_x + 1..row.len() {
        if row[x_right] >= row[tree_x] {
            return false;
        }
    }

    true
}

fn is_visible_from_the_bottom(tree_y: usize, col: &Vec<u32>) -> bool {
    for y_top in tree_y + 1..col.len() {
        if col[y_top] >= col[tree_y] {
            return false;
        }
    }

    true
}

fn is_visible(
    tree_x: usize,
    tree_y: usize,
    grid_rows: &Vec<Vec<u32>>,
    grid_cols: &Vec<Vec<u32>>,
) -> bool {
    if is_visible_from_the_left(tree_x, &grid_rows[tree_y])
        || is_visible_from_the_right(tree_x, &grid_rows[tree_y])
        || is_visible_from_the_top(tree_y, &grid_cols[tree_x])
        || is_visible_from_the_bottom(tree_y, &grid_cols[tree_x])
    {
        return true;
    }

    false
}

fn count_visible_trees(grid_rows: &Vec<Vec<u32>>, grid_cols: &Vec<Vec<u32>>) -> u32 {
    let count_visible =
        count_visible_outer(grid_rows, grid_cols) + count_visible_inner(grid_rows, grid_cols);

    count_visible
}

fn count_visible_inner(grid_rows: &Vec<Vec<u32>>, grid_cols: &Vec<Vec<u32>>) -> u32 {
    let mut count_visible: u32 = 0;

    for y in 1..grid_rows.len() - 1 {
        for x in 1..grid_cols.len() - 1 {
            if is_visible(x, y, &grid_rows, &grid_cols) {
                count_visible += 1;
            }
        }
    }

    count_visible
}

fn count_visible_outer(grid_rows: &Vec<Vec<u32>>, grid_cols: &Vec<Vec<u32>>) -> u32 {
    let count_visible_outer: u32 = 2 * grid_cols.len() as u32 + 2 * (grid_rows.len() as u32 - 2);

    count_visible_outer
}

fn part_1(input: &str) {
    println!();
    println!("### Part 1 ###");

    let grid_rows = build_grid_rows(input);
    let grid_cols = build_grid_cols(input);

    let count_visible = count_visible_trees(&grid_rows, &grid_cols);

    println!("{}", count_visible);
}

struct Coordinate {
    x: usize,
    y: usize,
}

fn find_max_scenic_score(grid_rows: &Vec<Vec<u32>>, grid_cols: &Vec<Vec<u32>>) -> u32 {
    let mut max_scenic_score: u32 = 0;

    for y in 1..grid_rows.len() - 1 {
        for x in 1..grid_cols.len() - 1 {
            let tree_coordinate = Coordinate { x, y };

            let scenic_sore = calc_scenic_score(tree_coordinate, grid_rows, grid_cols);

            if scenic_sore > max_scenic_score {
                max_scenic_score = scenic_sore;
            }
        }
    }

    max_scenic_score
}

fn calc_scenic_score(
    tree_coordinates: Coordinate,
    grid_rows: &Vec<Vec<u32>>,
    grid_cols: &Vec<Vec<u32>>,
) -> u32 {
    let scenic_score = visible_trees_left(tree_coordinates.x, &grid_rows[tree_coordinates.y])
        * visible_trees_top(tree_coordinates.y, &grid_cols[tree_coordinates.x])
        * visible_trees_right(tree_coordinates.x, &grid_rows[tree_coordinates.y])
        * visible_trees_bottom(tree_coordinates.y, &grid_cols[tree_coordinates.x]);

    scenic_score
}

fn visible_trees_left(tree_x: usize, row: &Vec<u32>) -> u32 {
    let mut last_visible_tree_index = tree_x;

    for x in (0..tree_x).rev() {
        if x == 0 || row[x] >= row[tree_x] {
            last_visible_tree_index = x;
            break;
        }
    }

    (tree_x - last_visible_tree_index) as u32
}

fn visible_trees_top(tree_y: usize, col: &Vec<u32>) -> u32 {
    let mut last_visible_tree_index = tree_y;

    for y in (0..tree_y).rev() {
        if y == 0 || col[y] >= col[tree_y] {
            last_visible_tree_index = y;
            break;
        }
    }

    (tree_y - last_visible_tree_index) as u32
}

fn visible_trees_right(tree_x: usize, row: &Vec<u32>) -> u32 {
    let mut last_visible_tree_index = tree_x;

    for x in tree_x + 1..row.len() {
        if x == row.len() - 1 || row[x] >= row[tree_x] {
            last_visible_tree_index = x;
            break;
        }
    }

    (last_visible_tree_index - tree_x) as u32
}

fn visible_trees_bottom(tree_y: usize, col: &Vec<u32>) -> u32 {
    let mut last_visible_tree_index = tree_y;

    for x in tree_y + 1..col.len() {
        if x == col.len() - 1 || col[x] >= col[tree_y] {
            last_visible_tree_index = x;
            break;
        }
    }

    (last_visible_tree_index - tree_y) as u32
}

fn part_2(input: &str) {
    println!();
    println!("### Part 2 ###");

    let grid_rows = build_grid_rows(input);
    let grid_cols = build_grid_cols(input);

    let count_visible = find_max_scenic_score(&grid_rows, &grid_cols);

    println!("{}", count_visible);
}

fn main() {
    let input = fs::read_to_string("./input/input_day08.txt").expect("Couldn't find file.");

    part_1(&input);
    part_2(&input);
}

/*
  Part 1 Tests
*/

#[test]
fn is_visible_from_the_left_works() {
    let row: Vec<u32> = vec![2, 3, 9, 8, 4, 9, 2];

    assert_eq!(true, is_visible_from_the_left(2, &row));
    assert_eq!(false, is_visible_from_the_left(3, &row));
    assert_eq!(false, is_visible_from_the_left(5, &row));
}

#[test]
fn is_visible_from_the_bottom_works() {
    let row: Vec<u32> = vec![2, 3, 9, 8, 4, 9, 2];

    assert_eq!(true, is_visible_from_the_bottom(row.len() - 2, &row));
    assert_eq!(false, is_visible_from_the_bottom(row.len() - 3, &row));
}

#[test]
fn is_visible_works() {
    let grid_rows: Vec<Vec<u32>> = vec![
        vec![3, 0, 3, 7, 3],
        vec![2, 5, 5, 1, 2],
        vec![6, 5, 3, 3, 2],
        vec![3, 3, 5, 4, 9],
        vec![3, 5, 3, 9, 0],
    ];

    let grid_cols: Vec<Vec<u32>> = vec![
        vec![3, 2, 6, 3, 3],
        vec![0, 5, 5, 3, 5],
        vec![3, 5, 3, 5, 3],
        vec![7, 1, 3, 4, 9],
        vec![3, 2, 2, 9, 0],
    ];

    assert_eq!(true, is_visible(1, 1, &grid_rows, &grid_cols));
    assert_eq!(true, is_visible(1, 2, &grid_rows, &grid_cols));
    assert_eq!(false, is_visible(1, 3, &grid_rows, &grid_cols));
    assert_eq!(true, is_visible(2, 1, &grid_rows, &grid_cols));
    assert_eq!(false, is_visible(2, 2, &grid_rows, &grid_cols));
    assert_eq!(true, is_visible(2, 3, &grid_rows, &grid_cols));
    assert_eq!(false, is_visible(3, 1, &grid_rows, &grid_cols));
    assert_eq!(true, is_visible(3, 2, &grid_rows, &grid_cols));
    assert_eq!(false, is_visible(3, 3, &grid_rows, &grid_cols));
}

#[test]
fn count_visible_inner_works() {
    let grid_rows: Vec<Vec<u32>> = vec![
        vec![3, 0, 3, 7, 3],
        vec![2, 5, 5, 1, 2],
        vec![6, 5, 3, 3, 2],
        vec![3, 3, 5, 4, 9],
        vec![3, 5, 3, 9, 0],
    ];

    let grid_cols: Vec<Vec<u32>> = vec![
        vec![3, 2, 6, 3, 3],
        vec![0, 5, 5, 3, 5],
        vec![3, 5, 3, 5, 3],
        vec![7, 1, 3, 4, 9],
        vec![3, 2, 2, 9, 0],
    ];

    let count_visible = count_visible_inner(&grid_rows, &grid_cols);

    assert_eq!(count_visible, 5);
}

#[test]
fn count_visible_outer_works() {
    let grid_rows: Vec<Vec<u32>> = vec![
        vec![3, 0, 3, 7, 3],
        vec![2, 5, 5, 1, 2],
        vec![6, 5, 3, 3, 2],
        vec![3, 3, 5, 4, 9],
        vec![3, 5, 3, 9, 0],
    ];

    let grid_cols: Vec<Vec<u32>> = vec![
        vec![3, 2, 6, 3, 3],
        vec![0, 5, 5, 3, 5],
        vec![3, 5, 3, 5, 3],
        vec![7, 1, 3, 4, 9],
        vec![3, 2, 2, 9, 0],
    ];

    let count_visible = count_visible_outer(&grid_rows, &grid_cols);

    assert_eq!(count_visible, 16);
}

#[test]
fn count_visible_trees_works() {
    let grid_rows: Vec<Vec<u32>> = vec![
        vec![3, 0, 3, 7, 3],
        vec![2, 5, 5, 1, 2],
        vec![6, 5, 3, 3, 2],
        vec![3, 3, 5, 4, 9],
        vec![3, 5, 3, 9, 0],
    ];

    let grid_cols: Vec<Vec<u32>> = vec![
        vec![3, 2, 6, 3, 3],
        vec![0, 5, 5, 3, 5],
        vec![3, 5, 3, 5, 3],
        vec![7, 1, 3, 4, 9],
        vec![3, 2, 2, 9, 0],
    ];

    let count_visible = count_visible_trees(&grid_rows, &grid_cols);

    assert_eq!(count_visible, 21);
}

/*
  Part 2 Tests
*/

#[test]
fn visible_trees_left_works() {
    let grid_rows: Vec<Vec<u32>> = vec![
        vec![3, 0, 3, 7, 3],
        vec![2, 5, 5, 1, 2],
        vec![6, 5, 3, 3, 2],
        vec![3, 3, 5, 4, 9],
        vec![3, 5, 3, 9, 0],
    ];

    assert_eq!(visible_trees_left(0, &grid_rows[1]), 0);
    assert_eq!(visible_trees_left(1, &grid_rows[1]), 1);
    assert_eq!(visible_trees_left(2, &grid_rows[1]), 1);
    assert_eq!(visible_trees_left(3, &grid_rows[1]), 1);
    assert_eq!(visible_trees_left(4, &grid_rows[1]), 2);

    assert_eq!(visible_trees_left(4, &grid_rows[3]), 4);
}

#[test]
fn visible_trees_right_works() {
    let grid_rows: Vec<Vec<u32>> = vec![
        vec![3, 0, 3, 7, 3],
        vec![2, 5, 5, 1, 2],
        vec![6, 5, 3, 3, 2],
        vec![3, 3, 5, 4, 9],
        vec![3, 5, 3, 9, 0],
    ];

    assert_eq!(visible_trees_right(4, &grid_rows[3]), 0);
    assert_eq!(visible_trees_right(3, &grid_rows[3]), 1);
}

#[test]
fn all_visible_trees_functions_work() {
    let grid_rows: Vec<Vec<u32>> = vec![
        vec![3, 0, 3, 7, 3],
        vec![2, 5, 5, 1, 2],
        vec![6, 5, 3, 3, 2],
        vec![3, 3, 5, 4, 9],
        vec![3, 5, 3, 9, 0],
    ];

    let grid_cols: Vec<Vec<u32>> = vec![
        vec![3, 2, 6, 3, 3],
        vec![0, 5, 5, 3, 5],
        vec![3, 5, 3, 5, 3],
        vec![7, 1, 3, 4, 9],
        vec![3, 2, 2, 9, 0],
    ];

    assert_eq!(visible_trees_left(4, &grid_rows[3]), 4);
    assert_eq!(visible_trees_top(3, &grid_cols[4]), 3);
    assert_eq!(visible_trees_right(4, &grid_rows[3]), 0);
    assert_eq!(visible_trees_bottom(3, &grid_cols[4]), 1);
}

#[test]
fn calc_scenic_score_works() {
    let grid_rows: Vec<Vec<u32>> = vec![
        vec![3, 0, 3, 7, 3],
        vec![2, 5, 5, 1, 2],
        vec![6, 5, 3, 3, 2],
        vec![3, 3, 5, 4, 9],
        vec![3, 5, 3, 9, 0],
    ];

    let grid_cols: Vec<Vec<u32>> = vec![
        vec![3, 2, 6, 3, 3],
        vec![0, 5, 5, 3, 5],
        vec![3, 5, 3, 5, 3],
        vec![7, 1, 3, 4, 9],
        vec![3, 2, 2, 9, 0],
    ];

    assert_eq!(
        calc_scenic_score(Coordinate { x: 2, y: 1 }, &grid_rows, &grid_cols),
        4
    );
}
