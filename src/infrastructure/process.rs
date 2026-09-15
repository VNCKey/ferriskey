use std::path::Path;
use std::process::{Command, Output};

use crate::platform;

pub const MAX_CAPTURED_OUTPUT: usize = 512 * 1024;

pub fn run_shell_command(command: &str, cwd: &Path) -> std::io::Result<Output> {
    platform::shell_command(command).current_dir(cwd).output()
}

pub fn run_cargo(args: &[&str], cwd: &Path) -> std::io::Result<Output> {
    Command::new("cargo").args(args).current_dir(cwd).output()
}

pub fn captured_text(bytes: &[u8]) -> String {
    let text = String::from_utf8_lossy(bytes);
    if text.len() <= MAX_CAPTURED_OUTPUT {
        return text.into_owned();
    }

    let mut end = MAX_CAPTURED_OUTPUT;
    while end > 0 && !text.is_char_boundary(end) {
        end -= 1;
    }
    let mut truncated = text[..end].to_owned();
    truncated.push_str("\n\n[Salida truncada por FerrisKey: superó el límite de 512 KiB.]");
    truncated
}

#[cfg(test)]
mod tests {
    use super::{MAX_CAPTURED_OUTPUT, captured_text};

    #[test]
    fn truncates_unicode_without_breaking_utf8() {
        let input = "🙂".repeat(MAX_CAPTURED_OUTPUT);
        let output = captured_text(input.as_bytes());

        assert!(output.ends_with("[Salida truncada por FerrisKey: superó el límite de 512 KiB.]"));
        assert!(std::str::from_utf8(output.as_bytes()).is_ok());
    }
}
