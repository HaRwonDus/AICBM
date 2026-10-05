use std::process::Command;
fn cli(args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_aicbm"))
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .args(args)
        .output()
        .unwrap()
}
#[test]
fn validates_example_with_json_report() {
    let result = cli(&["validate", "examples/plan.json", "--json"]);
    assert!(result.status.success());
    let report: serde_json::Value = serde_json::from_slice(&result.stdout).unwrap();
    assert_eq!(report["status"], "valid");
    assert_eq!(report["scope"], "structural");
    assert_eq!(report["errors"], serde_json::json!([]));
}
#[test]
fn reports_input_errors_as_json() {
    let result = cli(&["validate", "examples/does-not-exist.json", "--json"]);
    assert_eq!(result.status.code(), Some(2));
    let report: serde_json::Value = serde_json::from_slice(&result.stdout).unwrap();
    assert_eq!(report["status"], "input_error");
}
#[test]
fn rejects_invalid_plan() {
    let result = cli(&["validate", "examples/invalid-plan.json", "--json"]);
    assert_eq!(result.status.code(), Some(1));
    let report: serde_json::Value = serde_json::from_slice(&result.stdout).unwrap();
    assert_eq!(report["status"], "invalid");
    assert!(!report["errors"].as_array().unwrap().is_empty());
}
#[test]
fn handles_help_and_bad_arguments() {
    assert!(cli(&["--help"]).status.success());
    for args in [
        vec![],
        vec!["validate"],
        vec!["build", "examples/plan.json"],
        vec!["validate", "examples/plan.json", "--unknown"],
    ] {
        assert_eq!(cli(&args).status.code(), Some(2));
    }
}
