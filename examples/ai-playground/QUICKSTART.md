# Quick Start - 2 Minutes to Running

## 1. Get API Key (1 minute)

Visit: https://console.anthropic.com/

- Sign up / Log in
- Go to API Keys
- Create new key
- Copy it (starts with `sk-ant-`)

## 2. Configure (30 seconds)

```bash
cd examples/ai-playground
cp .env.example .env
nano .env  # Or use your favorite editor
```

Paste your API key:
```
ANTHROPIC_API_KEY=sk-ant-your-actual-key-here
```

Save and exit.

## 3. Run (30 seconds)

```bash
cargo run --bin morpheus-server
```

Wait for:
```
🚀 Server running at http://127.0.0.1:3000
   Open http://127.0.0.1:3000 in your browser
```

## 4. Use (starts immediately!)

1. Open: http://127.0.0.1:3000
2. Type: "Create a counter with increment and decrement buttons"
3. Click: "Generate Component"
4. Wait: ~10-15 seconds
5. See: Your working counter!

## 5. Try More

**Simple tests:**
- "Add a reset button"
- "Make the buttons green"
- "Change it to count by 5"

**New components:**
- "Create a color picker"
- "Build a todo list"
- "Make a calculator"

## Watch the Magic

The logs show you:
- AI generating Rust code
- Compilation (success or error)
- If error: AI seeing it and retrying
- Success: WASM loading into browser

**This is the full Morpheus loop!**

## Troubleshooting

**Server won't start:**
```bash
# Check if port 3000 is in use
lsof -ti:3000

# Kill it if needed
lsof -ti:3000 | xargs kill
```

**"ANTHROPIC_API_KEY not configured":**
- Check .env file exists in examples/ai-playground/
- Check the key starts with `sk-ant-`
- Try: `export ANTHROPIC_API_KEY=sk-ant-...` and run again

**Component doesn't appear:**
- Check browser console (F12) for errors
- Make sure you see "WASM module loaded" in logs
- Try a simpler request first

## What's Happening

```
Your request
    ↓
Claude AI generates Rust code
    ↓
Morpheus compiles to WASM
    ├─ Error? → AI fixes and retries (up to 5 times)
    └─ Success? → Loads into browser
    ↓
You see working component!
```

## Cost

Claude API pricing (as of 2024):
- Claude 3.5 Sonnet: $3 per million input tokens, $15 per million output tokens
- Each request: ~1,000-4,000 tokens
- Cost per component: ~$0.01-0.05
- 100 components: ~$1-5

This is a development demo - costs should be minimal.

## Next Steps

Read the full README.md to understand:
- How the retry loop works
- Architecture details
- Limitations and future work
- How to extend it

**Enjoy building with AI! 🧬**
