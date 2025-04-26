// impl Solution {
    fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32>{
    for (i_idx, num1) in nums.iter().enumerate() {
        for (j_idx, num2) in nums.iter().enumerate() {
            if num1 + num2 == target && i_idx != j_idx {
                let sol = vec![i_idx as i32, j_idx as i32];
                return sol
            }
            else {
                continue;
            }
        }
    } 
    return vec![]
}
// }
