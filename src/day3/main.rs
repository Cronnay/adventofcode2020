use std::env;

struct Coordinates {
    pub x: u16,
    pub y: u16,
}

impl Coordinates {
    pub fn new(x: u16, y: u16) -> Coordinates {
        Coordinates { x, y }
    }
}

const TREE: char = '#';
fn main() {
    let mut file: String = String::new();
    for args in env::args() {
        file = args;
    }

    let contents = std::fs::read_to_string(file).expect("Could not load file");
    let mut matrix: Vec<Vec<char>> = Vec::new();
    for line in contents.lines() {
        let line_as_char: Vec<char> = line.chars().collect();
        matrix.push(line_as_char);
    }

    step_1(&matrix);
    step_2(&matrix);
}

fn step_1(matrix: &Vec<Vec<char>>) {
    const STEPS_RIGHT: u16 = 3;
    const STEPS_DOWN: u16 = 1;

    let traverse_speed = Coordinates::new(STEPS_RIGHT, STEPS_DOWN);
    println!(
        "Amount of trees: {}",
        traverse_through_map(&matrix, traverse_speed)
    );
}

fn step_2(matrix: &Vec<Vec<char>>) {
    let mut product_of_trees: u64 = 1;

    let traverse_speed_vec = vec![
        Coordinates::new(1, 1),
        Coordinates::new(3, 1),
        Coordinates::new(5, 1),
        Coordinates::new(7, 1),
        Coordinates::new(1, 2),
    ];

    for traverse_speed in traverse_speed_vec {
        product_of_trees *= traverse_through_map(matrix, traverse_speed);
    }

    println!("Product of all trees: {}", product_of_trees);
}

fn traverse_through_map(matrix: &Vec<Vec<char>>, traverse_speed: Coordinates) -> u64 {
    let mut trees_found = 0;
    let mut position = Coordinates::new(0, 0);
    let width = matrix.get(0).unwrap().len();
    while position.y < matrix.len() as u16 {
        if let Some(row) = matrix.get(position.y as usize) {
            if let Some(character) = row.get(position.x as usize) {
                if *character == TREE {
                    trees_found += 1;
                }
            }
        }
        position.x = (position.x + traverse_speed.x) % width as u16;
        position.y = position.y + traverse_speed.y;
    }
    trees_found
}
