pub struct Solution {}

#[derive(PartialEq)]
enum Distance {
    Zero,
    One,
    More,
}

fn distance(first: &String, second: &String) -> Distance {
    let mut d = 0;
    for _ in first.chars().zip(second.chars()).filter(|(a, b)| a != b) {
        if d >= 1 {
            return Distance::More;
        }
        d += 1;
    }
    match d {
        0 => Distance::Zero,
        1 => Distance::One,
        _ => Distance::More,
    }
}

impl Solution {
    pub fn min_mutation(start_gene: String, end_gene: String, mut bank: Vec<String>) -> i32 {
        let mut stack: Vec<(i32, String)> = Vec::new();
        stack.push((0, start_gene));

        while !stack.is_empty() {
            let (dist, element) = stack.remove(0);
            if distance(&element, &end_gene) == Distance::Zero {
                return dist;
            }
            bank.retain(|e| {
                if distance(e, &element) == Distance::One {
                    stack.push((dist + 1, String::from(e)));
                    false
                } else {
                    true
                }
            });
        }
        return -1;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::min_mutation(
                String::from("AACCGGTT"),
                String::from("AACCGGTA"),
                vec![String::from("AACCGGTA")]
            ),
            1
        )
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::min_mutation(
                String::from("AACCGGTT"),
                String::from("AAACGGTA"),
                vec![
                    String::from("AACCGGTA"),
                    String::from("AACCGCTA"),
                    String::from("AAACGGTA"),
                ]
            ),
            2
        )
    }
}
