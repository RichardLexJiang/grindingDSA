impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        Self::index_mapping(s, num_rows)
    }

    /**
        Calculating the coordinate of the character according to its index.
    */
    fn index_mapping(s: String, num_rows: i32) -> String {
        if num_rows == 1 {
            s
        } else {
            let mut new_arr: Vec<(char, i32, i32)> = s
                .chars()
                .enumerate()
                .map(|(i, c)| {
                    let coordinator = Self::decode_index_to_coor(i, num_rows);
                    (c, coordinator.0, coordinator.1)
                })
                .collect();
            new_arr.sort_by(|(_, x0, y0), (_, x1, y1)| x0.cmp(&x1).then_with(|| y0.cmp(&y1)));
            new_arr.iter().map(|(c, _, _)| c).collect::<String>()
        }
    }

    fn decode_index_to_coor(i: usize, num_rows: i32) -> (i32, i32) {
        let group_count = 2 * (num_rows - 1);
        let nth_group = i as i32 / group_count;
        let nth_in_group = i as i32 % group_count;
        let group_start_x = nth_group * (num_rows - 1);
        if nth_in_group < num_rows {
            (nth_in_group, group_start_x)
        } else {
            (
                group_count - nth_in_group,
                group_start_x + nth_in_group - num_rows + 1,
            )
        }
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::convert("PAYPALISHIRING".to_string(), 3),
        "PAHNAPLSIIGYIR"
    );
    assert_eq!(
        Solution::convert("PAYPALISHIRING".to_string(), 4),
        "PINALSIGYAHRPI"
    );
    assert_eq!(
        Solution::convert("PAYPALISHIRING".to_string(), 1),
        "PAYPALISHIRING"
    );
    assert_eq!(Solution::convert("A".to_string(), 1), "A");
}

struct Solution;
