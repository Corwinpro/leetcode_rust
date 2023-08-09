pub struct Solution {}

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        match haystack.find(needle.as_str()) {
            Some(val) => return val as i32,
            None => return -1,
        }
    }
}
