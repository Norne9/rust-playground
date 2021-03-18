use mygrep::*;

#[test]
fn config_test() {
    let args = vec![
        "app.exe".to_owned(),
        "expression".to_owned(),
        "file.txt".to_owned(),
    ];
    let config = Config::new(&args).unwrap();
    assert_eq!(
        config,
        Config {
            query: "expression",
            filename: "file.txt",
            case_sensitive: false,
        }
    );
}

#[test]
fn query_casesensitive() {
    let query = "duct";
    let contents = "\
Rust:
safe, fast, productive.
rusty.
Duct tape.";

    assert_eq!(vec!["safe, fast, productive."], search(query, contents));
}

#[test]
fn query_caseinsensitive() {
    let query = "RuSt";
    let contents = "\
Rust:
safe, fast, productive.
rusty.
Duct tape.";

    assert_eq!(vec!["Rust:", "rusty."], search_ignore_case(query, contents));
}