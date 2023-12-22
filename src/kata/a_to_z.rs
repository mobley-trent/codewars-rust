// 7 Kyu: From A to Z
// https://www.codewars.com/kata/6512b3775bf8500baea77663/train/rust

#[allow(dead_code)]
pub fn gimme_the_letters(sp: &str) -> String {
    let mut res = String::new();

    let first = sp.chars().next().unwrap();
    let last = sp.chars().rev().next().unwrap();

    for letter in first..=last {
        res.push(letter);
    }

    res
}


#[cfg(test)]
mod tests {
    use super::gimme_the_letters;

    fn dotest(sp: &str, expected: &str) {
        let actual = gimme_the_letters(sp);
        assert!(actual == expected)
    }

    #[test]
    fn fixed_tests() {
        dotest("a-z", "abcdefghijklmnopqrstuvwxyz");
        dotest("h-o", "hijklmno");
        dotest("Q-Z", "QRSTUVWXYZ");
        dotest("J-J", "J");
        dotest("A-A", "A");
        dotest("a-b", "ab");
        dotest("g-i", "ghi");
        dotest("H-I", "HI");
        dotest("y-z", "yz");
        dotest("e-k", "efghijk");
        dotest("a-q", "abcdefghijklmnopq");
        dotest("F-O", "FGHIJKLMNO");
    }
}