// 8 Kyu: Gravity Flip
// https://www.codewars.com/kata/5f70c883e10f9e0001c89673/train/rust

#[allow(dead_code)]
pub fn flip(dir: char, cubes: &[u32]) -> Vec<u32> {
    let mut arr = cubes.to_vec();

    match dir {
        'R' => arr.sort(),
        'L' => arr.sort_by(|a, b| b.cmp(a)),
        _ => panic!()
    }

    return arr;
}


#[cfg(test)]
mod tests {
    use std::vec;
    use super::*;

    #[test]
    fn unittests() {
        assert_eq!(flip('R', &vec![3, 2, 1, 2]), vec![1, 2, 2, 3]);
        assert_eq!(flip('L', &vec![1, 4, 5, 3, 5]), vec![5, 5, 4, 3, 1]);
    }
}
