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
            filename: "file.txt"
        }
    );
}

#[test]
fn query_one_result() {
    let query = "duct";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.";

    assert_eq!(vec!["safe, fast, productive."], search(query, contents));
}
