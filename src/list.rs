use std::collections::HashMap;

/// 给定一个整数数组nums和一个整数目标值target，请你在该数组中找出和为目标值target的那两个整数，
/// 并返回它们的数组下标。

/// 你可以假设每种输入只会对应一个答案，并且你不能使用两次相同的元素。

/// 你可以按任意顺序返回答案。
pub fn to_sum(nums: &[i32], target: i32) -> (i32, i32) {
    //HashMap<A,B>,A是数组存储的值，B是对应下标索引；
    let mut map: HashMap<i32, usize> = HashMap::new();
    for (index, &num) in nums.iter().enumerate() {
        let complement = target - num;

        // 在哈希表中查找 complement
        if let Some(&complement_index) = map.get(&complement) {
            // 如果找到了，返回两个索引
            return (complement_index as i32, index as i32);
        }

        // 如果没找到，将当前的 (num, index) 存入哈希表
        map.insert(num, index);
    }
    // 题目保证有解，所以这里不会执行到
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_sum() {
        // 测试用例 1: nums = [2, 7, 11, 15], target = 9, 期望结果 (0, 1)
        let nums1 = vec![2, 7, 11, 15];
        let target1 = 9;
        let result1 = to_sum(&nums1, target1);
        assert_eq!(result1, (0, 1));

        // 测试用例 2: nums = [3, 2, 4], target = 6, 期望结果 (1, 2)
        let nums2 = vec![3, 2, 4];
        let target2 = 6;
        let result2 = to_sum(&nums2, target2);
        assert_eq!(result2, (1, 2));

        // 测试用例 3: nums = [1, 5, 3, 7], target = 8
        // 可能的配对包括: 1+7=8 (索引 0,3) 或 5+3=8 (索引 1,2)
        let nums3 = vec![1, 5, 3, 7];
        let target3 = 8;
        let result3 = to_sum(&nums3, target3);
        // 由于顺序任意，我们检查所有可能的组合
        assert!((result3 == (0, 3)) || (result3 == (3, 0)) || (result3 == (1, 2)) || (result3 == (2, 1)));
    }
}