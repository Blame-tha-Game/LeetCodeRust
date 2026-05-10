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


use std::{collections::{HashMap}};

pub fn leet_two_sum() {
    test_two_sum();
}

/*
Approach 1: Brute Force

Algorithm

The brute force approach is simple. Loop through each element x and find if there is another value that equals to target−x.

Complexity Analysis

    Time complexity: O(n2).
    For each element, we try to find its complement by looping through the rest of the array which takes O(n) time. Therefore, the time complexity is O(n2).

    Space complexity: O(1).
    The space required does not depend on the size of the input array, so only constant space is used.

*/
// Note: I made this without looking anything up! Yay me! First try!
#[allow(dead_code)]
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
#[allow(dead_code)]
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

/*
Approach 2: Two-pass Hash Table

Intuition

To improve our runtime complexity, we need a more efficient way to check if the complement exists in the array. If the complement exists, we need to get its index. What is the best way to maintain a mapping of each element in the array to its index? A hash table.

We can reduce the lookup time from O(n) to O(1) by trading space for speed. A hash table is well suited for this purpose because it supports fast lookup in near constant time. I say "near" because if a collision occurred, a lookup could degenerate to O(n) time. However, lookup in a hash table should be amortized O(1) time as long as the hash function was chosen carefully.

Algorithm

A simple implementation uses two iterations. In the first iteration, we add each element's value as a key and its index as a value to the hash table. Then, in the second iteration, we check if each element's complement (target−nums[i]) exists in the hash table. If it does exist, we return current element's index and its complement's index. Beware that the complement must not be nums[i] itself!

Complexity Analysis

    Time complexity: O(n).
    We traverse the list containing n elements exactly twice. Since the hash table reduces the lookup time to O(1), the overall time complexity is O(n).

    Space complexity: O(n).
    The extra space required depends on the number of items stored in the hash table, which stores exactly n elements.

*/
//Note: Still made this without looking anything up but it was not the best solution! :(
#[allow(dead_code)]
fn hash_two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut return_vec: Vec<i32> = Vec::new();
    let mut i: usize = 0;
    let mut hash: HashMap<i32, usize> = HashMap::new();
    while i < nums.len() {
        match hash.entry(nums[i]) {
            std::collections::hash_map::Entry::Occupied(_entry) => {
                if (nums[i] * 2) == target {
                    return_vec.push(*hash.get(&nums[i]).unwrap() as i32);
                    return_vec.push(i as i32);
                    return return_vec; 
                }
            }
            std::collections::hash_map::Entry::Vacant(_entry) => {
                hash.insert(nums[i], i);
            }
        }
        i += 1;
    }
    let mut x: usize = 0;
    let mut y: usize = nums.len() - 1;
    let mut temp_vec= nums.clone();
    temp_vec.sort();
    while x < y {
        let current_num = temp_vec[x] + temp_vec[y];
        if current_num == target {
            return_vec.push(*hash.get(&temp_vec[x]).unwrap() as i32);
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

/*
Approach 3: One-pass Hash Table

Algorithm

It turns out we can do it in one-pass. While we are iterating and inserting elements into the hash table, we also look back to check if current element's complement already exists in the hash table. If it exists, we have found a solution and return the indices immediately.

Complexity Analysis

    Time complexity: O(n).
    We traverse the list containing n elements only once. Each lookup in the table costs only O(1) time.

    Space complexity: O(n).
    The extra space required depends on the number of items stored in the hash table, which stores at most n elements.

*/
fn better_hash_two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut return_vec: Vec<i32> = Vec::new();
    let mut hash: HashMap<i32, usize> = HashMap::new();
    let mut i: usize = 0;
    let mut complement: i32;
    while i < nums.len() {
        complement = target - nums[i];
        match hash.entry(complement) {
            std::collections::hash_map::Entry::Occupied(_entry) => {
                return_vec.push(*hash.get(&complement).unwrap() as i32);
                return_vec.push(i as i32);
                return return_vec; 
            }
            std::collections::hash_map::Entry::Vacant(_entry) => {
                hash.insert(nums[i], i);
            }
        }
        i += 1;
    }

    return return_vec; 
}

fn test_two_sum() -> bool {
    let vec = vec![1, 2, 3, 4];
    let target = 7;
    let ex0_response: Vec<i32> = better_hash_two_sum(vec, target);
    if ex0_response.len() < 2 { print!("ERROR EX0 LEN: {}", ex0_response.len()); return false;}
    let vec_aws = vec![2, 3];
    if ex0_response != vec_aws { print!("ERROR EX0: {:?}", ex0_response); return false; }

    /*
    Example 1:
    Input: nums = [2,7,11,15], target = 9
    Output: [0,1]
     */
    let vec = vec![2,7,11,15];
    let target = 9;
    let ex1_response: Vec<i32> = better_hash_two_sum(vec, target);
    if ex1_response.len() < 2 { print!("ERROR EX1 LEN: {}", ex1_response.len()); return false;}
    let vec_aws = vec![0, 1];
    if ex1_response != vec_aws{ print!("ERROR EX1: {:?}", ex1_response); return false; }

    /*
    Example 2:
    Input: nums = [3,2,4], target = 6
    Output: [1,2]
     */
    let vec = vec![3,2,4];
    let target = 6;
    let ex2_response: Vec<i32> = better_hash_two_sum(vec, target);
    if ex2_response.len() < 2 { print!("ERROR EX2 LEN: {}", ex2_response.len()); return false;}
    let vec_aws = vec![1, 2];
    if ex2_response != vec_aws { print!("ERROR EX2: {:?}", ex2_response); return false; }

    /*
    Example 3:
    Input: nums = [3,3], target = 6
    Output: [0,1]
     */
    let vec = vec![3,3];
    let target = 6;
    let ex3_response: Vec<i32> = better_hash_two_sum(vec, target);
    if ex3_response.len() < 2 { print!("ERROR EX3 LEN: {}", ex3_response.len()); return false;}
    let vec_aws = vec![0, 1];
    if ex3_response != vec_aws { print!("ERROR EX3: {:?}", ex3_response); return false; }

    /*
    Example 54:
    Input: nums = [1,1,1,1,1,4,1,1,1,1,1,7,1,1,1,1,1], target = 11
    Output: [6,12]
     */
    let vec = vec![1,1,1,1,1,4,1,1,1,1,1,7,1,1,1,1,1];
    let target = 11;
    let ex54_response: Vec<i32> = better_hash_two_sum(vec, target);
    if ex54_response.len() < 2 { print!("ERROR EX54 LEN: {}", ex54_response.len()); return false;}
    let vec_aws = vec![5, 11];
    if ex54_response != vec_aws { print!("ERROR EX54: {:?}", ex54_response); return false; }

    return true;
}