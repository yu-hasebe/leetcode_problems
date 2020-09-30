pub struct Solution;

// submitted code starts here
impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let mut ret: Vec<String> = vec![];
        for digit in digits.chars() {
            let mut current = vec![];

            let letters = Self::numter_to_letters(digit);
            for &letter in letters.iter() {
                if ret.len() == 0 {
                    current.push(letter.to_string());
                } else {
                    for r in ret.iter() {
                        let mut r = r.clone();
                        r.push(letter);
                        current.push(r);
                    }
                }
            }
            ret = current;
        }
        ret.sort();
        ret
    }

    fn numter_to_letters(digit: char) -> Vec<char> {
        match digit {
            '2' => vec!['a', 'b', 'c'],
            '3' => vec!['d', 'e', 'f'],
            '4' => vec!['g', 'h', 'i'],
            '5' => vec!['j', 'k', 'l'],
            '6' => vec!['m', 'n', 'o'],
            '7' => vec!['p', 'q', 'r', 's'],
            '8' => vec!['t', 'u', 'v'],
            '9' => vec!['w', 'x', 'y', 'z'],
            _ => vec![],
        }
    }
}
// submitted code ends here

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_letter_combinations() {
        let digits = "23".to_string();
        let expect = vec![
            "ad".to_string(),
            "ae".to_string(),
            "af".to_string(),
            "bd".to_string(),
            "be".to_string(),
            "bf".to_string(),
            "cd".to_string(),
            "ce".to_string(),
            "cf".to_string(),
        ];
        let get = Solution::letter_combinations(digits);
        assert_eq!(expect, get);
    }
}
