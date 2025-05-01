// CLI for the dependency bundler

use std::path::{Path, PathBuf};
use std::fs;
use structopt::StructOpt;
use chrono::Local;

use super::{DependencyManager, WebBundlerConfig};

/// CLI arguments for the dependency bundler
#[derive(Debug, StructOpt)]
#[structopt(name = "roblox-rs-deps", about = "Dependency bundler for Roblox-RS")]
pub struct BundlerArgs {
    /// Path to the project directory
    #[structopt(short, long, parse(from_os_str))]
    pub project_dir: PathBuf,

    /// Output directory
    #[structopt(short, long, parse(from_os_str))]
    pub output_dir: Option<PathBuf>,

    /// Output format (rbxm, lua)
    #[structopt(short, long, default_value = "lua")]
    pub format: String,

    /// Project name
    #[structopt(short, long)]
    pub name: Option<String>,

    /// Whether to minify output
    #[structopt(short, long)]
    pub minify: bool,

    /// Whether to include documentation
    #[structopt(short, long)]
    pub docs: bool,

    /// Whether to verbose output
    #[structopt(short, long)]
    pub verbose: bool,
}

/// Dependency bundler CLI
pub struct BundlerCli {
    args: BundlerArgs,
    manager: DependencyManager,
}

impl BundlerCli {
    /// Create a new CLI instance
    pub fn new(args: BundlerArgs) -> Result<Self, String> {
        let output_dir = args.output_dir.clone().unwrap_or_else(|| {
            args.project_dir.join("roblox-rs-output")
        });

        // Create output directory if it doesn't exist
        fs::create_dir_all(&output_dir)
            .map_err(|e| format!("Failed to create output directory: {}", e))?;

        let manager = DependencyManager::new(&args.project_dir, &output_dir);

        Ok(Self {
            args,
            manager,
        })
    }

    /// Run the CLI tool
    pub fn run(&mut self) -> Result<(), String> {
        if self.args.verbose {
            println!("Roblox-RS Dependency Bundler");
            println!("============================");
            println!("Project directory: {:?}", self.args.project_dir);
            println!("Output directory: {:?}", self.get_output_dir());
            println!("Output format: {}", self.args.format);
            println!("Minify: {}", self.args.minify);
            println!("Include docs: {}", self.args.docs);
            println!();
        }

        // Scan dependencies
        let start_time = std::time::Instant::now();
        if self.args.verbose {
            println!("Scanning dependencies...");
        }
        let configs = self.manager.scan_dependencies()?;
        if self.args.verbose {
            println!("Found {} dependencies:", configs.len());
            for config in &configs {
                println!("  - {} v{}", config.name, config.version);
            }
            println!();
        }

        // Compile dependencies
        if self.args.verbose {
            println!("Compiling dependencies...");
        }
        let packages = self.manager.compile_all_dependencies()?;
        if self.args.verbose {
            println!("Compiled {} packages", packages.len());
            for package in &packages {
                println!("  - {} v{} ({} bytes)", 
                    package.name, 
                    package.version, 
                    package.code.len());
            }
            println!();
        }

        // Generate output
        if self.args.verbose {
            println!("Generating output...");
        }

        match self.args.format.as_str() {
            "rbxm" => {
                let model_path = self.manager.generate_roblox_model()?;
                if self.args.verbose {
                    println!("Generated Roblox model: {:?}", model_path);
                }
            },
            "lua" => {
                self.generate_lua_output(&packages)?;
                if self.args.verbose {
                    println!("Generated Lua modules in {:?}", self.get_output_dir());
                }
            },
            _ => return Err(format!("Unsupported output format: {}", self.args.format)),
        }

        let elapsed = start_time.elapsed();
        if self.args.verbose {
            println!();
            println!("Done in {:.2}s", elapsed.as_secs_f32());
        }

        Ok(())
    }

    /// Generate Lua output
    fn generate_lua_output(&self, packages: &[super::CompiledPackage]) -> Result<(), String> {
        let output_dir = self.get_output_dir();
        let modules_dir = output_dir.join("modules");
        fs::create_dir_all(&modules_dir)
            .map_err(|e| format!("Failed to create modules directory: {}", e))?;

        // Create index file
        let index_content = self.generate_index_file(packages)?;
        let index_path = output_dir.join("init.lua");
        fs::write(&index_path, index_content)
            .map_err(|e| format!("Failed to write index file: {}", e))?;

        // Create modules
        for package in packages {
            let module_dir = modules_dir.join(&package.name);
            fs::create_dir_all(&module_dir)
                .map_err(|e| format!("Failed to create module directory: {}", e))?;

            let content = if self.args.minify {
                self.minify_lua(&package.code)
            } else {
                package.code.clone()
            };

            let module_path = module_dir.join("init.lua");
            fs::write(&module_path, content)
                .map_err(|e| format!("Failed to write module: {}", e))?;
        }

        // Create types file if requested
        if self.args.docs {
            let types_content = self.generate_types_file(packages)?;
            let types_path = output_dir.join("types.d.lua");
            fs::write(&types_path, types_content)
                .map_err(|e| format!("Failed to write types file: {}", e))?;
        }

        Ok(())
    }

    /// Generate index file
    fn generate_index_file(&self, packages: &[super::CompiledPackage]) -> Result<String, String> {
        let project_name = self.args.name.clone().unwrap_or_else(|| {
            self.args.project_dir
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or("roblox-rs-project")
                .to_string()
        });

        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let mut content = String::new();

        content.push_str(&format!("-- Generated by Roblox-RS Dependency Bundler\n"));
        content.push_str(&format!("-- Project: {}\n", project_name));
        content.push_str(&format!("-- Generated on: {}\n\n", timestamp));

        content.push_str("local Packages = {}\n\n");

        for package in packages {
            content.push_str(&format!("-- Package: {} v{}\n", package.name, package.version));
            content.push_str(&format!("Packages.{} = require(script.modules.{})\n\n", 
                package.name, package.name));
        }

        content.push_str("return Packages\n");

        Ok(content)
    }

    /// Generate types file
    fn generate_types_file(&self, packages: &[super::CompiledPackage]) -> Result<String, String> {
        let project_name = self.args.name.clone().unwrap_or_else(|| {
            self.args.project_dir
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or("roblox-rs-project")
                .to_string()
        });

        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let mut content = String::new();

        content.push_str(&format!("-- Type definitions for Roblox-RS dependencies\n"));
        content.push_str(&format!("-- Project: {}\n", project_name));
        content.push_str(&format!("-- Generated on: {}\n\n", timestamp));

        content.push_str("export type Packages = {\n");

        for package in packages {
            content.push_str(&format!("    {}: any, -- v{}\n", package.name, package.version));
        }

        content.push_str("}\n\n");
        content.push_str("return nil\n");

        Ok(content)
    }

    /// Minify Lua code
    fn minify_lua(&self, code: &str) -> String {
        // Simple minification: remove comments and extra whitespace
        let mut minified = String::new();
        
        for line in code.lines() {
            let trimmed = match line.find("--") {
                Some(idx) => &line[..idx],
                None => line,
            }.trim();
            
            if !trimmed.is_empty() {
                minified.push_str(trimmed);
                minified.push(' ');
            }
        }
        
        minified
    }

    /// Get output directory
    fn get_output_dir(&self) -> PathBuf {
        self.args.output_dir.clone().unwrap_or_else(|| {
            self.args.project_dir.join("roblox-rs-output")
        })
    }
}

/// Run the CLI
pub fn run() -> Result<(), String> {
    let args = BundlerArgs::from_args();
    let mut cli = BundlerCli::new(args)?;
    cli.run()
}
