use std::sync::LazyLock;

static MAPPING: LazyLock<[Vec<char>; 8]> = LazyLock::new(|| {
    [
        vec!['a', 'b', 'c'],
        vec!['d', 'e', 'f'],
        vec!['g', 'h', 'i'],
        vec!['j', 'k', 'l'],
        vec!['m', 'n', 'o'],
        vec!['p', 'q', 'r', 's'],
        vec!['t', 'u', 'v'],
        vec!['w', 'x', 'y', 'z'],
    ]
});

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let mut rev_input: Vec<char> = digits.chars().rev().collect();
        let mut result = vec![];
        Self::dfs(&mut rev_input, &mut Vec::new(), &mut result);
        result
    }

    fn dfs(rev_input: &mut Vec<char>, appender: &mut Vec<char>, result: &mut Vec<String>) {
        if let Some(digit) = rev_input.pop() {
            for x in &MAPPING[digit as usize - '0' as usize - 2] {
                appender.push(*x);
                Self::dfs(rev_input, appender, result);
                appender.pop();
            }
            rev_input.push(digit);
        } else {
            result.push(appender.iter().collect::<String>());
        }
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::letter_combinations("2".to_string()),
        ["a", "b", "c"]
    );

    assert_eq!(
        Solution::letter_combinations("23".to_string()),
        ["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]
    );
}

struct Solution;
