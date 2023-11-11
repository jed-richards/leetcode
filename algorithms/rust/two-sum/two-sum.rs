impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut ans: Vec<i32> = vec![0;2];
        for i in 0..(nums.len()-1) {
            for j in (i+1)..nums.len() {
                if nums[i] + nums[j] == target {
                    ans[0] = i as i32;
                    ans[1] = j as i32;
                }
            }
        }
        return ans;
    }
}
