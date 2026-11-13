//! skillcheck: the gate every raea skill passes before it lands.
//!
//! Three kinds of check, in plain words:
//! 1. **Format**  — is the SKILL.md shaped so an agent can load it?
//! 2. **Drift**   — was it tested against the raea that is current now, and do the
//!    raea commands it relies on still exist?
//! 3. **Opinion** — does it smuggle in taste (fixed colors, "always use…") that would
//!    flatten raea's non-determinism? These are warnings: a human decides.

use anyhow::{Context, Result};
use serde::Deserialize;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::fs;
use std::path::Path;

/// The metadata key naming the raea version a skill was last tested against.
pub const TESTED_AGAINST: &str = "raea-tested-against";
/// The metadata key listing the raea CLI verbs a skill depends on (space separated).
pub const VERBS: &str = "raea-verbs";
/// A line containing this marker is skipped by the opinion check.
pub const OPINION_OK: &str = "opinion-ok";

/// Phrases that usually mean a skill is prescribing taste instead of describing a tool.
pub const OPINION_PHRASES: &[&str] = &[
    "always use",
    "never use",
    "the best ",
    "must look",
    "should feel",
    "the right look",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Level {
    Error,
    Warn,
}

#[derive(Debug, Clone)]
pub struct Finding {
    pub level: Level,
    pub skill: String,
    pub message: String,
}

impl fmt::Display for Finding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let tag = match self.level {
            Level::Error => "FAIL",
            Level::Warn => "warn",
        };
        write!(f, "{tag} {}: {}", self.skill, self.message)
    }
}

#[derive(Debug, Default)]
pub struct Report {
    pub findings: Vec<Finding>,
    pub skills_checked: usize,
}

impl Report {
    pub fn errors(&self) -> impl Iterator<Item = &Finding> {
        self.findings.iter().filter(|f| f.level == Level::Error)
    }
    pub fn warnings(&self) -> impl Iterator<Item = &Finding> {
        self.findings.iter().filter(|f| f.level == Level::Warn)
    }
    pub fn passed(&self, strict: bool) -> bool {
        self.errors().next().is_none() && (!strict || self.warnings().next().is_none())
    }
}

/// The snapshot of raea taken by `just sync-raea` (upstream/raea/).
#[derive(Debug, Clone)]
pub struct Upstream {
    pub version: String,
    pub verbs: BTreeSet<String>,
}

impl Upstream {
    /// Loads upstream/raea/VERSION and help.txt. Returns None if no snapshot exists yet.
    pub fn load(dir: &Path) -> Result<Option<Upstream>> {
        let version_file = dir.join("VERSION");
        if !version_file.exists() {
            return Ok(None);
        }
        let version = fs::read_to_string(&version_file)
            .with_context(|| format!("reading {}", version_file.display()))?
            .trim()
            .to_string();
        let help = fs::read_to_string(dir.join("help.txt")).unwrap_or_default();
        Ok(Some(Upstream {
            version,
            verbs: verbs_in_help(&help),
        }))
    }
}

/// Every word that follows "raea " in the CLI help, e.g. "render", "cut", "skills".
pub fn verbs_in_help(help: &str) -> BTreeSet<String> {
    let mut verbs = BTreeSet::new();
    for line in help.lines() {
        let mut prev = "";
        for tok in line.split_whitespace() {
            if prev == "raea" {
                let word: String = tok
                    .split('|')
                    .next()
                    .unwrap_or("")
                    .chars()
                    .take_while(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || *c == '-')
                    .collect();
                if !word.is_empty() {
                    verbs.insert(word);
                }
            }
            prev = tok;
        }
    }
    verbs
}

#[derive(Debug, Deserialize)]
struct Front {
    name: Option<String>,
    description: Option<String>,
    #[serde(default)]
    metadata: BTreeMap<String, serde_yaml::Value>,
}

fn meta_str(front: &Front, key: &str) -> Option<String> {
    front.metadata.get(key).map(|v| match v {
        serde_yaml::Value::String(s) => s.clone(),
        other => serde_yaml::to_string(other)
            .unwrap_or_default()
            .trim()
            .to_string(),
    })
}

/// Splits a SKILL.md into (frontmatter yaml, body). None if there is no frontmatter.
pub fn split_frontmatter(text: &str) -> Option<(&str, &str)> {
    let rest = text
        .strip_prefix("---\n")
        .or_else(|| text.strip_prefix("---\r\n"))?;
    let end = rest.find("\n---")?;
    let yaml = &rest[..end];
    let after = &rest[end + 4..];
    let body = after.split_once('\n').map(|(_, b)| b).unwrap_or("");
    Some((yaml, body))
}

fn valid_name(name: &str) -> bool {
    !name.is_empty()
        && name.len() <= 64
        && name
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
        && !name.starts_with('-')
        && !name.ends_with('-')
}

/// Finds 6-digit hex colors like #1b1b1f.
fn hex_colors(line: &str) -> Vec<String> {
    let bytes = line.as_bytes();
    let mut out = Vec::new();
    for (i, b) in bytes.iter().enumerate() {
        if *b != b'#' {
            continue;
        }
        if i > 0 && bytes[i - 1].is_ascii_alphanumeric() {
            continue;
        }
        let digits = bytes[i + 1..]
            .iter()
            .take_while(|c| c.is_ascii_hexdigit())
            .count();
        let next = bytes.get(i + 1 + digits);
        if digits == 6 && !next.is_some_and(|c| c.is_ascii_alphanumeric()) {
            out.push(line[i..i + 7].to_string());
        }
    }
    out
}

/// Checks one skill folder (skills/NAME/).
pub fn check_skill(dir: &Path, upstream: Option<&Upstream>) -> Vec<Finding> {
    let folder = dir
        .file_name()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_default();
    let mut out = Vec::new();
    let mut push = |level, message: String| {
        out.push(Finding {
            level,
            skill: folder.clone(),
            message,
        })
    };

    let path = dir.join("SKILL.md");
    let Ok(text) = fs::read_to_string(&path) else {
        push(Level::Error, "no SKILL.md in this folder".into());
        return out;
    };
    let Some((yaml, body)) = split_frontmatter(&text) else {
        push(
            Level::Error,
            "SKILL.md has no frontmatter (it must start with a --- block)".into(),
        );
        return out;
    };
    let front: Front = match serde_yaml::from_str(yaml) {
        Ok(f) => f,
        Err(e) => {
            push(Level::Error, format!("frontmatter is not valid YAML: {e}"));
            return out;
        }
    };

    // 1. Format
    match front.name.as_deref() {
        None => push(Level::Error, "frontmatter is missing `name`".into()),
        Some(n) if !valid_name(n) => push(
            Level::Error,
            format!("name `{n}` must be lowercase letters, digits and hyphens, at most 64"),
        ),
        Some(n) if n != folder => push(
            Level::Error,
            format!("name `{n}` must match its folder name `{folder}`"),
        ),
        _ => {}
    }
    match front.description.as_deref().map(str::trim) {
        None | Some("") => push(Level::Error, "frontmatter is missing `description`".into()),
        Some(d) if d.chars().count() > 1024 => push(
            Level::Error,
            format!(
                "description is {} characters; the limit is 1024",
                d.chars().count()
            ),
        ),
        _ => {}
    }

    // 2. Drift
    match meta_str(&front, TESTED_AGAINST) {
        None => push(
            Level::Error,
            format!("metadata is missing `{TESTED_AGAINST}` (the raea version you tested with)"),
        ),
        Some(v) => {
            if let Some(up) = upstream {
                if v != up.version {
                    push(
                        Level::Warn,
                        format!(
                            "stale: tested against raea {v}, the snapshot is {}; re-test, then bump `{TESTED_AGAINST}`",
                            up.version
                        ),
                    );
                }
            }
        }
    }
    if let (Some(verbs), Some(up)) = (meta_str(&front, VERBS), upstream) {
        for verb in verbs.split_whitespace() {
            if !up.verbs.contains(verb) {
                push(
                    Level::Error,
                    format!(
                        "uses `raea {verb}`, which raea {} no longer lists in its help",
                        up.version
                    ),
                );
            }
        }
    }

    // 3. Opinion creep (warnings; a human decides)
    for (n, line) in body.lines().enumerate() {
        if line.contains(OPINION_OK) {
            continue;
        }
        let lower = line.to_lowercase();
        for phrase in OPINION_PHRASES {
            if lower.contains(phrase) {
                push(
                    Level::Warn,
                    format!(
                        "opinion? line {}: \"{}\" prescribes taste; describe the choice instead, or mark the line `{OPINION_OK}`",
                        n + 1,
                        phrase.trim()
                    ),
                );
            }
        }
        for hex in hex_colors(line) {
            push(
                Level::Warn,
                format!(
                    "opinion? line {}: fixed color {hex}; colors belong to the person's taste or brand, not the skill",
                    n + 1
                ),
            );
        }
    }

    out
}

/// Checks every skill under `skills_dir` (folders starting with `_` or `.` are skipped, and so are the plugin's own
/// folders when `skills_dir` is a Claude plugin), then the plugin's manifest.
pub fn check_repo(skills_dir: &Path, upstream_dir: &Path) -> Result<Report> {
    let upstream = Upstream::load(upstream_dir)?;
    let mut report = Report::default();
    if !skills_dir.exists() {
        return Ok(report);
    }
    let plugin = skills_dir.join(PLUGIN_MANIFEST).is_file();
    let mut dirs: Vec<_> = fs::read_dir(skills_dir)
        .with_context(|| format!("reading {}", skills_dir.display()))?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.is_dir())
        .filter(|p| {
            let n = p.file_name().unwrap_or_default().to_string_lossy();
            !n.starts_with('_')
                && !n.starts_with('.')
                && !(plugin && PLUGIN_DIRS.contains(&n.as_ref()))
        })
        .collect();
    dirs.sort();
    let mut names = Vec::new();
    for dir in dirs {
        report.skills_checked += 1;
        report.findings.extend(check_skill(&dir, upstream.as_ref()));
        names.push(
            dir.file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string(),
        );
    }
    if plugin {
        report.findings.extend(check_plugin(skills_dir, &names));
    }
    Ok(report)
}

/// The manifest that makes the skills folder a Claude plugin, relative to it.
pub const PLUGIN_MANIFEST: &str = ".claude-plugin/plugin.json";
/// The plugin's own folders beside the skills, which aren't skills.
pub const PLUGIN_DIRS: &[&str] = &["hooks"];

/// Checks the plugin manifest: it parses, has a name, and its `skills` lists every skill folder and nothing else, so
/// no skill is left out of the plugin and no entry points at a skill that's gone.
pub fn check_plugin(skills_dir: &Path, skills: &[String]) -> Vec<Finding> {
    let mut out = Vec::new();
    let mut fail = |message: String| {
        out.push(Finding {
            level: Level::Error,
            skill: "plugin".into(),
            message,
        })
    };
    let path = skills_dir.join(PLUGIN_MANIFEST);
    let manifest: serde_yaml::Value = match fs::read_to_string(&path)
        .map_err(|e| e.to_string())
        .and_then(|t| serde_yaml::from_str(&t).map_err(|e| e.to_string()))
    {
        Ok(v) => v,
        Err(e) => {
            fail(format!("{PLUGIN_MANIFEST} doesn't parse: {e}"));
            return out;
        }
    };
    if manifest
        .get("name")
        .and_then(|n| n.as_str())
        .is_none_or(str::is_empty)
    {
        fail(format!("{PLUGIN_MANIFEST} has no `name`"));
    }
    let listed: Vec<&str> = match manifest.get("skills") {
        Some(serde_yaml::Value::String(s)) => vec![s.as_str()],
        Some(serde_yaml::Value::Sequence(v)) => v.iter().filter_map(|s| s.as_str()).collect(),
        _ => Vec::new(),
    };
    let folder = |entry: &str| {
        entry
            .trim_start_matches("./")
            .trim_end_matches('/')
            .to_string()
    };
    for name in skills {
        if !listed.iter().any(|e| folder(e) == *name) {
            fail(format!(
                "skills/{name} isn't in the plugin's skills ({PLUGIN_MANIFEST}): add \"./{name}\", or the plugin ships without it"
            ));
        }
    }
    for entry in listed {
        if !skills_dir.join(folder(entry)).join("SKILL.md").is_file() {
            fail(format!(
                "the plugin's skills list {entry:?}, which holds no SKILL.md"
            ));
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_verbs_from_help() {
        let help = "usage: raea render EDIT.tsx\n       raea gen image|video \"P\"\n | raea skills install";
        let v = verbs_in_help(help);
        assert!(v.contains("render") && v.contains("gen") && v.contains("skills"));
    }

    #[test]
    fn finds_six_digit_hex_only() {
        assert_eq!(hex_colors("bg #1b1b1f here"), vec!["#1b1b1f"]);
        assert!(hex_colors("see issue #123 and #12345678").is_empty());
    }
}
