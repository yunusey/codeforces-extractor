mod parser;

use std::fs;

use clap::Parser;
use parser::{parse_content, Problem};
use reqwest;

#[derive(Debug, Parser)]
struct Args {
    /// Contest Name
    contest_name: String,

    /// Save Path
    #[arg(short, long, default_value = "./problems")]
    save_path: std::path::PathBuf,
}

async fn get_content(contest_name: &str) -> String {
    let client = reqwest::Client::new();
    let url = format!("https://codeforces.com/contest/{}/problems", contest_name);
    let res = client
        .get(&url)
        .header("User-Agent", "Mozilla/5.0") // :D
        .send()
        .await
        .unwrap();
    res.text().await.unwrap()
}

/// Save given `problem` to the given directory `save_path`
fn save_problem(problem: &Problem, save_path: &std::path::Path) {
    let problem_label = problem.name.split_once('.').unwrap().0;
    let problem_path = save_path.join(problem_label);

    fs::create_dir_all(&problem_path).unwrap();

    for (i, input) in problem.inputs.iter().enumerate() {
        let input_path = problem_path.join(format!("{}.in", i));
        std::fs::write(input_path, input).unwrap();
    }
    for (i, output) in problem.outputs.iter().enumerate() {
        let output_path = problem_path.join(format!("{}.out", i));
        std::fs::write(output_path, output).unwrap();
    }
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let content = get_content(&args.contest_name).await;
    let problems: Vec<Problem> = parse_content(&content);

    for problem in problems {
        save_problem(&problem, &args.save_path);
    }
}

#[tokio::test]
async fn test_parser() {
    let content = get_content("1790").await;
    let problems: Vec<Problem> = parse_content(&content);
    assert_eq!(
        problems
            .iter()
            .map(|p: &Problem| p.name.clone())
            .collect::<Vec<String>>(),
        vec![
            "A. Polycarp and the Day of Pi".to_string(),
            "B. Taisia and Dice".to_string(),
            "C. Premutation".to_string(),
            "D. Matryoshkas".to_string(),
            "E. Vlad and a Pair of Numbers".to_string(),
            "F. Timofey and Black-White Tree".to_string(),
            "G. Tokens on Graph".to_string(),
        ]
    );
}
