/*
1. Two Sum

Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.

You may assume that each input would have exactly one solution, and you may not use the same element twice.

You can return the answer in any order. 

Example 1:

Input: nums = [2,7,11,15], target = 9
Output: [0,1]
Explanation: Because nums[0] + nums[1] == 9, we return [0, 1].

Example 2:

Input: nums = [3,2,4], target = 6
Output: [1,2]

Example 3:

Input: nums = [3,3], target = 6
Output: [0,1]

 

Constraints:

    2 <= nums.length <= 104
    -109 <= nums[i] <= 109
    -109 <= target <= 109
    Only one valid answer exists.

 
Follow-up: Can you come up with an algorithm that is less than O(n2) time complexity?
*/

use std::collections::HashMap;

pub fn leet_two_sum() {
    test_two_sum();
}

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut return_vec: Vec<i32> = Vec::new();
    let mut x: usize = 0;
    let mut y: usize;
    'outer: while x < nums.len() {
        y = x + 1;
        while y < nums.len(){
            if nums[x] + nums[y] == target {
                return_vec.push(x as i32);
                return_vec.push(y as i32);
                break 'outer;
            }
            y += 1;
        }
        x += 1;
    }
    return return_vec; 
}

// ops this doesn't work with this because you are returning the index not the num
fn better_two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut return_vec: Vec<i32> = Vec::new();
    let mut x: usize = 0;
    let mut y: usize = nums.len() - 1;
    let mut temp_vec= nums.clone();
    temp_vec.sort();
    while x < y {
        let current_num = temp_vec[x] + temp_vec[y];
        if current_num == target {
            return_vec.push(x as i32);
            return_vec.push(y as i32);
            break;
        }
        else if current_num > target {
            y -= 1;
        }
        else if current_num < target {
            x += 1;
        }
    }
    return return_vec; 
}

fn hash_two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut return_vec: Vec<i32> = Vec::new();
    let mut i: usize = 0;
    let mut hash: HashMap<i32, usize> = HashMap::new();
    while i < nums.len() {
        hash.insert(nums[i], i);
        i += 1;
    }
    let mut x: usize = 0;
    let mut y: usize = nums.len() - 1;
    let mut temp_vec= nums.clone();
    temp_vec.sort();
    while x < y {
        let current_num = temp_vec[x] + temp_vec[y];
        if current_num == target {
            // return_vec.push(x as i32);
            // if let temp = *hash.get(&temp_vec[x]).unwrap() as i32;
            // return_vec.push(hash.get(temp_vec[x]).unwrap());
            return_vec.push(*hash.get(&temp_vec[x]).unwrap() as i32);
            // return_vec.push(y as i32);
            return_vec.push(*hash.get(&temp_vec[y]).unwrap() as i32);
            break;
        }
        else if current_num > target {
            y -= 1;
        }
        else if current_num < target {
            x += 1;
        }
    }
    return return_vec; 
}

fn test_two_sum() -> bool {
    let vec = vec![1, 2, 3, 4];
    let target = 7;
    let ex0_response: Vec<i32> = hash_two_sum(vec, target);
    if ex0_response.len() < 2 { print!("ERROR EX0 LEN"); return false;}
    let vec_aws = vec![2, 3];
    if ex0_response != vec_aws { print!("ERROR EX0: {:?}", ex0_response); return false; }

    /*
    Example 1:
    Input: nums = [2,7,11,15], target = 9
    Output: [0,1]
     */
    let vec = vec![2,7,11,15];
    let target = 9;
    let ex1_response: Vec<i32> = hash_two_sum(vec, target);
    if ex1_response.len() < 2 { print!("ERROR EX1 LEN"); return false;}
    let vec_aws = vec![0, 1];
    if ex1_response != vec_aws{ print!("ERROR EX1: {:?}", ex1_response); return false; }

    /*
    Example 2:
    Input: nums = [3,2,4], target = 6
    Output: [1,2]
     */
    let vec = vec![3,2,4];
    let target = 6;
    let ex2_response: Vec<i32> = hash_two_sum(vec, target);
    if ex2_response.len() < 2 { print!("ERROR EX2 LEN"); return false;}
    let vec_aws = vec![1, 2];
    if ex2_response != vec_aws { print!("ERROR EX2: {:?}", ex2_response); return false; }

    /*
    Example 3:
    Input: nums = [3,3], target = 6
    Output: [0,1]
     */
    let vec = vec![3,3];
    let target = 6;
    let ex3_response: Vec<i32> = hash_two_sum(vec, target);
    if ex3_response.len() < 2 { print!("ERROR EX3 LEN"); return false;}
    let vec_aws = vec![0, 1];
    if ex3_response != vec_aws { print!("ERROR EX3: {:?}", ex3_response); return false; }

    return true;
}