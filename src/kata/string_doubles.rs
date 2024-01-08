// 7 Kyu: String doubles
// https://www.codewars.com/kata/5a145ab08ba9148dd6000094/train/rust

#[allow(dead_code)]
pub fn doubles(s: &str) -> String {
    let mut result = String::new();
    let mut chars = s.chars().peekable();

    while let Some(current_char) = chars.next() {
        if let Some(next_char) = chars.peek() {
            if current_char == *next_char {
                chars.next(); // Skip the next character
            } else {
                result.push(current_char);
            }
        } else {
            result.push(current_char);
        }
    }

    result
}


#[cfg(test)]
mod tests {
    use super::doubles;

    #[test]
    fn sample_tests() {
        assert_eq!(doubles(&"abbbzz"), "ab");
        assert_eq!(doubles(&"zzzzykkkd"), "ykd");
        assert_eq!(doubles(&"abbcccdddda"), "aca");
        assert_eq!(doubles(&"vvvvvoiiiiin"), "voin");
        assert_eq!(doubles(&"rrrmooomqqqqj"), "rmomj");
        assert_eq!(doubles(&"xxbnnnnnyaaaaam"), "bnyam");
    }
}
