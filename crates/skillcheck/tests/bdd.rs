//! Runs the Gherkin features in /features against skillcheck.
//! Each scenario gets a throwaway library folder, so nothing touches the real skills/.

use cucumber::{gherkin::Step, given, then, when, World};
use skillcheck::{check_repo, Level, Report};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Default, World)]
struct Library {
    dir: Option<tempfile::TempDir>,
    report: Option<Report>,
}

impl Library {
    fn root(&mut self) -> PathBuf {
        self.dir
            .get_or_insert_with(|| tempfile::tempdir().expect("temp dir"))
            .path()
            .to_path_buf()
    }
    fn report(&self) -> &Report {
        self.report.as_ref().expect("run the check first")
    }
}

fn docstring(step: &Step) -> String {
    step.docstring
        .as_ref()
        .expect("this step needs a \"\"\" block")
        .trim_start_matches('\n')
        .to_string()
}

#[given(expr = "raea {string} is the current upstream snapshot")]
fn upstream_version(lib: &mut Library, version: String) {
    let up = lib.root().join("upstream");
    fs::create_dir_all(&up).unwrap();
    fs::write(up.join("VERSION"), version).unwrap();
}

#[given("the upstream raea help says:")]
fn upstream_help(lib: &mut Library, step: &Step) {
    let up = lib.root().join("upstream");
    fs::create_dir_all(&up).unwrap();
    fs::write(up.join("help.txt"), docstring(step)).unwrap();
}

#[given(expr = "a skill folder {string} with this SKILL.md:")]
fn skill_folder(lib: &mut Library, step: &Step, name: String) {
    let dir = lib.root().join("skills").join(name);
    fs::create_dir_all(&dir).unwrap();
    fs::write(dir.join("SKILL.md"), docstring(step)).unwrap();
}

#[given(expr = "an empty skill folder {string}")]
fn empty_folder(lib: &mut Library, name: String) {
    fs::create_dir_all(lib.root().join("skills").join(name)).unwrap();
}

#[given(expr = "the plugin manifest lists the skills {string}")]
fn plugin_lists(lib: &mut Library, entries: String) {
    let list: Vec<String> = entries
        .split_whitespace()
        .map(|e| format!("{e:?}"))
        .collect();
    let manifest = format!("{{\"name\": \"raea\", \"skills\": [{}]}}", list.join(", "));
    plugin_manifest(lib, manifest);
}

#[given(expr = "the plugin manifest is {string}")]
fn plugin_manifest(lib: &mut Library, text: String) {
    let dir = lib.root().join("skills/.claude-plugin");
    fs::create_dir_all(&dir).unwrap();
    fs::write(dir.join("plugin.json"), text).unwrap();
}

#[when("I run the skill check")]
fn run_check(lib: &mut Library) {
    let root = lib.root();
    lib.report = Some(check_repo(&root.join("skills"), &root.join("upstream")).unwrap());
}

#[then("it passes with no warnings")]
fn passes_clean(lib: &mut Library) {
    let r = lib.report();
    assert!(
        r.findings.is_empty(),
        "expected nothing, got: {:#?}",
        r.findings
    );
}

#[then("it passes")]
fn passes(lib: &mut Library) {
    let r = lib.report();
    assert!(r.passed(false), "expected a pass, got: {:#?}", r.findings);
}

#[then("it fails")]
fn fails(lib: &mut Library) {
    assert!(!lib.report().passed(false), "expected a failure");
}

#[then(expr = "an error mentions {string}")]
fn error_mentions(lib: &mut Library, text: String) {
    let r = lib.report();
    assert!(
        r.findings
            .iter()
            .any(|f| f.level == Level::Error && f.message.contains(&text)),
        "no error mentions {text:?}: {:#?}",
        r.findings
    );
}

#[then(expr = "a warning mentions {string}")]
fn warning_mentions(lib: &mut Library, text: String) {
    let r = lib.report();
    assert!(
        r.findings
            .iter()
            .any(|f| f.level == Level::Warn && f.message.contains(&text)),
        "no warning mentions {text:?}: {:#?}",
        r.findings
    );
}

#[then(expr = "{int} skill(s) were checked")]
fn count_checked(lib: &mut Library, n: usize) {
    assert_eq!(lib.report().skills_checked, n);
}

#[tokio::main]
async fn main() {
    let features = concat!(env!("CARGO_MANIFEST_DIR"), "/../../features");
    Library::cucumber().run_and_exit(features).await;
}
