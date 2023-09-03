use std::collections::HashMap;

pub struct Solution {}

fn counter(str: &str) -> Vec<i32> {
    let mut vec = vec![0; 26];
    for c in str.chars().map(|c| (c as usize) - ('a' as usize)) {
        vec[c] += 1;
    }
    return vec;
}

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map = HashMap::new();
        for str in strs {
            map.entry(counter(&str)).or_insert(Vec::new()).push(str);
        }
        map.into_values().collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_1() {
        let mut result = Solution::group_anagrams(vec![
            "eat".to_string(),
            "tea".to_string(),
            "tan".to_string(),
            "ate".to_string(),
            "nat".to_string(),
            "bat".to_string(),
        ]);
        for entry in &mut result {
            entry.sort();
        }
        result.sort();
        assert_eq!(
            result,
            vec![
                vec!["ate".to_string(), "eat".to_string(), "tea".to_string()],
                vec!["bat".to_string()],
                vec!["nat".to_string(), "tan".to_string()],
            ]
        );
    }
}
