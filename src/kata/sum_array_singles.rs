// 7 Kyu: Sum of array singles
// https://www.codewars.com/kata/59f11118a5e129e591000134/train/rust

use std::collections::HashMap;

#[allow(dead_code)]
pub fn repeats(arr: &Vec<i32>) -> i32 {
    let mut value_counts: HashMap<i32, i32> = HashMap::new();

    for num in arr.iter() {
        *value_counts.entry(*num).or_insert(0) += 1;
    }

    let sum = value_counts.iter()
        .filter(|(_, value)| *value == &1)
        .map(|(key, _)| *key)
        .sum();

    sum    
}


#[cfg(test)]
mod tests {
    use super::repeats;

    #[test]
    fn basic_tests() {
        assert_eq!(repeats(&vec![4, 5, 7, 5, 4, 8]), 15);
        assert_eq!(repeats(&vec![9, 10, 19, 13, 19, 13]), 19);
        assert_eq!(repeats(&vec![16, 0, 11, 4, 8, 16, 0, 11]), 12);
        assert_eq!(repeats(&vec![5, 17, 18, 11, 13, 18, 11, 13]), 22);
        assert_eq!(repeats(&vec![5, 10, 19, 13, 10, 13]), 24);
    }
}