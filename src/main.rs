use crate::fetch_problem::{Fetcher, ProblemManager};
use std::fs;
use std::io::Write;
use std::path::Path;

mod fetch_problem;

#[tokio::main]
async fn main() {
    let manager = ProblemManager::scan().expect("Failed to scan mod file.");
    let fetcher = Fetcher::new();

    let args: Vec<String> = std::env::args().collect();
    let id = args[1].parse::<u32>();

    match id {
        Ok(id) => {
            if manager.problem_list.contains(&id) {
                eprintln!("Problem {} exists.", id);
                return;
            }

            println!("Try to get problem {}...", id);

            let problem = fetcher
                .get_problem(id)
                .await
                .expect(&*format!("Failed to get problem {}.", id));

            let file_name = problem.get_filename();
            println!("Get problem: {}.", file_name);
            let content = problem
                .get_file_content()
                .expect("Failed to format file content");

            write_file(&file_name, &content).expect("Failed to write problem file.");
        }
        Err(_) => {
            eprintln!("Get operation needs a usize param.")
        }
    }
}

fn write_file(file_name: &String, file_content: &String) -> std::io::Result<()> {
    let file_path = Path::new("./src/problem").join(format!("{}.rs", file_name));

    let mut file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&file_path)?;

    file.write_all(file_content.as_bytes())?;
    drop(file);

    let mut mod_file = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open("./src/problem/mod.rs")?;

    write!(mod_file, "\nmod {};", file_name)
}
