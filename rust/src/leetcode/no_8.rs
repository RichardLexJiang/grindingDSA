impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        Self::my_naive_way(s)
    }

    fn my_naive_way(s: String) -> i32 {
        let mut chars = s.trim().chars().collect::<Vec<char>>();
        if chars.len() == 0 {
            return 0;
        }
        let mut negative = false;
        if chars[0] == '-' {
            negative = true;
            chars.remove(0);
        } else if chars[0] == '+' {
            chars.remove(0);
        }
        for (i, c) in chars.iter().enumerate() {
            if !c.is_numeric() {
                for _ in 0..(chars.len() - i) {
                    chars.pop();
                }
                break;
            };
        }
        let mut answer = 0i64;
        for x in chars {
            answer = answer * 10;
            if negative {
                answer = answer - x.to_digit(10).unwrap() as i64;
                if answer < i32::MIN as i64 {
                    return i32::MIN;
                }
            } else {
                answer = answer + x.to_digit(10).unwrap() as i64;
                if answer > i32::MAX as i64 {
                    return i32::MAX;
                }
            }
        }
        answer as i32
    }
}

#[test]
fn test() {
    assert_eq!(Solution::my_atoi("  -123f2a4124".to_owned()), -123);
    assert_eq!(Solution::my_atoi("123".to_owned()), 123);
    assert_eq!(Solution::my_atoi("  123".to_owned()), 123);
    assert_eq!(Solution::my_atoi("0-1".to_owned()), 0);
    assert_eq!(Solution::my_atoi("words and 987".to_owned()), 0);
    assert_eq!(Solution::my_atoi("-91283472332".to_owned()), -2147483648);
}

struct Solution;
