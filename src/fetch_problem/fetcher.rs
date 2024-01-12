use std::error::Error;
use serde_derive::{Deserialize, Serialize};
use super::{Problem, Problems, Query, Fetcher};

const PROBLEMS_URL: &str = "https://leetcode.cn/api/problems/algorithms/";
const GRAPHQL_URL: &str = "https://leetcode.cn/graphql";

impl Fetcher {
    pub fn new() -> Fetcher {
        Fetcher {
            client: reqwest::Client::new()
        }
    }

    pub async fn get_problems(&self) -> Result<Problems, reqwest::Error> {
        Ok(reqwest::get(PROBLEMS_URL)
            .await?
            .json()
            .await?
        )
    }

    pub async fn get_problem(self, question_id: u32) -> Result<Problem, Box<dyn Error>> {
        let problems = self.get_problems().await?;

        for problem in &problems.stat_status_pairs {
            match problem.stat.frontend_question_id.parse::<u32>() {
                Ok(id) => {
                    if id == question_id {
                        if problem.paid_only {
                            return Err("failed to get paid only problem".into())
                        }

                        let query = match &problem.stat.question_title_slug {
                            None => {
                                Err::<Query, Box<dyn Error>>("failed to get problem title slug".into())
                            }
                            Some(value) => {
                                Ok(Query::new(value.as_str()))
                            }
                        }?;

                        let response = self.client
                            .post(GRAPHQL_URL)
                            .json(&query)
                            .send()
                            .await?
                            .json::<RawProblem>()
                            .await?;

                        let title = problem.stat.question_title.clone()
                            .ok_or::<Box<dyn Error>>("failed to get problem title".into())?;
                        let title_slug = problem.stat.question_title_slug.clone()
                            .ok_or::<Box<dyn Error>>("failed to get problem title slug".into())?;
                        let return_type = {
                            let v = serde_json::from_str::<serde_json::Value>(
                                &response.data.question.meta_data);
                            v.and_then(|x| {
                                return Ok(x.to_string().replace("\"", ""))
                            })
                        }?;
                        let code_definition = serde_json::from_str(
                            &response.data.question.code_definition
                        )?;

                        return Ok(Problem {
                            title,
                            title_slug,
                            code_definition,
                            content: response.data.question.content,
                            question_id: id,
                            return_type
                        })
                    }
                }
                Err(_) => {}
            }
        }

        Err("failed to get target problem".into())
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct RawProblem {
    data: Data,
}

#[derive(Debug, Serialize, Deserialize)]
struct Data {
    question: Question,
}

#[derive(Debug, Serialize, Deserialize)]
struct Question {
    content: String,
    stats: String,
    #[serde(rename = "codeDefinition")]
    code_definition: String,
    #[serde(rename = "sampleTestCase")]
    sample_test_case: String,
    #[serde(rename = "metaData")]
    meta_data: String,
}