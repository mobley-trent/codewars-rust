// 7 Kyu: Credit Card Mask
// https://www.codewars.com/kata/5412509bd436bd33920011bc/train/rust

#[allow(dead_code)]
pub 
fn maskify(cc: &str) -> String {
    let mut result: String = String::new();
    let n = 4;

    for (i, c) in cc.chars().rev().enumerate() {
        if i >= n {
            result.push('#')
        } else {
            result.push(c);
        }
    }

    result.chars().rev().collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unittests() {
        assert_eq!(maskify("4556364607935616"), "############5616");
        assert_eq!(maskify("64607935616"), "#######5616");
        assert_eq!(maskify("1"), "1");
        assert_eq!(maskify(""), "");
    }
}