use std::env;
use std::fs;
use std::path::Path;

fn discover_rust_tests(test_dir: &str) -> Vec<String> {
    // For now, just list all .rs files in the test_dir
    let mut tests = Vec::new();
    if let Ok(entries) = fs::read_dir(test_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("rs") {
                tests.push(path.display().to_string());
            }
        }
    }
    tests
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 && args[1] == "test" {
        println!("roblox-rs-tester: Discovering tests in ./tests ...");
        let test_files = discover_rust_tests("./tests");
        if test_files.is_empty() {
            println!("No test files found in ./tests");
            return;
        }
        println!("Found test files:");
        for file in &test_files {
            println!("  - {}", file);
        }
        // TODO: Compile each test file to Luau and run them
        println!("roblox-rs-tester: (Stub) Would now compile and run these tests.");
    } else {
        println!("roblox-rs-tester: Usage: roblox-rs-tester test");
    }
}
