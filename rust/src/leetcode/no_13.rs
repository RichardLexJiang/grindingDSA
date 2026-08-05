impl Solution {
    const ROMAN_NUMERALS: [&str; 13] = [
        "M", "CM", "D", "CD", "C", "XC", "L", "XL", "X", "IX", "V", "IV", "I",
    ];
    const DECIMAL: [i32; 13] = [1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1];

    pub fn roman_to_int(s: String) -> i32 {
        let mut result = 0;
        let mut s_copy = s;
        while !s_copy.is_empty() {
            for (i, x) in Self::ROMAN_NUMERALS.iter().enumerate() {
                if s_copy.starts_with(x) {
                    s_copy = s_copy.split_at(x.len()).1.to_string();
                    result += Self::DECIMAL[i];
                    break;
                }
            }
        }

        result
    }
}

#[test]
fn sample_test() {
    assert_eq!(Solution::roman_to_int(String::from("M")), 1000);
    assert_eq!(Solution::roman_to_int(String::from("CM")), 900);
    assert_eq!(Solution::roman_to_int(String::from("D")), 500);
    assert_eq!(Solution::roman_to_int(String::from("III")), 3);
    assert_eq!(Solution::roman_to_int(String::from("LVIII")), 58);
    assert_eq!(Solution::roman_to_int(String::from("MCMXCIV")), 1994);
}

struct Solution {}
