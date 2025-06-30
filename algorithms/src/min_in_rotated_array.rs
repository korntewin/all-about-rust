// https://leetcode.com/problems/find-minimum-in-rotated-sorted-array/

pub fn solve(nums: Vec<i32>) -> i32 {
    if nums.len() == 1 {
        return nums[0];
    }

    let (mut left, mut right) = (0, nums.len() - 1);
    let mut min = i32::MAX;

    while left < right {
        let mid = (left + right) / 2;
        let mid_val = nums[mid];
        let lleft = if mid == 0 { mid } else { mid - 1 };

        if nums[0] < mid_val && nums[lleft] < mid_val {
            // left side low
            min = min.min(nums[0]);
            left = if mid == nums.len() - 1 { mid } else { mid + 1 };
        } else {
            // right side high
            min = min.min(mid_val);
            right = if mid == 0 { mid } else { mid - 1 };
        }
    }

    if left == 0 {
        min.min(nums[left]).min(nums[left + 1])
    } else if right == nums.len() - 1 {
        min.min(nums[left - 1]).min(nums[left])
    } else {
        min.min(nums[left - 1]).min(nums[left]).min(nums[left + 1])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_in_rotated_array() {
        assert_eq!(solve(vec![3, 4, 5, 1, 2]), 1);
        assert_eq!(solve(vec![4, 5, 6, 7, 0, 1, 2]), 0);
        assert_eq!(solve(vec![11, 13, 15, 17]), 11);
    }
}
