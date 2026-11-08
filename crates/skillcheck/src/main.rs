//! `skillcheck [--skills DIR] [--upstream DIR] [--strict]`
//!
//! Exit 0 when the library passes, 1 when it does not. `--strict` also fails on warnings.

use std::path::PathBuf;
use std::process::ExitCode;

fn main() -> ExitCode {
    let mut skills = PathBuf::from("skills");
    let mut upstream = PathBuf::from("upstream/raea");
    let mut strict = false;

    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--skills" => skills = args.next().map(PathBuf::from).unwrap_or(skills),
            "--upstream" => upstream = args.next().map(PathBuf::from).unwrap_or(upstream),
            "--strict" => strict = true,
            "-h" | "--help" => {
                println!("skillcheck [--skills DIR] [--upstream DIR] [--strict]");
                return ExitCode::SUCCESS;
            }
            other => {
                eprintln!("skillcheck: unknown argument `{other}`");
                return ExitCode::from(2);
            }
        }
    }

    let report = match skillcheck::check_repo(&skills, &upstream) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("skillcheck: {e:#}");
            return ExitCode::from(2);
        }
    };

    for f in &report.findings {
        println!("{f}");
    }
    let errors = report.errors().count();
    let warnings = report.warnings().count();
    println!(
        "\n{} skill(s) checked: {errors} error(s), {warnings} warning(s)",
        report.skills_checked
    );

    if report.passed(strict) {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}
