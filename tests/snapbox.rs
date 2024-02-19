use std::process::{Command, Stdio};

#[test]
fn snapbox() {
    let status = Command::new("cargo")
        .arg("clean")
        .current_dir("fixture")
        .status()
        .unwrap();
    assert!(status.success());

    let output = Command::new("cargo")
        .args([
            "test",
            "--config",
            &format!(
                "target.'cfg(all())'.runner = '{}'",
                env!("CARGO_BIN_EXE_group-runner"),
            ),
            "--",
            "--test-threads=1",
        ])
        .env("CARGO_TERM_COLOR", "never")
        .env_remove("CI")
        .current_dir("fixture")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .unwrap();
    assert!(output.status.success());
    snapbox::assert_matches(snapbox::file!("expected.stdout"), output.stdout);
    snapbox::assert_matches(snapbox::file!("expected.stderr"), output.stderr);
}
