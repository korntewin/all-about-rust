use std::collections::HashMap;

pub fn two_sum(nums: Vec<i64>, target: i64) -> Vec<i64> {
    let mut map: HashMap<i64, i64> = HashMap::new();

    for (idx, n) in nums.iter().enumerate() {
        let other_n = target - n;

        if let Some(&other_idx) = map.get(&other_n) {
            return vec![other_idx, idx as i64];
        }

        map.insert(*n, idx as i64);
    }

    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        let result = two_sum(nums, target);
        assert_eq!(result, vec![0, 1]);
    }
}
