use roblox_rs_core::transform::RustToLuauTransformer;
use roblox_rs_core::ast::{LuauNode, Program};
use roblox_rs_core::codegen::LuauCodeGenerator;
use roblox_rs_core::tests::TestHelper;
use syn::parse_str;

// Test helper to go from Rust code to Luau code
fn transform_and_generate(rust_code: &str) -> String {
    // Parse Rust code
    let syntax = parse_str(rust_code).expect("Failed to parse Rust code");
    
    // Transform to Luau AST
    let mut transformer = RustToLuauTransformer::new();
    let program = transformer.transform_program(&syntax)
        .expect("Failed to transform Rust to Luau");
    
    // Generate Luau code
    let mut generator = LuauCodeGenerator::new();
    generator.generate(&LuauNode::Program(program.clone()))
        .expect("Failed to generate Luau code")
}

#[test]
fn test_basic_function_transform() {
    let rust_code = r#"
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }
    "#;
    
    let luau_code = transform_and_generate(rust_code);
    
    // Verify function was properly transformed
    assert!(luau_code.contains("function add(a, b)"));
    assert!(luau_code.contains("return a + b"));
}

#[test]
fn test_struct_transform() {
    let rust_code = r#"
    struct Point {
        x: f32,
        y: f32,
    }
    
    impl Point {
        fn new(x: f32, y: f32) -> Self {
            Point { x, y }
        }
        
        fn distance(&self, other: &Point) -> f32 {
            let dx = self.x - other.x;
            let dy = self.y - other.y;
            (dx * dx + dy * dy).sqrt()
        }
    }
    "#;
    
    let luau_code = transform_and_generate(rust_code);
    
    // Verify struct and methods were properly transformed
    assert!(luau_code.contains("Point = {}"));
    assert!(luau_code.contains("Point.__index = Point"));
    assert!(luau_code.contains("function Point.new(x, y)"));
    assert!(luau_code.contains("function Point:distance(other)"));
}

#[test]
fn test_enum_transform() {
    let rust_code = r#"
    enum Result<T, E> {
        Ok(T),
        Err(E),
    }
    
    impl<T, E> Result<T, E> {
        fn is_ok(&self) -> bool {
            match self {
                Result::Ok(_) => true,
                Result::Err(_) => false,
            }
        }
        
        fn unwrap(self) -> T {
            match self {
                Result::Ok(val) => val,
                Result::Err(_) => panic!("Called unwrap on an Err value"),
            }
        }
    }
    "#;
    
    let luau_code = transform_and_generate(rust_code);
    
    // Verify enum and methods were properly transformed
    assert!(luau_code.contains("Result = {}"));
    assert!(luau_code.contains("Result.Ok = function(value)"));
    assert!(luau_code.contains("Result.Err = function(value)"));
    assert!(luau_code.contains("function Result:is_ok()"));
    assert!(luau_code.contains("function Result:unwrap()"));
}

#[test]
fn test_control_flow_transform() {
    let rust_code = r#"
    fn process_value(x: i32) -> String {
        let result = if x > 10 {
            "greater"
        } else if x < 0 {
            "negative"
        } else {
            "normal"
        };
        
        let mut counter = 0;
        while counter < x {
            counter += 1;
        }
        
        for i in 0..5 {
            // Do something
            println!("{}", i);
        }
        
        return format!("Result: {} (counted to {})", result, counter);
    }
    "#;
    
    let luau_code = transform_and_generate(rust_code);
    
    // Verify control flow constructs were properly transformed
    assert!(luau_code.contains("if x > 10 then"));
    assert!(luau_code.contains("elseif x < 0 then"));
    assert!(luau_code.contains("else"));
    assert!(luau_code.contains("while counter < x do"));
    assert!(luau_code.contains("for i = 0, 4 do"));
    assert!(luau_code.contains("return"));
}

#[test]
fn test_vector_transform() {
    let rust_code = r#"
    fn create_vector() -> Vec<i32> {
        let mut vec = Vec::new();
        vec.push(1);
        vec.push(2);
        vec.push(3);
        vec
    }
    
    fn process_vector(vec: &Vec<i32>) -> i32 {
        let mut sum = 0;
        for item in vec {
            sum += item;
        }
        sum
    }
    "#;
    
    let luau_code = transform_and_generate(rust_code);
    
    // Verify vector operations were properly transformed
    assert!(luau_code.contains("local vec = {}"));
    assert!(luau_code.contains("table.insert(vec, 1)"));
    assert!(luau_code.contains("for _, item in ipairs(vec) do"));
}

#[test]
fn test_pattern_matching_transform() {
    let rust_code = r#"
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    
    fn process_message(msg: Message) -> String {
        match msg {
            Message::Quit => "Quit".to_string(),
            Message::Move { x, y } => format!("Move to ({}, {})", x, y),
            Message::Write(text) => format!("Text message: {}", text),
            Message::ChangeColor(r, g, b) => format!("Change color to ({}, {}, {})", r, g, b),
        }
    }
    "#;
    
    let luau_code = transform_and_generate(rust_code);
    
    // Verify pattern matching was properly transformed
    assert!(luau_code.contains("if msg.tag == \"Quit\" then"));
    assert!(luau_code.contains("elseif msg.tag == \"Move\" then"));
    assert!(luau_code.contains("local x = msg.x"));
    assert!(luau_code.contains("elseif msg.tag == \"Write\" then"));
    assert!(luau_code.contains("local text = msg.value"));
    assert!(luau_code.contains("elseif msg.tag == \"ChangeColor\" then"));
    assert!(luau_code.contains("local r, g, b = msg.r, msg.g, msg.b"));
}

#[test]
fn test_error_handling_transform() {
    let rust_code = r#"
    fn parse_number(input: &str) -> Result<i32, String> {
        match input.parse::<i32>() {
            Ok(num) => Ok(num),
            Err(_) => Err(format!("Failed to parse: {}", input)),
        }
    }
    
    fn safe_divide(a: i32, b: i32) -> Result<i32, String> {
        if b == 0 {
            return Err("Division by zero".to_string());
        }
        Ok(a / b)
    }
    "#;
    
    let luau_code = transform_and_generate(rust_code);
    
    // Verify error handling was properly transformed
    assert!(luau_code.contains("Result.Ok"));
    assert!(luau_code.contains("Result.Err"));
    assert!(luau_code.contains("if b == 0 then"));
    assert!(luau_code.contains("return Result.Err(\"Division by zero\")"));
}

#[test]
fn test_delimiters_and_blocks() {
    let rust_code = r#"
    fn nested_blocks() {
        {
            let x = 10;
            {
                let y = 20;
                {
                    let z = 30;
                    println!("{} {} {}", x, y, z);
                }
            }
        }
    }
    "#;
    
    let luau_code = transform_and_generate(rust_code);
    
    // Verify delimiter handling for nested blocks
    assert!(luau_code.contains("function nested_blocks()"));
    assert!(luau_code.contains("do"));
    assert!(luau_code.contains("local x = 10"));
    assert!(luau_code.contains("local y = 20"));
    assert!(luau_code.contains("local z = 30"));
    assert!(luau_code.contains("end"));
}
