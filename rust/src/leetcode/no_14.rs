impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut common = String::from("");
        let max = strs.iter().map(String::len).max().unwrap_or(0);
        let vec = strs
            .iter()
            .map(|x| x.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        'first: for i in 0..max {
            if vec[0].len() <= i {
                break;
            }
            for j in 1..vec.len() {
                if (vec[j].len() <= i) || (vec[j][i] != vec[0][i]) {
                    break 'first;
                };
            }
            common.push(vec[0][i]);
        }

        common
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::longest_common_prefix(
            vec!["dog", "racecar", "car"]
                .iter()
                .map(|x| { x.to_string() })
                .collect()
        ),
        ""
    );
    assert_eq!(
        Solution::longest_common_prefix(vec![
            "flower".to_string(),
            "flow".to_string(),
            "flight".to_string()
        ]),
        "fl"
    );
}

struct Solution {}
