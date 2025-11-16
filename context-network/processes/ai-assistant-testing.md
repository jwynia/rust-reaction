# AI Assistant Testing Guidelines

## Purpose
This document outlines safe practices for AI assistants when testing applications to prevent accidentally terminating the TUI interface or other critical processes.

## Classification
- **Domain:** Process
- **Stability:** Semi-stable
- **Abstraction:** Detailed
- **Confidence:** Established

## Problem Statement

When testing whether applications run correctly, broad process management commands like `pkill` can accidentally kill the TUI interface used to interact with the AI assistant, terminating the entire session.

### Root Cause

Commands like the following are **too broad** and will kill unintended processes:

```bash
# DANGEROUS - Do not use these patterns
pkill -f cargo          # Kills ALL cargo processes
pkill -f rust           # Kills ALL rust processes  
pkill -f morpheus       # Kills anything with "morpheus" in command
pkill bash              # Kills ALL bash shells including the TUI!
pkill -f "cargo run"    # Still too broad
```

These commands can kill:
- The user's TUI interface (e.g., if running via cargo/rust)
- Other unrelated development processes
- The OpenCode/Claude interface itself
- Critical system processes

## Safe Process Management Practices

### 1. Track PIDs Explicitly (Recommended)

When starting processes for testing, capture and track their PIDs:

```bash
# Start process and capture PID
cargo run --bin morpheus &
MORPHEUS_PID=$!

# Later, kill only that specific process
kill $MORPHEUS_PID
```

**Advantages:**
- Surgical precision - only kills the intended process
- No risk of killing other processes
- Easy to track multiple test processes

### 2. Use Specific Process Matching

If you must search for processes, use very specific patterns:

```bash
# Find the specific process and get its PID
ps aux | grep "[c]argo run --bin morpheus" | awk '{print $2}' | xargs kill

# Or use pgrep with exact matching
pgrep -f "cargo run --bin morpheus$" | xargs kill
```

**Note:** The `[c]` in grep prevents grep from matching itself.

### 3. Use Job Control for Background Processes

For interactive testing, use shell job control:

```bash
# Start in background
cargo run --bin morpheus &

# List jobs
jobs

# Kill by job number
kill %1
```

### 4. Use Process Groups

For complex scenarios with multiple related processes:

```bash
# Start process group
setsid cargo run --bin morpheus &
PID=$!

# Kill entire process group later
kill -TERM -$PID
```

### 5. Manual Interrupt (Safest for Interactive Testing)

When possible, manually interrupt processes:

```bash
# Start process in foreground
cargo run --bin morpheus

# User can press Ctrl+C to stop
# Or AI can send SIGINT if supported
```

## Testing Workflow

### Before Starting a Test

1. **Plan process management** - Decide how you'll start AND stop the process
2. **Document the PID tracking strategy** - Tell the user how you'll manage processes
3. **Verify no name conflicts** - Check if similar process names exist

### During Testing

1. **Capture PIDs immediately** when starting processes
2. **Store PIDs in clearly named variables** (e.g., `MORPHEUS_PID`, `SERVER_PID`)
3. **Verify processes are running** with `ps -p $PID` before testing
4. **Monitor for unexpected termination**

### After Testing

1. **Kill processes using stored PIDs** - Never use broad pkill commands
2. **Verify process termination** with `ps -p $PID`
3. **Clean up PID variables** to avoid confusion
4. **Report any orphaned processes** to the user

## Prohibited Commands

The following commands are **NEVER** allowed:

```bash
❌ pkill bash
❌ pkill -f cargo
❌ pkill -f rust
❌ killall bash
❌ killall cargo
❌ killall rust
```

## Example: Safe Application Testing

```bash
# 1. Start the application and track its PID
echo "Starting morpheus application..."
cargo run --bin morpheus &
MORPHEUS_PID=$!

echo "Morpheus running with PID: $MORPHEUS_PID"

# 2. Wait for startup
sleep 3

# 3. Verify it's running
if ps -p $MORPHEUS_PID > /dev/null; then
    echo "Morpheus is running successfully"
else
    echo "Morpheus failed to start"
fi

# 4. Test the application
curl http://localhost:3002/health || echo "Health check failed"

# 5. Clean shutdown using the tracked PID
echo "Stopping morpheus..."
kill $MORPHEUS_PID

# 6. Verify termination
sleep 2
if ps -p $MORPHEUS_PID > /dev/null; then
    echo "Warning: Process still running, forcing termination"
    kill -9 $MORPHEUS_PID
else
    echo "Morpheus stopped cleanly"
fi
```

## Multiple Concurrent Processes

When testing multiple applications:

```bash
# Start multiple services with tracked PIDs
cargo run --bin service-a &
SERVICE_A_PID=$!

cargo run --bin service-b &
SERVICE_B_PID=$!

# Test them...

# Clean shutdown in reverse order
kill $SERVICE_B_PID
kill $SERVICE_A_PID

# Or shutdown all at once
kill $SERVICE_A_PID $SERVICE_B_PID
```

## Recovery from Session Loss

If the TUI session is accidentally terminated:

1. **Document what happened** - Note which command caused the termination
2. **Update this guide** - Add the problematic pattern to prohibited commands
3. **Inform the user** - Explain what went wrong and how to prevent it

## Exceptions

There are very rare cases where broad process killing might be acceptable:

1. **User explicitly requests it** - "kill all cargo processes"
2. **Emergency cleanup** - System is unresponsive and user approves
3. **Isolated test environment** - In a container where killing processes is safe

**Always confirm with the user before using broad kill commands.**

## Related Processes

- See `validation.md` for testing strategies
- See `creation.md` for development environment setup

## Relationships
- **Parent Nodes:** [processes/validation.md]
- **Related Nodes:** 
  - [processes/validation.md] - Testing is part of validation
  - [foundation/principles.md] - Safety is a core principle

## Navigation Guidance
- **Access Context:** Reference this document whenever testing applications or managing processes
- **Common Next Steps:** Follow safe process management patterns during validation
- **Related Tasks:** Application testing, process management, validation execution

## Metadata
- **Created:** 2025-11-15
- **Last Updated:** 2025-11-15
- **Updated By:** AI Assistant (OpenCode)

## Change History
- 2025-11-15: Initial creation based on lesson learned from TUI session termination due to broad pkill usage
