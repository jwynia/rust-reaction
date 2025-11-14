# Integration Test - Phase 3 Demo

This example demonstrates **Phase 3: Full Integration** - connecting the compiler and runtime to show the complete end-to-end flow.

## What This Proves

✅ **Compiler → Runtime integration works**
✅ **Hot-reload preserves component identity**
✅ **Version tracking across reloads**
✅ **Error handling prevents broken code from loading**
✅ **The complete safety mechanism works end-to-end!**

## Prerequisites

You need:
- **Rust toolchain**: https://rustup.rs/
- **wasm-pack**: `cargo install wasm-pack`
- **wasm32 target**: `rustup target add wasm32-unknown-unknown`

## Running

From repository root:
```bash
cargo run --bin test-integration
```

Or from this directory:
```bash
cargo run
```

## What It Does

The test demonstrates 6 steps:

### 1. Initialize Compiler and Runtime
Sets up the `SubprocessCompiler` and `ComponentRegistry`.

### 2. Compile Version 1
Compiles a simple greeting function:
```rust
#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}
```

Shows compilation time and WASM output size.

### 3. Load into Runtime
- Creates a `WasmComponent` from the compiled WASM bytes
- Assigns it a component ID based on content hash
- Registers it in the component registry
- Shows component metadata (ID, version, loaded_at)

### 4. Compile Version 2
Compiles an updated version with a different message:
```rust
#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("¡Hola, {}! 🎉", name)
}
```

### 5. Hot-Reload
- Retrieves the existing component from registry
- Calls `reload()` with new WASM bytes
- Component ID stays the same
- Version increments: 1 → 2
- App continues running without restart!

### 6. Error Handling
Attempts to compile code with a type error:
```rust
pub fn broken() -> String {
    let x: i32 = "not a number";  // Type error!
    x.to_string()
}
```

Shows:
- Compilation fails (as expected)
- Error is caught before loading
- App still running normally
- Registry unaffected

## Expected Output

```
🧬 Morpheus Integration Test - Phase 3

This demonstrates the complete flow:
  1. Compile Rust code → WASM
  2. Load WASM into runtime
  3. Hot-reload with new version

1. Initializing compiler and runtime...
   ✓ Compiler and registry ready

2. Compiling version 1 of component...
   [... shows code ...]
   ✓ Compiled in 7.45s
   - WASM size: 45123 bytes (44.06 KB)

3. Loading component into runtime...
   ✓ Component loaded:
     - ID: 0x1a2b3c4d5e6f7890
     - Version: 1
     - Loaded at: timestamp-1234567890
   ✓ Registered in component registry

4. Compiling version 2 (updated greeting)...
   [... shows updated code ...]
   ✓ Compiled in 7.32s
   - WASM size: 45234 bytes (44.17 KB)

5. Hot-reloading component with version 2...
   ✓ Hot-reload successful!
     - Version: 1 → 2
     - Component ID unchanged: 0x1a2b3c4d5e6f7890
     - App still running! No restart required.

6. Testing error handling (compile bad code)...
   ✓ Compilation correctly failed!
   ✓ App still running - error was caught before loading

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
✅ Phase 3 Complete: Full Integration Works!
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

What we demonstrated:
  ✓ Compile Rust → WASM
  ✓ Load WASM into runtime
  ✓ Hot-reload with new version
  ✓ Component ID preserved across reloads
  ✓ Version tracking
  ✓ Error handling prevents broken code from loading

🎯 The Safety Gate Works:
   Bad code is rejected BEFORE it can break the app!

Registry status:
  - Components loaded: 1
    • 0x1a2b3c4d5e6f7890 (v2)

Next: Phase 4 - Add visual UI component
```

## Why This Matters

This proves the **complete safety mechanism** for self-modifying apps:

```rust
// Compile (safety gate #1)
let wasm = compiler.compile(ai_code).await?;

// Load (creates isolated component)
let component = WasmComponent::load(&wasm, permissions).await?;
registry.register(component.id(), component, metadata);

// Hot-reload (preserves identity, increments version)
let component = registry.get_mut(&id)?;
component.reload(&new_wasm).await?;  // v1 → v2, same ID

// Error handling (prevents breakage)
match compiler.compile(bad_code).await {
    Ok(wasm) => /* load it */,
    Err(e) => {
        // App still works!
        show_user(e);
        ai.fix_and_retry(e);
    }
}
```

The three safety mechanisms working together:

1. **Compilation barrier**: Type errors caught before execution
2. **Component isolation**: Each component has its own identity and permissions
3. **Hot-reload**: Updates happen atomically without breaking the app

## Next Steps

**Phase 4**: Add Visual UI Component
- Render something to DOM
- Interactive counter or similar
- Modify it while running
- See the changes instantly

**Phase 5**: Add AI Integration
- LLM generates component code
- Compiler validates it
- Load and display
- AI fixes its own errors

**Phase 6**: Advanced Safety
- Permission enforcement (network, storage, APIs)
- State snapshots and rollback
- Sandboxed execution
- Audit logging
