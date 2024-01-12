use serde_derive::{Deserialize, Serialize};
use serde_json::json;

pub mod fetcher;
pub mod manager;

#[derive(Serialize, Deserialize)]
pub struct CodeDefinition {
    pub value: String,
    pub text: String,
    #[serde(rename = "defaultCode")]
    pub default_code: String
}

/// LeetCode 单个问题
pub struct Problem {
    pub title: String,
    pub title_slug: String,
    pub content: String,
    pub code_definition: Vec<CodeDefinition>,
    pub question_id: u32,
    pub return_type: String
}

#[derive(Serialize, Deserialize)]
pub struct Stat {
    question_id: u32,
    #[serde(rename = "question__article__slug")]
    question_article_slug: Option<String>,
    #[serde(rename = "question__title")]
    pub question_title: Option<String>,
    #[serde(rename = "question__title_slug")]
    pub question_title_slug: Option<String>,
    #[serde(rename = "question__hide")]
    question_hide: bool,
    total_acs: u32,
    total_submitted: u32,
    pub frontend_question_id: String,
    is_new_question: bool,
}

#[derive(Serialize, Deserialize)]
pub struct StatWithStatus {
    pub stat: Stat,
    pub paid_only: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Problems {
    pub stat_status_pairs: Vec<StatWithStatus>
}

const QUESTION_QUERY_STRING: &str = r#"
query questionData($titleSlug: String!) {
    question(titleSlug: $titleSlug) {
        content
        stats
        codeDefinition
        sampleTestCase
        metaData
    }
}"#;
const QUESTION_QUERY_OPERATION: &str = "questionData";

/// 题目查询
#[derive(Serialize, Deserialize)]
pub struct Query {
    #[serde(rename = "operationName")]
    operation_name: String,
    variables: serde_json::Value,
    query: String
}

impl Query {
    pub fn new(title: &str) -> Query {
        Query {
            operation_name: QUESTION_QUERY_OPERATION.to_owned(),
            variables: json!({
                "titleSlug": title
            }),
            query: QUESTION_QUERY_STRING.to_owned()
        }
    }
}

pub struct Fetcher {
    client: reqwest::Client
}

pub struct ProblemManager {
    pub problem_list: Vec<u32>,
}