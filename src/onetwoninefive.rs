impl Solution {
fn find_numbers(nums: Vec<i32>) -> i32 {
    let mut counter = 0;  
    for i in nums{
        let ax: Vec<char> = i.to_string().chars().collect();
        if (ax.len()) % 2 == 0 {
            counter += 1;
        }
    }
    return counter
}
}
