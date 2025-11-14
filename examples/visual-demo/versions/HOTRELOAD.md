# Hot-Reload Demo Instructions

This directory contains three different versions of the counter component to demonstrate hot-reload capabilities.

## The Three Versions

### Version 1: Blue Theme (Default)
- Clean, professional blue color scheme
- Standard buttons and layout
- Located in: `src/lib.rs`

### Version 2: Green Theme
- Eco-friendly green design
- Enhanced styling with shadows
- Shows same functionality, different appearance
- Located in: `versions/lib_v2.rs`

### Version 3: Animated
- Dynamic color changes based on count value
- Hover animations on buttons
- Rainbow border animation
- Demonstrates adding new visual features
- Located in: `versions/lib_v3.rs`

## How to Experience Hot-Reload

### Manual Method (Current Phase 4)

1. **Build and run Version 1:**
   ```bash
   wasm-pack build --target web
   # Start web server (see main README)
   # Open browser to http://localhost:8080/public/
   ```

2. **Hot-reload to Version 2:**
   ```bash
   # Copy v2 over the current version
   cp versions/lib_v2.rs src/lib.rs

   # Rebuild
   wasm-pack build --target web

   # Refresh browser - see green theme!
   ```

3. **Hot-reload to Version 3:**
   ```bash
   # Copy v3 over the current version
   cp versions/lib_v3.rs src/lib.rs

   # Rebuild
   wasm-pack build --target web

   # Refresh browser - see animations!
   ```

4. **Return to Version 1:**
   ```bash
   git checkout src/lib.rs
   wasm-pack build --target web
   # Refresh browser
   ```

### What You'll See

**Version 1 → Version 2:**
- Background changes from white to green gradient
- Buttons get new styling and labels
- Border becomes green
- Text mentions "Eco Mode"

**Version 2 → Version 3:**
- Background becomes yellow/amber gradient
- Count display gets pulse animation
- Buttons have hover effects
- Border has rainbow animation
- Count color changes based on value:
  - Positive = Green
  - Zero = Blue
  - Negative = Red

## What This Demonstrates

This manual hot-reload process shows what Morpheus will do automatically:

```rust
// What you do manually now:
// 1. Edit src/lib.rs
// 2. Run wasm-pack build
// 3. Refresh browser

// What Morpheus Phase 5+ will do automatically:
User: "Make the counter green with nature theme"
AI: generates Version 2 code
Morpheus: compiles it (5-10 sec)
Morpheus: hot-reloads into browser
User: sees green counter (no refresh needed!)

User: "Add animations when I click"
AI: generates Version 3 code
Morpheus: compiles it
Morpheus: hot-reloads
User: sees animations immediately
```

## The Safety Gate in Action

Try introducing an error:

```bash
# Edit src/lib.rs and break something:
# - Remove a semicolon
# - Use wrong type
# - Call undefined function

wasm-pack build --target web
# ❌ Build fails with error message
# ✅ Browser still shows last working version
# ✅ Application didn't crash!
```

This is the key difference from JavaScript:
- **JavaScript**: Bad code crashes the app at runtime
- **Morpheus**: Bad code fails to compile, app keeps running

## Future Phases

**Phase 5: AI Integration**
- User says what they want in natural language
- LLM generates Rust code (like these versions)
- Automatic compilation
- Hot-reload without manual steps

**Phase 6: Advanced Features**
- State preservation across reloads (counter value doesn't reset!)
- Rollback to previous versions
- A/B testing (run v1 and v2 simultaneously)
- Permission enforcement (what APIs can component use)

## Notes

- Current implementation requires browser refresh after rebuild
- Future implementation will inject new WASM without refresh
- State (counter value) is currently lost on reload
- Phase 6 will preserve state across hot-reloads
