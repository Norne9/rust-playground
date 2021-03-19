use mygrep::*;

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