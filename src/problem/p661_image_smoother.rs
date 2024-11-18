/**
 * [661] Image Smoother
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn image_smoother(img: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let (m, n) = (img.len(), img[0].len());
        let mut result = vec![vec![0; n]; m];

        let add = |x: usize, y: usize| {
            img[x][y]
                + if x + 1 < m { img[x + 1][y] } else { 0 }
                + if x > 0 { img[x - 1][y] } else { 0 }
        };

        let grid =
            |x: usize, y: usize| 1 + if x + 1 < m { 1 } else { 0 } + if x > 0 { 1 } else { 0 };

        for x in 0..m {
            let mut sum = add(x, 0);
            let mut g = grid(x, 0);

            for y in 0..n {
                if y + 1 < n {
                    sum += add(x, y + 1);
                    g += grid(x, y + 1);
                }
                if y > 1 {
                    sum -= add(x, y - 2);
                    g -= grid(x, y - 2);
                }

                result[x][y] = sum / g;
            }
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_661() {
        assert_eq!(
            vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0]],
            Solution::image_smoother(vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]])
        );
        assert_eq!(
            vec![
                vec![137, 141, 137],
                vec![141, 138, 141],
                vec![137, 141, 137]
            ],
            Solution::image_smoother(vec![
                vec![100, 200, 100],
                vec![200, 50, 200],
                vec![100, 200, 100]
            ])
        );
    }
}
