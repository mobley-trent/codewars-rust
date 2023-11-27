// 7 Kyu: Multiples By Permutations II
// https://www.codewars.com/kata/5ba178be875de960a6000187/train/rust

#[allow(dead_code)]
pub fn find_lowest_int(k: u64) -> u64 {
    let k2 = k + 1;
    let mut found = false;
    let mut n: u64 = 1;
    
    while !found {
        let s1 = (k * n).to_string();
        let s2 = (k2 * n).to_string();

        let mut chars1: Vec<char> = s1.chars().collect();
        let mut chars2: Vec<char> = s2.chars().collect();

        chars1.sort();
        chars2.sort();

        if chars1 == chars2 {
            found = true;
        } else {
            n = n + 1;
        }
    }

    return n;
}


#[cfg(test)]
mod tests {
    use super::find_lowest_int;
        
    fn dotest(n: u64, expected: u64) {
        let actual = find_lowest_int(n);
        assert!(actual == expected, 
            "With k = {n}\nExpected {expected} but got {actual}")
    }

    #[test]
    fn fixed_tests() {
        dotest(325,477);
        dotest(599,2394);
        dotest(855, 999);
        dotest(1,125874);
        dotest(100,8919);
        dotest(1000,89919);
        dotest(10000,899919);
    }
}