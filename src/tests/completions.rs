use crate::cli::parse_cli;

#[test]
fn parse_cli_accepts_auto_complete() {
    let args = vec!["leaf".to_string(), "--auto-complete".to_string()];
    let options = parse_cli(&args).unwrap();
    assert!(options.auto_complete);
}

#[test]
fn auto_complete_rejects_with_file() {
    let args = vec![
        "leaf".to_string(),
        "--auto-complete".to_string(),
        "README.md".to_string(),
    ];
    assert!(parse_cli(&args).is_err());
}

#[test]
fn auto_complete_rejects_with_watch() {
    let args = vec![
        "leaf".to_string(),
        "--auto-complete".to_string(),
        "--watch".to_string(),
    ];
    assert!(parse_cli(&args).is_err());
}

#[test]
fn auto_complete_rejects_with_update() {
    let args = vec![
        "leaf".to_string(),
        "--auto-complete".to_string(),
        "--update".to_string(),
    ];
    assert!(parse_cli(&args).is_err());
}

#[test]
fn auto_complete_rejects_with_config() {
    let args = vec![
        "leaf".to_string(),
        "--auto-complete".to_string(),
        "--config".to_string(),
    ];
    assert!(parse_cli(&args).is_err());
}

#[test]
fn auto_complete_rejects_with_theme() {
    let args = vec![
        "leaf".to_string(),
        "--auto-complete".to_string(),
        "--theme".to_string(),
        "arctic".to_string(),
    ];
    assert!(parse_cli(&args).is_err());
}
