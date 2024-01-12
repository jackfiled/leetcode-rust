use super::{Problem, ProblemManager};
use std::fs;
use regex::{Regex};

impl ProblemManager {
    pub fn scan() -> Result<ProblemManager, Box<dyn std::error::Error>> {
        let pattern = Regex::new(r"p(\d{4})_")?;
        let mut problems = Vec::new();
        let mod_content = fs::read_to_string("./src/problem/mod.rs")?;

        for i in pattern.captures_iter(&mod_content) {
            match i.get(1) {
                None => {}
                Some(value) => {
                    match value.as_str().parse::<u32>() {
                        Ok(id) => {
                            problems.push(id);
                        }
                        Err(_) => {}
                    }
                }
            }
        }

        Ok(ProblemManager {
            problem_list: problems
        })
    }
}

impl Problem {
    pub fn get_filename(&self) -> String {
        format!("p{}_{}", self.question_id, self.title_slug.replace('-', "_"))
    }

    pub fn get_file_content(&self) -> Result<String, Box<dyn std::error::Error>> {
        let template = fs::read_to_string("./template.rs")?;

        let code = self.code_definition
            .iter()
            .find(|x| x.value == "rust");

        let code = code.ok_or::<Box<dyn std::error::Error>>(
            format!("problem {} doesn't have rust version", self.question_id).into()
        )?;

        let source = template
            .replace("__PROBLEM_TITLE__", &self.title)
            .replace("__PROBLEM_ID__", self.question_id.to_string().as_str())
            .replace(
                "__PROBLEM_DEFAULT_CODE__",
                &code.default_code)
            .replace("__EXTRA_USE__", &parse_extra_use(&code.default_code));

        Ok(source)
    }
}

fn parse_extra_use(code: &str) -> String {
    let mut extra_use_line = String::new();
    // a linked-list problem
    if code.contains("pub struct ListNode") {
        extra_use_line.push_str("\nuse crate::util::linked_list::{ListNode, to_list};")
    }
    if code.contains("pub struct TreeNode") {
        extra_use_line.push_str("\nuse crate::util::tree::{TreeNode, to_tree};")
    }
    if code.contains("pub struct Point") {
        extra_use_line.push_str("\nuse crate::util::point::Point;")
    }
    extra_use_line
}