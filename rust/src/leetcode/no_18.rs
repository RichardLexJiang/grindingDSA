impl Solution {
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        if nums.len() < 4 {
            return vec![];
        }
        Self::binary_search(nums.iter().map(|x| *x as i64).collect(), target as i64)
    }

    fn binary_search(mut nums: Vec<i64>, target: i64) -> Vec<Vec<i32>> {
        nums.sort();
        let mut results = Vec::new();
        for i in 0..nums.len() - 3 {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }
            for j in i + 1..nums.len() - 2 {
                if j > i + 1 && nums[j] == nums[j - 1] {
                    continue;
                }
                for k in j + 1..nums.len() - 1 {
                    if k > j + 1 && nums[k] == nums[k - 1] {
                        continue;
                    }
                    let my_target = target - nums[i] - nums[j] - nums[k];
                    // println!("i:{i:?}, j:{j:?}, k:{k:?}, my_target:{my_target:?}");
                    let mut l = k + 1;
                    let mut r = nums.len() - 1;
                    let mut mid = (l + r) / 2;
                    while l < r && nums[mid] != my_target {
                        if nums[mid] > my_target {
                            r = mid - 1;
                        } else if nums[mid] < my_target {
                            l = mid + 1;
                        }
                        mid = (l + r) / 2;
                    }
                    if l <= r && nums[mid] == my_target {
                        results.push(vec![
                            nums[i] as i32,
                            nums[j] as i32,
                            nums[k] as i32,
                            nums[mid] as i32,
                        ]);
                    }
                }
            }
        }

        results
    }
}

/// # 18. 4Sum
/// [link](https://leetcode.com/problems/4sum/description/)
///
/// Given an array nums of n integers, return an array of all the unique quadruplets \[nums\[a\], nums\[b], nums\[c], nums\[d]] such that:
///
/// 0 <= a, b, c, d < n
/// a, b, c, and d are distinct.
/// nums\[a] + nums\[b] + nums\[c] + nums\[d] == target
/// You may return the answer in any order.
///
///
///
/// Example 1:
///
/// Input: nums = \[1,0,-1,0,-2,2\], target = 0
/// Output: \[\[-2,-1,1,2\],\[-2,0,0,2\],\[-1,0,0,1\]]
/// Example 2:
///
/// Input: nums = [2,2,2,2,2], target = 8
/// Output: [[2,2,2,2]]
///
/// Constraints:
///
/// - 1 <= nums.length <= 200
/// - -10^9 <= nums[i] <= 10^9
/// - -10^9 <= target <= 10^9
struct Solution;

#[test]
fn test() {
    let nums = vec![1, 0, -1, 0, -2, 2];
    let target = 0;
    let output = vec![vec![-2, -1, 1, 2], vec![-2, 0, 0, 2], vec![-1, 0, 0, 1]];
    assert_eq!(Solution::four_sum(nums, target), output);

    let nums = vec![1000000000, 1000000000, 1000000000, 1000000000];
    let target = -294967296;
    let output = Vec::<Vec<i32>>::new();
    assert_eq!(Solution::four_sum(nums, target), output);
}
