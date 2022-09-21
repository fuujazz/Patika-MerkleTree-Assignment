use super::*;
use sha2::{Digest, Sha256};

pub fn create_next_level(current_level: Vec<String>) -> Vec<String> {
    let mut next_level: Vec<String> = Vec::new();
    let mut element = String::new();

    // taking two elements from the current level and hashing them
    for (index, item) in current_level.iter().enumerate() {
        if index % 2 == 0 {
            element = item.clone();
        } else {
            element = element + item;
            next_level.push(element.clone());
        }
    }

    next_level
}
