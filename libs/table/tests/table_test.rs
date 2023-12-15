use std::{fs::File, io::BufReader};

use table::{build_tables, Table};

#[test]
fn test_data() {
    let file = File::open("tests/files/input_test").unwrap();
    let mut reader = BufReader::new(file);
    let tables: Vec<Table> = build_tables(&mut reader);
    assert!(!tables.first().unwrap().to_string().is_empty());
}
