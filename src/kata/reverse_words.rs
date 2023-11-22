// 7 Kyu: Reverse Words
// https://www.codewars.com/kata/5259b20d6021e9e14c0010d4/train/rust

#[allow(dead_code)]
pub fn reverse_words(str: &str) -> String {
    let mut result = String::new();
    let mut curr_word = String::new();

    for c in str.chars() {
        if c.is_whitespace() {
            if !curr_word.is_empty() {
                let reversed_word: String = curr_word.chars()
                    .rev()
                    .collect();
                result.push_str(&reversed_word);
                result.push(c);
                curr_word.clear();
            } else {
                result.push(c);
            }
        } else {
            curr_word.push(c);
        }
    }

    let reversed_word: String = curr_word.chars()
        .rev()
        .collect();

    result.push_str(&reversed_word);
    result
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unittests() {
        assert_eq!(reverse_words("The quick brown fox jumps over the lazy dog."), "ehT kciuq nworb xof spmuj revo eht yzal .god");
        assert_eq!(reverse_words("apple"), "elppa");
        assert_eq!(reverse_words("a b c d"),"a b c d");
        assert_eq!(reverse_words(" double  spaced  words "), " elbuod  decaps  sdrow ");
    }
}