impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        Self::one_pointer_and_two_pointer_close(nums, target)
    }

    pub fn one_pointer_and_two_pointer_close(nums: Vec<i32>, target: i32) -> i32 {
        let mut i = 0;
        let mut closest = i32::MAX;
        let mut nums = nums;
        nums.sort();
        while i < nums.len() {
            let mut j = i + 1;
            let mut k = nums.len() - 1;
            while j < k {
                let sum = nums[i] + nums[j] + nums[k];
                if target.abs_diff(sum) < target.abs_diff(closest) {
                    closest = sum;
                }
                if sum < target {
                    j += 1;
                } else if sum > target {
                    k -= 1;
                } else {
                    return sum;
                }
            }
            i += 1;
        }
        closest
    }

    pub fn two_pointer_and_binary_search(nums: Vec<i32>, target: i32) -> i32 {
        trait SearchClosest {
            fn search(&self, start: usize, target: i32) -> usize;
        }

        impl SearchClosest for Vec<i32> {
            fn search(&self, start: usize, target: i32) -> usize {
                let mut l = start;
                let mut r = self.len() - 1;
                let mut pre_mid = (l + r) / 2;
                while l <= r {
                    let mid = (l + r) / 2;
                    if self[mid] == target {
                        return mid;
                    } else if self[mid] < target {
                        l = mid + 1;
                    } else {
                        r = mid - 1;
                    }
                    pre_mid = mid;
                }
                let mut result = pre_mid;
                if pre_mid - 1 >= start
                    && target.abs_diff(self[result]) > target.abs_diff(self[pre_mid - 1])
                {
                    result = pre_mid - 1;
                };
                if pre_mid + 1 < self.len()
                    && target.abs_diff(self[result]) > target.abs_diff(self[pre_mid + 1])
                {
                    result = pre_mid + 1;
                };
                result
            }
        }

        let mut closest_result: i32 = i32::MAX;

        let mut sorted = nums.clone();
        sorted.sort();
        let mut i = 0;
        while i < sorted.len() - 2 {
            let mut j = i + 1;
            while j < sorted.len() - 1 {
                let new_target = target - sorted[j] - sorted[i];
                let closest = sorted.search(j + 1, new_target);
                let new_sum = sorted[i] + sorted[j] + sorted[closest];
                if target.abs_diff(new_sum) < target.abs_diff(closest_result) {
                    closest_result = new_sum;
                }
                j += 1;
            }
            i += 1;
        }
        closest_result
    }
}

#[test]
fn test() {
    assert_eq!(Solution::three_sum_closest(vec![-1, 2, 1, -4], 1), 2);
    assert_eq!(Solution::three_sum_closest(vec![0, 0, 0], 1), 0);
    assert_eq!(
        Solution::three_sum_closest(vec![-1000, -5, -5, -5, -5, -5, -5, -1, -1, -1], -14),
        -15
    );
}

struct Solution;
