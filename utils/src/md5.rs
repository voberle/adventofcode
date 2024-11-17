/// Computes the MD5 hash of a string and converts it back to a string
fn hash(some_str: &str, another_str: &str) -> String {
    let digest = md5::compute(format!("{some_str}{another_str}").as_bytes());
    format!("{digest:x}")
}
