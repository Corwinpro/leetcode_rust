pub struct Solution {}

impl Solution {
    pub fn simplify_path(path: String) -> String {
        let mut stack: Vec<&str> = path
            .split("/")
            .filter(|&e| e.len() != 0 && e != ".")
            .collect();

        let mut result = String::new();
        let mut back_counter = 0;
        loop {
            match stack.pop() {
                None => {
                    if result.len() == 0 {
                        result.insert(0, '/');
                    }
                    return result;
                }
                Some(value) => {
                    if value == ".." {
                        back_counter += 1;
                        continue;
                    }
                    if back_counter > 0 {
                        back_counter -= 1;
                        continue;
                    }
                    result.insert_str(0, value);
                    result.insert(0, '/');
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::simplify_path(String::from("/home/")),
            String::from("/home")
        )
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::simplify_path(String::from("/../")),
            String::from("/")
        )
    }

    #[test]
    fn example_3() {
        assert_eq!(
            Solution::simplify_path(String::from("/home//foo/")),
            String::from("/home/foo")
        )
    }

    #[test]
    fn multiple_backs() {
        assert_eq!(
            Solution::simplify_path(String::from("/a/b/c/d/../e/../../")),
            String::from("/a/b")
        )
    }

    #[test]
    fn with_cwd() {
        assert_eq!(
            Solution::simplify_path(String::from("/a/./b/../../c/")),
            String::from("/c")
        )
    }
}
