use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use std::process;

use clap::{Parser, Subcommand};
use roblox_rs_core::{compile, CompileOptions, compiler::OptimizationLevel};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Compile a Rust file to Luau
    Compile {
        /// Input Rust file
        #[arg(value_name = "INPUT")]
        input: String,

        /// Output Luau file [default: input filename with .lua extension]
        #[arg(short, long, value_name = "OUTPUT")]
        output: Option<String>,

        /// Optimization level (minimal, default, aggressive)
        #[arg(short, long, default_value = "default")]
        optimization: String,

        /// Include debug information
        #[arg(short, long)]
        debug: bool,
    },

    /// Create a new roblox-rs project
    New {
        /// Project name
        #[arg(value_name = "NAME")]
        name: String,

        /// Project directory [default: NAME]
        #[arg(short, long, value_name = "DIRECTORY")]
        directory: Option<String>,

        /// Use ECS framework
        #[arg(short, long)]
        ecs: bool,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Compile { input, output, optimization, debug } => {
            if !Path::new(input).exists() {
                eprintln!("Error: Input file '{}' does not exist", input);
                process::exit(1);
            }

            // Read input file
            let mut content = String::new();
            File::open(input)
                .and_then(|mut file| file.read_to_string(&mut content))
                .unwrap_or_else(|err| {
                    eprintln!("Error reading input file: {}", err);
                    process::exit(1);
                });

            // Parse optimization level
            let opt_level = match optimization.to_lowercase().as_str() {
                "minimal" => OptimizationLevel::Minimal,
                "default" => OptimizationLevel::Default,
                "aggressive" => OptimizationLevel::Aggressive,
                _ => {
                    eprintln!("Invalid optimization level: {}", optimization);
                    eprintln!("Using default optimization level");
                    OptimizationLevel::Default
                }
            };

            // Setup compile options
            let options = CompileOptions {
                include_runtime: true,
                debug_mode: *debug,
                enable_parallelization: false,
                optimization_level: opt_level,
                target_dir: None,
                flags: Vec::new(),
            };

            // Compile the code
            match compile(&content, options) {
                Ok(luau_code) => {
                    // Determine output filename
                    let output_path = match output {
                        Some(path) => PathBuf::from(path),
                        None => {
                            let input_path = PathBuf::from(input);
                            let mut output_path = input_path.with_extension("lua");
                            if output_path == input_path {
                                output_path = PathBuf::from(format!("{}.lua", input));
                            }
                            output_path
                        }
                    };

                    // Write the output
                    fs::write(&output_path, luau_code).unwrap_or_else(|err| {
                        eprintln!("Error writing output file: {}", err);
                        process::exit(1);
                    });

                    println!("Successfully compiled to: {}", output_path.display());
                }
                Err(err) => {
                    eprintln!("Compilation failed: {}", err);
                    process::exit(1);
                }
            }
        }

        Commands::New { name, directory, ecs } => {
            let dir_path = PathBuf::from(directory.clone().unwrap_or_else(|| name.clone()));

            // Create project directory
            if dir_path.exists() {
                eprintln!("Error: Directory '{}' already exists", dir_path.display());
                process::exit(1);
            }

            fs::create_dir_all(&dir_path).unwrap_or_else(|err| {
                eprintln!("Error creating project directory: {}", err);
                process::exit(1);
            });

            // Create Cargo.toml
            let mut cargo_toml = format!(
                r#"[package]
name = "{}"
version = "0.1.0"
edition = "2021"

[dependencies]
roblox-rs-core = "0.1.0"
"#,
                name
            );

            if *ecs {
                cargo_toml.push_str("roblox-rs-ecs = \"0.1.0\"\n");
            }

            cargo_toml.push_str(
                r#"
[features]
default = ["native"]
native = []
roblox = []
"#,
            );

            fs::write(dir_path.join("Cargo.toml"), cargo_toml).unwrap_or_else(|err| {
                eprintln!("Error writing Cargo.toml: {}", err);
                process::exit(1);
            });

            // Create src directory
            let src_dir = dir_path.join("src");
            fs::create_dir_all(&src_dir).unwrap_or_else(|err| {
                eprintln!("Error creating src directory: {}", err);
                process::exit(1);
            });

            // Create main.rs
            let main_rs = if *ecs {
                r#"use roblox_rs_ecs::prelude::*;

fn main() {
    let mut app = App::new();
    
    app.add_system(hello_world);
    
    app.run();
}

fn hello_world() {
    println!("Hello, world!");
}
"#
            } else {
                r#"fn main() {
    println!("Hello, world!");
}
"#
            };

            fs::write(src_dir.join("main.rs"), main_rs).unwrap_or_else(|err| {
                eprintln!("Error writing main.rs: {}", err);
                process::exit(1);
            });

            // Create README.md
            let readme_md = format!(
                r#"# {}

A Roblox game written in Rust, compiled to Luau.

## Building

```bash
# Compile for testing on native platform
cargo build

# Compile for Roblox
roblox-rs build --target roblox
```
"#,
                name
            );

            fs::write(dir_path.join("README.md"), readme_md).unwrap_or_else(|err| {
                eprintln!("Error writing README.md: {}", err);
                process::exit(1);
            });

            println!("Successfully created project: {}", name);
            println!("Project directory: {}", dir_path.display());
        }
    }
}
