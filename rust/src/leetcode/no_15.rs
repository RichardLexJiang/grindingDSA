impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        Self::one_fixed_two_pointer(nums)
    }

    /**
        i < j < k
        fix one point i , then j = i + 1 , k = len - 1
        if nums\[j\] + nums\[k\] < nums\[i\] : move j to right
        if nums\[j\] + nums\[k\] > nums\[i\] : move k to left
        until find the answer
    */
    pub fn one_fixed_two_pointer(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();

        let mut sorted = nums.clone();
        sorted.sort();
        let mut i = 0;
        while i < sorted.len() - 2 {
            let mut j = i + 1;
            let mut k = sorted.len() - 1;
            while j < k {
                if sorted[i] + sorted[j] + sorted[k] == 0 {
                    result.push(vec![sorted[i], sorted[j], sorted[k]]);
                    j = Self::next_number(j, &sorted);
                } else if -sorted[i] > sorted[j] + sorted[k] {
                    j = Self::next_number(j, &sorted);
                } else {
                    k = if let Some(t) = Self::before_number(k, &sorted) {
                        t
                    } else {
                        break;
                    };
                }
            }

            i = Self::next_number(i, &sorted);
        }

        result
    }

    /**
    i<j<k , fix i and j ,
    use binary search to find a k that can make nums\[i\] + nums\[j\] = -nums\[k\]
    */
    #[allow(dead_code)]
    pub fn two_fixed_binary_search(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();

        let mut sorted = nums.clone();
        sorted.sort();
        let mut i = 0;
        while i < sorted.len() - 2 {
            let mut j = i + 1;
            while j < sorted.len() - 1 {
                if let Some(k) = Self::binary_search(&sorted, j + 1, -sorted[i] - sorted[j]) {
                    result.push(vec![sorted[i], sorted[j], sorted[k]]);
                }
                j = Self::next_number(j, &sorted);
            }
            i = Self::next_number(i, &sorted);
        }

        result
    }

    fn next_number(cur: usize, nums: &Vec<i32>) -> usize {
        for i in cur + 1..nums.len() {
            if nums[i] != nums[cur] {
                return i;
            }
        }
        nums.len()
    }

    fn before_number(cur: usize, nums: &Vec<i32>) -> Option<usize> {
        for i in (0..cur).rev() {
            if nums[i] != nums[cur] {
                return Some(i);
            }
        }
        None
    }

    fn binary_search(nums: &Vec<i32>, start: usize, target: i32) -> Option<usize> {
        let mut l = start;
        let mut r = nums.len() - 1;
        while l < r {
            let mid = (l + r) / 2;
            if nums[mid] == target {
                return Some(mid);
            } else if nums[mid] < target {
                l = mid + 1;
            } else {
                r = mid - 1;
            }
        }
        if l == r && nums[l] == target {
            return Some(l);
        }
        None
    }
}

#[test]
fn test() {
    assert_eq!(
        vec![vec![-100, -60, 160], vec![-70, -60, 130]],
        Solution::three_sum(vec![-100, -70, -60, 110, 120, 130, 160])
    );
    assert_eq!(
        vec![vec![-1, -1, 2], vec![-1, 0, 1]],
        Solution::three_sum(vec![-1, 0, 1, 2, -1, -4])
    );
    assert_eq!(vec![vec![0, 0, 0]], Solution::three_sum(vec![0, 0, 0]));
    assert_eq!(Vec::<Vec<i32>>::new(), Solution::three_sum(vec![0, 1, 1]));
}

struct Solution {}
