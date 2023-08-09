pub struct Solution {}

fn swap(i: usize, j: usize, last: usize, matrix: &mut Vec<Vec<i32>>) {
    let first = matrix[i][j];
    let second = matrix[j][last - i];
    let third = matrix[last - i][last - j];
    let fourth = matrix[last - j][i];

    matrix[j][last - i] = first;
    matrix[last - i][last - j] = second;
    matrix[last - j][i] = third;
    matrix[i][j] = fourth;
}

impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let length = matrix.len();
        let (half_length, rem) = (length / 2, length.rem_euclid(2));

        let last = length - 1;
        for i in 0..half_length {
            for j in 0..half_length {
                swap(i, j, last, matrix)
            }
        }

        if rem != 0 {
            for i in 0..half_length {
                swap(i, half_length, last, matrix)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::s0048_rotate_image::Solution;

    #[test]
    fn even() {
        let mut vec = vec![
            vec![5, 1, 9, 11],
            vec![2, 4, 8, 10],
            vec![13, 3, 6, 7],
            vec![15, 14, 12, 16],
        ];
        Solution::rotate(&mut vec);
        assert_eq!(
            vec,
            vec![
                vec![15, 13, 2, 5],
                vec![14, 3, 4, 1],
                vec![12, 6, 8, 9],
                vec![16, 7, 10, 11]
            ]
        );
    }

    #[test]
    fn odd() {
        let mut vec = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        Solution::rotate(&mut vec);
        assert_eq!(vec, vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3]]);
    }
}
