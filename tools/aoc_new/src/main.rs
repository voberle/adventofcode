use std::fs::OpenOptions;
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;
use std::{env, fs, path};

use clap::Parser;
use regex::Regex;
use toml::Table;

/// Starting a new Advent of Code day
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Day to create
    day: u8,
}

fn main() {
    let args = Args::parse();

    // Get all the data
    let year = get_year();
    let day = get_day(&args);
    let day_dir = get_day_dir(day);

    println!("Starting year {}, day {}", year, day);

    let puzzle_statement = read_puzzle(&year, day);
    let title = extract_title(&puzzle_statement);
    let url = build_url(&year, day);
    println!("Title {}; URL: {}", &title, &url);

    // Update year (aka workspace) level things
    update_main_readme(day, &title, &day_dir);
    update_main_cargo(&day_dir);

    // Generate project and change directory
    generate_template(&day_dir);
    change_dir(&day_dir);

    // Update day things
    get_input(&year, day);
    update_project_readme(day, &title, &url);
}

fn get_year() -> String {
    let path = std::env::current_dir().unwrap();
    if let path::Component::Normal(y) = path.components().last().unwrap() {
        let year = y.to_string_lossy();
        if year.contains(|c: char| !c.is_ascii_digit()) {
            panic!("Invalid year {}", year);
        }
        year.to_string()
    } else {
        panic!("Failed to get year");
    }
}

fn get_day(args: &Args) -> u8 {
    assert!(args.day >= 1 && args.day <= 25);
    args.day
}

fn get_day_dir(day: u8) -> String {
    assert!((1..=25).contains(&day));
    format!("day{:0>2}", day)
}

fn generate_template(day: &str) {
    let output = Command::new("cargo")
        .arg("generate")
        .arg("aoc")
        .arg("--name=".to_string() + day)
        .output()
        .expect("Failed to generate template");
    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();
    assert!(output.status.success());
}

fn change_dir(day: &str) {
    let day_dir = Path::new(day);
    assert!(env::set_current_dir(day_dir).is_ok());
    println!(
        "Successfully changed working directory to {}!",
        day_dir.display()
    );
}

fn get_input(year: &str, day: u8) {
    let output = Command::new("aoc")
        .arg("download")
        .arg("--day")
        .arg(day.to_string())
        .arg("--year")
        .arg(year)
        .arg("--input-only")
        .arg("--input-file")
        .arg("resources/input")
        .arg("--overwrite")
        .output()
        .expect("Failed to get input file");
    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();
    assert!(output.status.success());
}

fn read_puzzle(year: &str, day: u8) -> String {
    let output = Command::new("aoc")
        .arg("read")
        .arg("--day")
        .arg(day.to_string())
        .arg("--year")
        .arg(year)
        .output()
        .expect("Failed to read puzzle");
    io::stderr().write_all(&output.stderr).unwrap();
    assert!(output.status.success());
    String::from_utf8(output.stdout).unwrap()
}

fn extract_title(puzzle: &str) -> String {
    let re = Regex::new(r"--- Day (\d+): (.+) ---").unwrap();
    let parts = re.captures(puzzle).unwrap();
    assert_eq!(parts.len(), 3);
    parts[2].to_string()
}

fn build_url(year: &str, day: u8) -> String {
    format!("https://adventofcode.com/{}/day/{}", year, day)
}

// Must be executed from year folder
fn update_main_readme(day: u8, title: &str, day_dir: &str) {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("README.md")
        .unwrap();
    if let Err(e) = writeln!(
        file,
        "\n\n### Day {}: [{}]({}/README.md)",
        day, title, day_dir
    ) {
        eprintln!("Couldn't write to file: {}", e);
    }
}

fn update_main_cargo(day_dir: &str) {
    let cargotoml = fs::read_to_string("Cargo.toml").unwrap();
    let mut values = cargotoml.parse::<Table>().unwrap();
    values["workspace"].as_table_mut().unwrap()["members"]
        .as_array_mut()
        .unwrap()
        .push(day_dir.into());
    fs::write("Cargo.toml", toml::to_string_pretty(&values).unwrap()).unwrap();
}

// Must be executed from day folder
fn update_project_readme(day: u8, title: &str, url: &str) {
    let data = fs::read_to_string("README.md").unwrap();
    let new = data.replace("# \n", &format!("# Day {}: [{}]({})\n", day, title, url));
    fs::write("README.md", new).unwrap();
}
