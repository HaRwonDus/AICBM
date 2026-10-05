use std::{env, fs, process::ExitCode};

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();
    if args == ["--help"] || args == ["-h"] {
        println!("Usage: aicbm validate <plan.json> [--json]");
        return ExitCode::SUCCESS;
    }
    let json = args.len() == 3 && args[2] == "--json";
    if !(args.len() == 2 || json) || args[0] != "validate" {
        eprintln!("Usage: aicbm validate <plan.json> [--json]");
        return ExitCode::from(2);
    }
    let result = fs::read_to_string(&args[1])
        .map_err(|e| format!("{}: {e}", args[1]))
        .and_then(|s| serde_json::from_str::<aicbm::Plan>(&s).map_err(|e| e.to_string()));
    match result {
        Err(error) => {
            if json {
                println!(
                    "{}",
                    serde_json::json!({"scope": "structural", "status": "input_error", "errors": [error]})
                );
            } else {
                eprintln!("{error}");
            }
            ExitCode::from(2)
        }
        Ok(plan) => {
            let errors = aicbm::validate(&plan);
            if json {
                println!(
                    "{}",
                    serde_json::json!({"scope": "structural", "status": if errors.is_empty() {"valid"} else {"invalid"}, "errors": errors})
                );
            } else if errors.is_empty() {
                println!("Structural validation passed; full buildability is not checked.");
            } else {
                for error in &errors {
                    eprintln!("{error}");
                }
            }
            if errors.is_empty() {
                ExitCode::SUCCESS
            } else {
                ExitCode::from(1)
            }
        }
    }
}
