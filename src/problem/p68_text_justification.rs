/**
 * [68] Text Justification
 */
pub struct Solution {}


// submission codes start here

impl Solution {
    pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
        let max_width = max_width as usize;
        let mut pos = 0;
        let mut result = Vec::with_capacity(words.len());

        let mut line_length = 0;
        let mut lines : Vec<String> = Vec::new();
        while pos < words.len() {
            if line_length + words[pos].len() > max_width {
                let line_count = lines.len();
                let mut line = String::new();
                if line_count == 1 {
                    line.push_str(&lines[0]);

                    for i in 0..(max_width - lines[0].len()) {
                        line.push(' ');
                    }

                    result.push(line);
                    lines.clear();
                    line_length = 0;
                    continue;
                }

                let actual_line_length: usize = lines.iter().map(|word| word.len()).sum();
                let mut mod_length = (max_width - actual_line_length) % (line_count - 1);
                let average_length = (max_width - actual_line_length) / (line_count - 1); 

                for i in 0..line_count {
                    line.push_str(&lines[i]);

                    if i != line_count - 1 {
                        let space_length = if mod_length > 0 {
                            mod_length -= 1;
                            1 + average_length
                        } else {
                            average_length
                        };

                        for j in 0..space_length {
                            line.push(' ');
                        }                        
                    } 
                }

                result.push(line);
                lines.clear();
                line_length = 0;
                continue;
            }

            line_length += words[pos].len() + 1;
            lines.push(words[pos].clone());
            pos += 1;
        }

        if lines.len() != 0 {
            let mut line = String::new();

            for i in 0..lines.len() {
                line.push_str(&lines[i]);

                if i != lines.len() - 1 {
                    line.push(' ');
                }
            }

            for j in 0..(max_width - line.len()) {
                line.push(' ');
            }
            result.push(line);
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_68() {
        let words = vec!["This", "is", "an", "example", "of", "text", "justification."];
        let words: Vec<String> = words.iter().map(|word| {String::from(*word)}).collect();

        for line in Solution::full_justify(words, 16) {
            println!("{}", line);
        }
    }
}
