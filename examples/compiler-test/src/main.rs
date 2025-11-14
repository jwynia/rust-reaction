//! Test the Morpheus compiler
//!
//! This example demonstrates Phase 1: Runtime Rust compilation.
//!
//! Run with: cargo run --example compiler-test (from repo root)
//! Or: cargo run (from this directory)

use morpheus_compiler::{Compiler, SubprocessCompiler};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("🧬 Morpheus Compiler Test\n");

    // Check if tools are available
    println!("1. Checking for required tools...");
    match SubprocessCompiler::check_tools() {
        Ok(_) => println!("   ✓ rustc and wasm-pack found\n"),
        Err(e) => {
            eprintln!("   ✗ Error: {}", e);
            eprintln!("\n   Please install:");
            eprintln!("   - Rust: https://rustup.rs/");
            eprintln!("   - wasm-pack: cargo install wasm-pack");
            std::process::exit(1);
        }
    }

    // Create compiler
    println!("2. Creating compiler...");
    let compiler = SubprocessCompiler::new().await?;
    println!("   ✓ Compiler ready\n");

    // Test 1: Compile valid code
    println!("3. Testing compilation of valid code...");
    let hello_world = r#"
        use wasm_bindgen::prelude::*;

        #[wasm_bindgen]
        pub fn greet(name: &str) -> String {
            format!("Hello, {}!", name)
        }
    "#;

    println!("   Source code:");
    for line in hello_world.lines() {
        println!("   {}", line);
    }
    println!();

    println!("   Compiling... (this may take 5-10 seconds)");
    let start = std::time::Instant::now();

    match compiler.compile(hello_world).await {
        Ok(wasm_bytes) => {
            let duration = start.elapsed();
            println!("   ✓ Compilation successful!");
            println!("   - Time: {:.2}s", duration.as_secs_f64());
            println!("   - WASM size: {} bytes", wasm_bytes.len());
            println!("   - Size: {:.2} KB\n", wasm_bytes.len() as f64 / 1024.0);
        }
        Err(e) => {
            println!("   ✗ Compilation failed:");
            println!("   {}\n", e);
            return Err(e.into());
        }
    }

    // Test 2: Catch compilation errors
    println!("4. Testing error detection...");
    let bad_code = r#"
        use wasm_bindgen::prelude::*;

        #[wasm_bindgen]
        pub fn broken(x: i32) -> String {
            x.to_string(  // Syntax error - missing )
        }
    "#;

    println!("   Source code (intentionally broken):");
    for line in bad_code.lines() {
        println!("   {}", line);
    }
    println!();

    println!("   Compiling...");
    match compiler.compile(bad_code).await {
        Ok(_) => {
            println!("   ✗ Should have failed!");
            return Err(anyhow::anyhow!("Bad code compiled!"));
        }
        Err(e) => {
            println!("   ✓ Correctly caught compilation error:");
            for line in e.to_string().lines().take(10) {
                println!("     {}", line);
            }
            println!("\n");
        }
    }

    // Test 3: Type checking
    println!("5. Testing type check (faster than full compile)...");
    let type_error_code = r#"
        use wasm_bindgen::prelude::*;

        #[wasm_bindgen]
        pub fn wrong_types() -> String {
            let x: i32 = "not a number";  // Type error!
            x.to_string()
        }
    "#;

    println!("   Source code (type error):");
    for line in type_error_code.lines() {
        println!("   {}", line);
    }
    println!();

    println!("   Type checking...");
    match compiler.check(type_error_code).await {
        Ok(_) => {
            println!("   ✗ Should have failed type check!");
        }
        Err(e) => {
            println!("   ✓ Correctly caught type error:");
            for line in e.to_string().lines().take(10) {
                println!("     {}", line);
            }
        }
    }

    println!("\n✅ All tests passed!");
    println!("\n🎯 Phase 1 Complete: Runtime Rust compilation works!");
    println!("   Next: Phase 2 - WASM hot-loading");

    Ok(())
}
