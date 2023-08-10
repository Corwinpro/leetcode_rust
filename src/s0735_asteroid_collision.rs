pub struct Solution {}

impl Solution {
    pub fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
        let mut asteroids = asteroids;
        let mut index = 0;
        let mut previous_value = std::i32::MIN;
        while index < asteroids.len() {
            let current = asteroids[index];
            if (current > 0) | (previous_value <= 0) {
                previous_value = current;
                index += 1;
                continue;
            }

            // `current` value is negative
            // `previous_value` is positive
            if previous_value + current >= 0 {
                asteroids.remove(index);
            }
            if previous_value + current > 0 {
                continue;
            }

            if asteroids.len() == 0 {
                return asteroids;
            }
            index -= 1;
            asteroids.remove(index);

            if index == 0 {
                previous_value = std::i32::MIN;
            } else {
                previous_value = asteroids[index - 1];
            }
        }
        return asteroids;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(Solution::asteroid_collision(vec![5, 10, -5]), vec![5, 10]);
    }
    #[test]
    fn example_2() {
        assert_eq!(Solution::asteroid_collision(vec![8, -8]), vec![]);
    }
    #[test]
    fn example_3() {
        assert_eq!(Solution::asteroid_collision(vec![-8, -8]), vec![-8, -8]);
    }
    #[test]
    fn example_4() {
        assert_eq!(Solution::asteroid_collision(vec![1, -2]), vec![-2]);
    }

    #[test]
    fn example_5() {
        assert_eq!(Solution::asteroid_collision(vec![1, -2, -2]), vec![-2, -2]);
    }
}
