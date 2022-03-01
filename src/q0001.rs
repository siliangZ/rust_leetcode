pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut nums: Vec<(usize, i32)> = (0..nums.len()).zip(nums).collect();
    nums.sort_by_key(|(_, value)| *value);
    let mut start = 0;
    let mut end = nums.len() - 1;
    while start < end {
        let sum = nums[start].1 + nums[end].1;
        if sum < target {
            start += 1;
        } else if sum > target {
            end -= 1;
        } else {
            return vec![nums[start].0 as i32, nums[end].0 as i32];
        }
    }
    vec![-1, -1]
}
