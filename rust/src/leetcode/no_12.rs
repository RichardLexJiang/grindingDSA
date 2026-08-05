impl Solution {
    const ROMAN_NUMERALS: [&str; 13] = [
        "M", "CM", "D", "CD", "C", "XC", "L", "XL", "X", "IX", "V", "IV", "I",
    ];
    const DECIMAL: [i32; 13] = [1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1];

    pub fn int_to_roman(num: i32) -> String {
        let mut result = String::new();
        let mut num_copy = num;
        while num_copy > 0 {
            for (i, v) in Self::DECIMAL.iter().enumerate() {
                if num_copy >= *v {
                    num_copy = num_copy - v;
                    result = result + Self::ROMAN_NUMERALS[i];
                    break;
                }
            }
        }
        result
    }
    
}

#[test]
fn test() {
    assert_eq!(Solution::int_to_roman(3749), "MMMDCCXLIX");
    assert_eq!(Solution::int_to_roman(58), "LVIII");
    assert_eq!(Solution::int_to_roman(1994), "MCMXCIV");
}

/**
Roman numerals are formed by appending the conversions of decimal place values from highest to lowest. Converting a decimal place value into a Roman numeral has the following rules:

If the value does not start with 4 or 9, select the symbol of the maximal value that can be subtracted from the input, append that symbol to the result, subtract its value, and convert the remainder to a Roman numeral.
If the value starts with 4 or 9 use the subtractive form representing one symbol subtracted from the following symbol, for example, 4 is 1 (I) less than 5 (V): IV and 9 is 1 (I) less than 10 (X): IX. Only the following subtractive forms are used: 4 (IV), 9 (IX), 40 (XL), 90 (XC), 400 (CD) and 900 (CM).
Only powers of 10 (I, X, C, M) can be appended consecutively at most 3 times to represent multiples of 10. You cannot append 5 (V), 50 (L), or 500 (D) multiple times. If you need to append a symbol 4 times use the subtractive form.
Given an integer, convert it to a Roman numeral.
*/
struct Solution {}
