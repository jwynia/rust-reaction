//! Integration Test - Phase 3 Demo
//!
//! This example demonstrates the complete compiler → runtime → hot-reload flow.
//!
//! Run with: cargo run --bin test-integration

use morpheus_compiler::{Compiler, SubprocessCompiler};
use morpheus_runtime::{ComponentRegistry, WasmComponent};
use morpheus_core::permissions::Permissions;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("🧬 Morpheus Integration Test - Phase 3\n");
    println!("This demonstrates the complete flow:");
    println!("  1. Compile Rust code → WASM");
    println!("  2. Load WASM into runtime");
    println!("  3. Hot-reload with new version\n");

    // Step 1: Initialize components
    println!("1. Initializing compiler and runtime...");
    SubprocessCompiler::check_tools()?;
    let compiler = SubprocessCompiler::new().await?;
    let mut registry = ComponentRegistry::new();
    println!("   ✓ Compiler and registry ready\n");

    // Step 2: Compile version 1
    println!("2. Compiling version 1 of component...");
    let v1_code = r#"
        use wasm_bindgen::prelude::*;

        #[wasm_bindgen]
        pub fn greet(name: &str) -> String {
            format!("Hello, {}!", name)
        }
    "#;

    println!("   Source code (v1):");
    for line in v1_code.lines().filter(|l| !l.trim().is_empty()) {
        println!("   {}", line);
    }
    println!();

    println!("   Compiling...");
    let start = std::time::Instant::now();
    let wasm_v1 = compiler.compile(v1_code).await?;
    println!("   ✓ Compiled in {:.2}s", start.elapsed().as_secs_f64());
    println!("   - WASM size: {} bytes ({:.2} KB)\n", wasm_v1.len(), wasm_v1.len() as f64 / 1024.0);

    // Step 3: Load into runtime
    println!("3. Loading component into runtime...");
    let component = WasmComponent::load(&wasm_v1, Permissions::default()).await?;
    let component_id = component.id();
    let metadata = component.metadata().clone();

    println!("   ✓ Component loaded:");
    println!("     - ID: {}", component_id);
    println!("     - Version: {}", metadata.version);
    println!("     - Loaded at: {}", metadata.loaded_at);

    registry.register(component_id, component, metadata);
    println!("   ✓ Registered in component registry\n");

    // Step 4: Compile version 2
    println!("4. Compiling version 2 (updated greeting)...");
    let v2_code = r#"
        use wasm_bindgen::prelude::*;

        #[wasm_bindgen]
        pub fn greet(name: &str) -> String {
            format!("¡Hola, {}! 🎉", name)
        }
    "#;

    println!("   Source code (v2 - now with emoji!):");
    for line in v2_code.lines().filter(|l| !l.trim().is_empty()) {
        println!("   {}", line);
    }
    println!();

    println!("   Compiling...");
    let start = std::time::Instant::now();
    let wasm_v2 = compiler.compile(v2_code).await?;
    println!("   ✓ Compiled in {:.2}s", start.elapsed().as_secs_f64());
    println!("   - WASM size: {} bytes ({:.2} KB)\n", wasm_v2.len(), wasm_v2.len() as f64 / 1024.0);

    // Step 5: Hot-reload
    println!("5. Hot-reloading component with version 2...");
    let component = registry.get_mut(&component_id)
        .ok_or_else(|| anyhow::anyhow!("Component not found"))?;

    let old_version = component.metadata().version;
    component.reload(&wasm_v2).await?;
    let new_version = component.metadata().version;

    println!("   ✓ Hot-reload successful!");
    println!("     - Version: {} → {}", old_version, new_version);
    println!("     - Component ID unchanged: {}", component_id);
    println!("     - App still running! No restart required.\n");

    // Step 6: Demonstrate error handling
    println!("6. Testing error handling (compile bad code)...");
    let bad_code = r#"
        use wasm_bindgen::prelude::*;

        #[wasm_bindgen]
        pub fn broken() -> String {
            let x: i32 = "not a number";  // Type error!
            x.to_string()
        }
    "#;

    println!("   Attempting to compile code with type error...");
    match compiler.compile(bad_code).await {
        Ok(_) => {
            println!("   ✗ Should have failed!");
            return Err(anyhow::anyhow!("Bad code compiled!"));
        }
        Err(e) => {
            println!("   ✓ Compilation correctly failed!");
            println!("   ✓ App still running - error was caught before loading");
            println!("\n   Error preview:");
            for line in e.to_string().lines().take(5) {
                println!("     {}", line);
            }
        }
    }

    // Summary
    println!("\n");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("✅ Phase 3 Complete: Full Integration Works!");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!();
    println!("What we demonstrated:");
    println!("  ✓ Compile Rust → WASM");
    println!("  ✓ Load WASM into runtime");
    println!("  ✓ Hot-reload with new version");
    println!("  ✓ Component ID preserved across reloads");
    println!("  ✓ Version tracking");
    println!("  ✓ Error handling prevents broken code from loading");
    println!();
    println!("🎯 The Safety Gate Works:");
    println!("   Bad code is rejected BEFORE it can break the app!");
    println!();
    println!("Registry status:");
    println!("  - Components loaded: {}", registry.list().count());
    for meta in registry.list() {
        println!("    • {} (v{})", meta.id, meta.version);
    }
    println!();
    println!("Next: Phase 4 - Add visual UI component");

    Ok(())
}
