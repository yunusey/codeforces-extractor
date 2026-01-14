mod parser;

use std::fs;

use chromiumoxide::browser::{Browser, BrowserConfig};
use clap::Parser;
use futures::StreamExt;
use parser::{parse_content, Problem};

use std::process::{Child, Command};
use std::{env, thread, time::Duration};

#[derive(Debug, Parser)]
struct Args {
    /// Contest Name
    contest_name: String,

    /// Save Path
    #[arg(short, long, default_value = "./problems")]
    save_path: std::path::PathBuf,

    /// Use native display (no Xvfb)
    #[arg(short, long, default_value_t = false)]
    native_display: bool,
}

fn start_xvfb() -> Child {
    let display = ":99";
    let child = Command::new("Xvfb")
        .args([display, "-screen", "0", "1280x800x24", "-nolisten", "tcp"])
        .spawn()
        .expect("Failed to start Xvfb");

    thread::sleep(Duration::from_millis(500));
    env::set_var("DISPLAY", display);
    child
}

async fn get_content(
    contest_name: &str,
    force_x11: bool,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut config_builder = BrowserConfig::builder().with_head();
    if force_x11 {
        config_builder = config_builder.arg("--ozone-platform=x11") // Ensure X11 platform is used for Xvfb
    }
    let config = config_builder.build()?;

    let (browser, mut handler) = Browser::launch(config).await?;
    tokio::spawn(async move { while let Some(_) = handler.next().await {} });

    let url = format!("https://codeforces.com/contest/{}/problems", contest_name);
    let page = browser.new_page(&url).await?;
    let html = page.content().await?;

    Ok(html)
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

    let xvfb_child = if args.native_display {
        None
    } else {
        Some(start_xvfb())
    };
    let content = get_content(&args.contest_name, !args.native_display)
        .await
        .unwrap_or_else(|_| {
            panic!("Failed to get content for contest: {}", &args.contest_name);
        });

    let problems: Vec<Problem> = parse_content(&content);
    for problem in problems {
        save_problem(&problem, &args.save_path);
    }

    drop(xvfb_child);
}

#[tokio::test]
async fn test_parser_1790() {
    let content = get_content("1790", false).await.unwrap();
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

#[tokio::test]
async fn test_parser_2184() {
    let content = get_content("2184", false).await.unwrap();
    let problems: Vec<Problem> = parse_content(&content);
    assert_eq!(
        problems
            .iter()
            .map(|p: &Problem| p.name.clone())
            .collect::<Vec<String>>(),
        vec![
            "A. Social Experiment".to_string(),
            "B. Hourglass".to_string(),
            "C. Huge Pile".to_string(),
            "D. Unfair Game".to_string(),
            "E. Exquisite Array".to_string(),
            "F. Cherry Tree".to_string(),
            "G. Nastiness of Segments".to_string(),
        ]
    );
}
