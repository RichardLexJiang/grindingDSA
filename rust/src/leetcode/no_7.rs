impl Solution {
    pub fn reverse(x: i32) -> i32 {
        Self::std_answer(x)
    }

    fn std_answer(x: i32) -> i32 {
        let mut ans = 0;
        let is_minus = x < 0;
        let mut x = x;
        const BORDER_PRE: i32 = 214748364;
        while x != 0 {
            if is_minus && (ans > BORDER_PRE || (ans == BORDER_PRE && -x % 10 > 8)) {
                return 0;
            } else if !is_minus && (ans > BORDER_PRE || (ans == BORDER_PRE && x % 10 > 7)) {
                return 0;
            };
            ans = ans * 10 + (x % 10).abs();
            x /= 10;
        }
        if is_minus { -ans } else { ans }
    }

    /**
        conv to string , and do compare with i32::MAX 's str , kind of stupid.
    */
    #[allow(dead_code)]
    fn conv_str_and_reverse(x: i32) -> i32 {
        use std::str::FromStr;

        let is_minus = x < 0;
        let x_str = if !is_minus {
            x.to_string().chars().rev().collect::<String>()
        } else {
            x.to_string()
                .strip_prefix("-")
                .unwrap()
                .to_string()
                .chars()
                .rev()
                .collect::<String>()
        };
        let result = if Self::is_over_i32(is_minus, &x_str) {
            0
        } else {
            i32::from_str(&x_str).unwrap()
        };
        if is_minus { -result } else { result }
    }

    fn is_over_i32(is_minus: bool, x: &String) -> bool {
        use std::cmp::Ordering;

        const POSITIVE_MAX: &str = "2147483647";
        const NEGATIVE_MAX: &str = "2147483648";

        if x.len() > POSITIVE_MAX.len() {
            true
        } else if x.len() == POSITIVE_MAX.len() {
            if is_minus && NEGATIVE_MAX.cmp(x) == Ordering::Less {
                true
            } else if !is_minus && POSITIVE_MAX.cmp(x) == Ordering::Less {
                true
            } else {
                false
            }
        } else {
            false
        }
    }
}

#[test]
fn test() {
    // 2147483647
    assert_eq!(Solution::reverse(i32::MAX), 0);
    assert_eq!(Solution::reverse(i32::MIN), 0);
    assert_eq!(Solution::reverse(123), 321);
    assert_eq!(Solution::reverse(-123), -321);
    assert_eq!(Solution::reverse(120), 21);
    assert_eq!(Solution::reverse(-2147483412), -2143847412);
}

struct Solution;
