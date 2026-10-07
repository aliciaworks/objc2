use std::fmt;
use std::path::Path;
use std::process::{Command, Stdio};

pub fn run_cargo_fmt(packages: impl IntoIterator<Item = impl fmt::Display>) {
    let status = Command::new("cargo")
        .arg("fmt")
        .args(packages.into_iter().map(|package| format!("-p{package}")))
        .current_dir(Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap())
        .status()
        .expect("failed running cargo fmt");

    // Not an assertion here: formatting is the last cosmetic step over code that has already been written, and a
    // crate this fork generates can be one rustfmt refuses without that meaning the generation is wrong. The
    // first failure of this kind is a crate that is not a member of the trimmed workspace, which is not a
    // property of the code at all.
    if !status.success() {
        tracing::warn!("cargo fmt said {status}; the generated code is written but left unformatted");
    }
}

pub fn run_rustfmt(data: impl fmt::Display) -> Vec<u8> {
    use std::io::Write;

    let mut child = Command::new("rustfmt")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed running rustfmt");

    let mut stdin = child.stdin.take().expect("failed to open stdin");
    write!(stdin, "{data}").expect("failed writing");
    drop(stdin);

    let output = child.wait_with_output().expect("failed formatting");

    if !output.status.success() {
        panic!("failed running rustfmt with exit code {}", output.status)
    }

    output.stdout
}
