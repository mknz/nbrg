use regex::Regex;

#[test]
fn test_search() {
    let filename = "tests/stub.ipynb";
    let pattern = "os";
    let re = Regex::new(&pattern).unwrap();
    nbrg::search(&filename, &re, &pattern);
}
