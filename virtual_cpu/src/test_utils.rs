pub const RESOURCES_DIR: &str = "src/bin/input";

pub fn get_input_file(puzzle_name: &str) -> String {
    format!("{}/{}_input", RESOURCES_DIR, puzzle_name)
}
