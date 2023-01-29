use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}

// Function that returns the indexes of two numbers that sum to a target value
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = HashMap::new();
    for (index, element) in nums.iter().enumerate() {
        let pair = map.get(&(target - element));
        if let Some(&pair_index) = pair {
            return vec![pair_index as i32, index as i32];
        } else {
            map.insert(element, index);
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0, 1])
    }

    #[test]
    fn case2() {
        assert_eq!(two_sum(vec![3, 2, 4], 6), vec![1, 2])
    }

    #[test]
    fn case3() {
        assert_eq!(two_sum(vec![3, 3], 6), vec![0, 1])
    }
}
