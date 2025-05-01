mod runtime_test;

fn main() {
    println!("===== RobloxRS Runtime Tests =====\n");
    
    // Run all tests and get the overall result
    let all_passed = runtime_test::tests::run_all_tests();
    
    // Exit with proper status code
    std::process::exit(if all_passed { 0 } else { 1 });
}
