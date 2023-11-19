// 7 Kyu: Odd or Even ?
// https://www.codewars.com/kata/5949481f86420f59480000e7/train/rust

#[allow(dead_code)]
pub fn odd_or_even(numbers: Vec<i32>) -> String {
    let sum: i32 = numbers.iter().sum();
    if sum % 2 == 0 {
        "even".to_string()
    } else {
        "odd".to_string()
    }
}


#[cfg(test)]
mod tests {
    use std::vec;
    use super::*;

    #[test]
    fn unittests() {
        assert_eq!(odd_or_even(vec![]), "even");
        assert_eq!(odd_or_even(vec![0]), "even");
        assert_eq!(odd_or_even(vec![1]), "odd");
        assert_eq!(odd_or_even(vec![0, 1, 4]), "odd");
        assert_eq!(odd_or_even(vec![0, -1, -5]), "even");
        assert_eq!(odd_or_even(vec![0, -1, 2]), "odd");

    }
}