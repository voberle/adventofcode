use std::{fs, process::Command};

pub fn exec_c_code(code: &str) -> String {
    // Write the C file.
    fs::write("main.c", code).expect("Unable to write file");
    // Compile it.
    let _ = Command::new("gcc").arg("-O3").arg("main.c").output();
    // Run it.
    let output = Command::new("./a.out").output().unwrap();
    // Clean the files.
    let _ = Command::new("rm").arg("a.out").output();
    let _ = Command::new("rm").arg("main.c").output();

    String::from_utf8(output.stdout).unwrap().trim().to_string()
}
