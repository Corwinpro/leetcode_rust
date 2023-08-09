pub struct Solution {}

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        return s.split_ascii_whitespace().last().unwrap().len() as i32;
    }

    pub fn length_via_char_iter(s: String) -> i32 {
        s.chars()
            .rev()
            .map(|c| (if c == ' ' { 0 } else { 1 }))
            .try_fold(0, |acc, x| {
                if acc > 0 && x == 0 {
                    return Err(acc);
                }
                Ok(acc + x)
            })
            .unwrap_or_else(|err| err)
    }
}
