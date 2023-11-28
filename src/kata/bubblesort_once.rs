// 7 Kyu: Bubblesort Once
// https://www.codewars.com/kata/56b97b776ffcea598a0006f2/train/rust

#[allow(dead_code)]
pub fn bubblesort_once(lst: &[u32]) -> Vec<u32> {
    let n = lst.len();
    let mut lst = lst.to_vec();

    for i in 0..n - 1 {
        if lst[i] > lst[i + 1] {
            lst.swap(i, i + 1);
        }
    }

    lst
}


mod tests {
    use super::bubblesort_once;
        
    fn dotest(a: &[u32], expected: &[u32]) {
        let actual = bubblesort_once(a);
        assert!(actual == expected, 
            "With a = {a:?}\nExpected {expected:?} but got {actual:?}")
    }

    #[test]
    fn example_test() {
        dotest(&[9, 7, 5, 3, 1, 2, 4, 6, 8], &[7, 5, 3, 1, 2, 4, 6, 8, 9]);
    }
}
