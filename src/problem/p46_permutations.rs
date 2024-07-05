/**
 * [46] Permutations
 */
pub struct Solution {}


// submission codes start here

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = vec![];
        let mut output = nums.clone();

        Self::search(output.len(), &mut output, &mut result, 0);

        result
    }

    fn search(length: usize, output: &mut Vec<i32>, result: &mut Vec<Vec<i32>>, pos: usize) {
        if length == pos {
            result.push(output.clone());
            return;
        }
        
        for i in pos..length {
            Self::swap(output, i, pos);
            Self::search(length, output, result, pos + 1);
            Self::swap(output, i, pos);
        }
        
    }
    
    fn swap(output: &mut Vec<i32>, a: usize, b: usize) {
        let temp = output[a];
        output[a] = output[b];
        output[b] = temp;
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_46() {
    }
}
