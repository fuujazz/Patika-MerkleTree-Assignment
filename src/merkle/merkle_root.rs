use super::create_next_level::create_next_level;
use super::hash_input::hash_input;
use std::fs;

pub fn merkle_root(filename: String) -> String {
    let result_vector = fs::read_to_string(filename)
        .expect("Unable to read file")
        .lines()
        .map(|line| line.to_string())
        .collect::<Vec<String>>();

    let lenght_of_leaves: u32 = result_vector[0].parse().unwrap();

    let mut data = result_vector[1..].to_vec();

    let mut hex_vector: Vec<String> = Vec::new();

    for _ in 0..lenght_of_leaves {
        hex_vector = data.iter().map(|element| hash_input(element)).collect();
        data = create_next_level(hex_vector);
    }

    hash_input(&data[0])
}
