impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        println!("{:?}", nums);
        let mut i = 0;
        let mut j = nums.len() - 1;
        let mut sorted_num: Vec<(i32, usize)> = nums.clone().iter().enumerate().map(|(i, x)| (x.clone(), i)).collect();
        sorted_num.sort();
        while i < j {
            let cur = sorted_num[i].0 + sorted_num[j].0;
            if cur == target {
                break;
            }
            if cur < target {
                i += 1;
            }
            else {
                j -= 1;
            }
        }
        vec![sorted_num[i].1 as i32, sorted_num[j].1 as i32]
    }
}
