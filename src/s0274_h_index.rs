pub struct Solution {}

impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        let mut citations = citations;
        citations.retain(|x| x > &0);
        citations.sort();
        match citations
            .iter()
            .rev()
            .enumerate()
            .skip_while(|(i, &v)| v as usize >= i + 1)
            .next()
        {
            // In both cases, this is `citations[i - 1].min(i as i32)`
            // but we know that `citations[i - 1]` >= i because
            // of the pre-conditioner above, so `i as i32` is safe
            None => return citations.len() as i32,
            Some((i, _)) => {
                return i as i32;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(Solution::h_index(vec![3, 0, 6, 1, 5]), 3);
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::h_index(vec![1, 3, 1]), 1);
    }

    #[test]
    fn example_single() {
        assert_eq!(Solution::h_index(vec![42]), 1);
    }

    #[test]
    fn example_zeroes() {
        assert_eq!(Solution::h_index(vec![0, 0, 0]), 0);
        assert_eq!(Solution::h_index(vec![0]), 0);
    }

    #[test]
    fn example_few_arcticles_many_cites() {
        assert_eq!(Solution::h_index(vec![11, 15]), 2);
    }

    #[test]
    fn example_3() {
        assert_eq!(Solution::h_index(vec![1, 4, 7, 9]), 3);
    }
}
