use std::collections::HashMap;

/* 
给定一个整数数组nums和一个整数目标值target，请你在该数组中找出和为目标值target的那两个整数，
并返回它们的数组下标。
你可以假设每种输入只会对应一个答案。但是，数组中同一个元素在答案里不能重复出现。
你可以按任意顺序返回答案。
*/
fn main() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    let result = two_sum2(nums, target);
    if !result.is_empty() {
        dbg!(result);
    } else {
        dbg!("no such");
    }
}

// 暴力解法
fn two_sum1(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for i in 0..nums.len() {
        for j in (i + 1)..nums.len() {
            if nums[i] + nums[j] == target {
                return vec![i as i32, j as i32];
            }
        }
    }
    return vec![];
}

// 哈希表解法，时间复杂度O(n)
fn two_sum2(nums: Vec<i32>, target: i32) -> Vec<i32> {
    // 初始化hash表
    let mut map = HashMap::new();
    // 遍历数组
    for (i, &num) in nums.iter().enumerate() {
        // 计算补数
        let complement = target - num;
        // 查找补数下标
        if let Some(&complement_index) = map.get(&complement) {
            return vec![complement_index as i32, i as i32];
        }
        // 插入当前元素
        map.insert(num, i);
    }
    vec![]
}