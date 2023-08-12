use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn partition_string(s: String) -> i32 {
        let mut count = 1;
        let mut set: HashSet<char> = HashSet::new();
        for c in s.chars() {
            if set.contains(&c) {
                count += 1;
                set.clear();
            }
            set.insert(c);
        }
        return count;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn abacaba() {
        assert_eq!(Solution::partition_string(String::from("abacaba")), 4);
    }
    #[test]
    fn all_same() {
        assert_eq!(Solution::partition_string(String::from("aaaaa")), 5);
    }
}
