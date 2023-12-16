use std::io;

use table::build_tables;
use table::Table;

fn main() {
    let stdin = io::stdin();

    let patterns: Vec<Table<char>> = build_tables(&mut stdin.lock());
    for p in &patterns {
        println!("{}", &p);
    }
}
