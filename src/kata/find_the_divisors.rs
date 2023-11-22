// 7 Kyu: Find the divisors!
// https://www.codewars.com/kata/544aed4c4a30184e960010f4/train/rust

#[allow(dead_code)]
pub fn divisors(integer: u32) -> Result<Vec<u32>, String> {
    let mut result: Vec<u32> = Vec::new();

    for i in 2..integer {
        if integer % i == 0 {
            result.push(i);
        }
    }

    if result.is_empty() {
        let res = format!("{} is a prime number", integer);
        Err(res.to_string())
    } else {
        Ok(result)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unittests() {
        assert_eq!(divisors(15), Ok(vec![3, 5]));
        assert_eq!(divisors(12), Ok(vec![2, 3, 4, 6]));
        assert_eq!(divisors(13), Err("13 is a prime number".to_string()));
        assert_eq!(divisors(25), Ok(vec![5]));
    }
}