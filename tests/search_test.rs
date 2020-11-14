use regex::Regex;

#[test]
fn test_search() {
    let filename = "tests/stub.ipynb";
    let pattern = "foo";
    let re = Regex::new(&pattern).unwrap();
    assert!(nbrg::search(&filename, &re, &pattern));
}

#[test]
fn test_search_empty_notebook() {
    let filename = "tests/stub_empty.ipynb";
    let pattern = "foo";
    let re = Regex::new(&pattern).unwrap();
    assert!(!nbrg::search(&filename, &re, &pattern));
}
