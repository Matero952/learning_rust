fn two_sum(nums: Vec<i32>, target: i32) -> Vec<usize>{
    for (i_idx, num1) in nums.iter().enumerate() {
        for (j_idx, num2) in nums.iter().enumerate() {
            if num1 + num2 == target {
                let sol = vec![i_idx, j_idx];
                return sol
            }
            else {
                continue;
            }
        }
    } 
    return vec![]
}
        

