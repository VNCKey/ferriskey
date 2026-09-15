use std::path::{Path, PathBuf};
use std::process::Output;

use crate::infrastructure::process;

pub fn run_shell_command(command: &str, cwd: &Path) -> std::io::Result<Output> {
    process::run_shell_command(command, cwd)
}

pub fn detect_cargo_new(command: &str, cwd: &Path) -> Option<(PathBuf, String, bool)> {
    let parts: Vec<&str> = command.split_whitespace().collect();
    if parts.first() != Some(&"cargo") {
        return None;
    }

    let subcommand_index = if parts.get(1).is_some_and(|part| part.starts_with('+')) {
        2
    } else {
        1
    };
    if parts.get(subcommand_index) != Some(&"new") {
        return None;
    }

    let mut is_lib = false;
    let mut project_arg = None;
    let mut index = subcommand_index + 1;

    while index < parts.len() {
        let part = parts[index];
        if part == "--lib" || part == "-l" {
            is_lib = true;
        } else if part == "--name" || part == "--vcs" || part == "--edition" {
            index += 1;
        } else if !part.starts_with('-') && project_arg.is_none() {
            project_arg = Some(part);
        }
        index += 1;
    }

    let project_arg = project_arg?;
    let raw_path = PathBuf::from(project_arg);
    let project_dir = if raw_path.is_absolute() {
        raw_path
    } else {
        cwd.join(raw_path)
    };
    let project_name = project_dir.file_name()?.to_str()?.to_owned();

    Some((project_dir, project_name, is_lib))
}

pub fn command_updates_manifest(command: &str) -> bool {
    matches!(
        command.split_whitespace().collect::<Vec<_>>().as_slice(),
        ["cargo", "add", ..]
            | ["cargo", "remove", ..]
            | ["cargo", "update", ..]
            | ["cargo", "generate-lockfile", ..]
    )
}

pub fn is_cargo_run(command: &str) -> bool {
    let parts: Vec<&str> = command.split_whitespace().collect();
    if parts.first() != Some(&"cargo") {
        return false;
    }

    let subcommand_index = if parts.get(1).is_some_and(|part| part.starts_with('+')) {
        2
    } else {
        1
    };

    parts.get(subcommand_index) == Some(&"run")
}

#[cfg(test)]
mod tests {
    use super::{command_updates_manifest, detect_cargo_new, is_cargo_run};

    #[test]
    fn detects_binary_and_library_projects() {
        let cwd = std::path::Path::new("/tmp/Projects");
        let binary = detect_cargo_new("cargo new demo", cwd).expect("binary project");
        assert_eq!(binary.1, "demo");
        assert!(!binary.2);

        let library =
            detect_cargo_new("cargo +stable new demo-lib --lib", cwd).expect("library project");
        assert_eq!(library.1, "demo-lib");
        assert!(library.2);
    }

    #[test]
    fn classifies_terminal_commands() {
        assert!(command_updates_manifest("cargo add rand"));
        assert!(is_cargo_run("cargo +stable run"));
        assert!(!is_cargo_run("cargo build"));
    }
}
