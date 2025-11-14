# Compiler Test - Phase 1 Demo

This example demonstrates **Phase 1: Runtime Rust Compilation** - the foundational piece of Morpheus.

## What This Proves

✅ **Rust code can be compiled at runtime**
✅ **Type errors are caught before execution**
✅ **Compilation errors are properly reported**
✅ **The safety gate works!**

This is the critical difference from TypeScript - AI-generated code **must type-check** before it can run.

## Prerequisites

You need:
- **Rust toolchain**: https://rustup.rs/
- **wasm-pack**: `cargo install wasm-pack`
- **wasm32 target**: `rustup target add wasm32-unknown-unknown`

## Running

From repository root:
```bash
cargo run --bin test-compiler
```

Or from this directory:
```bash
cargo run
```

## What It Does

The test runs 5 steps:

### 1. Tool Check
Verifies `rustc` and `wasm-pack` are available.

### 2. Create Compiler
Initializes the `SubprocessCompiler`.

### 3. Compile Valid Code
Compiles a simple "Hello World" function:
```rust
#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}
```

Shows:
- Compilation time (~5-10 seconds)
- WASM output size
- Success!

### 4. Test Error Detection
Tries to compile code with a syntax error:
```rust
pub fn broken(x: i32) -> String {
    x.to_string(  // Missing )
}
```

Shows:
- Compilation correctly fails
- Error messages are captured
- App didn't crash!

### 5. Test Type Checking
Tries to compile code with a type error:
```rust
pub fn wrong_types() -> String {
    let x: i32 = "not a number";  // Type error!
    x.to_string()
}
```

Shows:
- Type error caught
- Faster than full compilation
- Safety gate working!

## Expected Output

```
🧬 Morpheus Compiler Test

1. Checking for required tools...
   ✓ rustc and wasm-pack found

2. Creating compiler...
   ✓ Compiler ready

3. Testing compilation of valid code...
   Source code:
   [... code ...]

   Compiling... (this may take 5-10 seconds)
   ✓ Compilation successful!
   - Time: 7.32s
   - WASM size: 45123 bytes
   - Size: 44.06 KB

4. Testing error detection...
   [... shows syntax error caught ...]

5. Testing type check...
   [... shows type error caught ...]

✅ All tests passed!

🎯 Phase 1 Complete: Runtime Rust compilation works!
   Next: Phase 2 - WASM hot-loading
```

## Why This Matters

This proves the **core safety mechanism** that makes Morpheus different from TypeScript:

**TypeScript:**
```typescript
eval(aiGeneratedCode);  // Runs immediately, crashes on error!
```

**Morpheus:**
```rust
match compiler.compile(aiCode).await {
    Ok(wasm) => load_safely(wasm),
    Err(errors) => {
        // App still works!
        show_user(errors);
        ai.fix_and_retry(errors);
    }
}
```

The compilation step is a **safety gate** that prevents broken code from ever reaching the running application.

## Performance

Current: **~5-10 seconds** per compilation

This is acceptable for:
- Proof of concept
- Infrequent modifications
- Interactive use with users

Future optimizations:
- Incremental compilation
- Component caching
- Pre-compiled templates
- Parallel compilation

## Next Steps

**Phase 2**: WASM Hot-Loading
- Load compiled WASM modules
- Call functions from them
- Hot-reload with new versions
- Preserve state across reloads

**Phase 3**: Integration
- Connect compiler → runtime
- Show end-to-end flow
- Demonstrate hot-reload

**Phase 4**: Add UI Component
- Make it visual
- Interactive counter
- Modify while running

**Phase 5**: Add AI
- LLM generates code
- Full conversation flow
- AI fixes its own errors

**Phase 6**: Safety Features
- Permissions enforcement
- State snapshots/rollback
- Sandboxed execution
