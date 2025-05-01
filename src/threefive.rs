impl Solution {
    fn search_insert(mut nums: Vec<i32>, target: i32) -> i32 {
    if !nums.contains(&target) {
        nums.push(target);
    }
    nums.sort();
    for (index, value) in nums.iter().enumerate(){ 
        if *value == target{
            return index as i32
        }
    }
    nums.len() as i32
}
}
