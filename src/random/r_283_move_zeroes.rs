/*
    Given an integer array nums, move all 0's to the end of it while maintaining the relative order of the non-zero elements.

    Note that you must do this in-place without making a copy of the array.



    Example 1:

    Input: nums = [0,1,0,3,12]
    Output: [1,3,12,0,0]
    Example 2:

    Input: nums = [0]
    Output: [0]


    Constraints:

    1 <= nums.length <= 104
    -231 <= nums[i] <= 231 - 1


    Follow up: Could you minimize the total number of operations done?
*/

pub fn move_zeroes(nums: &mut Vec<i32>) {
    let len = nums.len();

    nums.retain(|&n| n != 0);

    nums.extend(vec![0; len - nums.len()]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_zeroes_with_mixed_elements() {
        let mut nums = vec![0, 1, 0, 3, 12];
        move_zeroes(&mut nums);
        assert_eq!(nums, vec![1, 3, 12, 0, 0]);
    }

    #[test]
    fn move_zeroes_with_zero_elements() {
        let mut nums = vec![0];
        move_zeroes(&mut nums);
        assert_eq!(nums, vec![0]);
    }
}