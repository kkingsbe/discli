# Image Support for discli - Architectural Plan

## Executive Summary

This plan provides a comprehensive architectural analysis for adding image attachment support to the discli Discord CLI tool. The current implementation supports only text messages via a simple single-command interface. This document evaluates multiple architectural approaches, analyzes trade-offs, and recommends a solution that balances simplicity, extensibility, and user experience.

**Key Findings:**
- Current implementation uses direct Discord API calls with JSON payloads
- Three viable architectural approaches identified
- Recommended approach: **Hybrid Subcommand Architecture** (Approach 3)
- Primary implementation effort: ~4-6 major tasks
- Key dependencies to add: clap for CLI parsing, optional image validation library

---

## Current Implementation Analysis

### Codebase Overview

**Main Entry Point**: [`src/main.rs`](src/main.rs:1)

The current implementation follows a straightforward architecture:

```
┌─────────────────────────────────────────────────────────────┐
│                     CLI Entry Point                         │
│  Parse args[1] as message content                            │
└────────────────────┬────────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────────┐
│           Environment Configuration                         │
│  • DISCORD_TOKEN (bot authentication)                       │
│  • DISCORD_CHANNEL_ID (target channel)                     │
│  • Loaded via dotenv from discli.env                         │
└────────────────────┬────────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────────┐
│         Discord API Request Builder                          │
│  • HTTP POST to /channels/{id}/messages                     │
│  • JSON payload: {"content": message}                       │
│  • Authorization header: "Bot {token}"                      │
└────────────────────┬────────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────────┐
│           Response Handling                                 │
│  • Success: Print confirmation                             │
│  • Error: Print error message and exit(1)                 │
└─────────────────────────────────────────────────────────────┘
```

### Current Dependencies

| Crate | Version | Purpose | Current Usage |
|-------|---------|---------|---------------|
| `reqwest` | 0.12 | HTTP client | Discord API requests |
| `tokio` | 1.40 | Async runtime | Main async executor |
| `serde` | 1.0 | Serialization | JSON handling |
| `serde_json` | 1.0 | JSON serialization | Request body construction |
| `dotenv` | 0.15 | Environment variables | Loading discli.env |

**Key Observations:**
- No multipart form support (required for file uploads)
- No command-line argument parsing library (currently using manual `env::args()`)
- No image validation or processing capabilities
- No subcommand structure

### Current Message Flow

```
discli "Hello, World!"
       │
       ▼
Load DISCORD_TOKEN and DISCORD_CHANNEL_ID
       │
       ▼
Construct JSON: {"content": "Hello, World!"}
       │
       ▼
POST https://discord.com/api/v10/channels/{id}/messages
       │
       ▼
Handle response
```

### Discord API Considerations

**Current Method:** JSON Payload
- Endpoint: `POST /channels/{id}/messages`
- Content-Type: `application/json`
- Body: `{"content": "text message"}`
- Limitations: No file attachments

**Image Upload Method Required:**
- Endpoint: `POST /channels/{id}/messages`
- Content-Type: `multipart/form-data`
- Fields:
  - `content`: Text message (optional)
  - `files[N]`: Binary file attachments (N = 0, 1, 2, ... up to 10)
  - `payload_json`: JSON metadata for attachments (optional)
- Limits:
  - Max 10 attachments per message
  - Max 25 MB per file
  - Max 8 MB for non-Nitro users in DMs

**Alternative: Image URL Embedding**
- Use JSON payload with `embeds` array
- Include image URLs in embed objects
- Requires images to be hosted externally
- Simpler API (no multipart)
- Less control over image hosting

---

## Problem Definition

### Core Challenge

The discli tool currently only supports text-only messages. Users need to send Discord messages containing images as attachments, potentially combined with text content.

### Constraints

1. **Planning Only**: This is a planning task - no code implementation
2. **Separate Command Consideration**: The request mentions potentially implementing as a separate command
3. **Backward Compatibility**: Existing CLI usage should not break
4. **Simplicity**: Tool should remain easy to use for basic text messages
5. **Discord API Limits**: Must work within Discord's API constraints (10 attachments, 25MB each)
6. **Rust Ecosystem**: Should leverage Rust's robust CLI and async libraries

### Success Criteria

- [ ] Users can send messages with image attachments
- [ ] Support for both image file upload and image URL embedding
- [ ] Ability to combine text and images in a single message
- [ ] Clear, intuitive CLI interface for specifying images
- [ ] Proper error handling for invalid/missing images
- [ ] Support for multiple images per message (up to Discord's limit)
- [ ] Existing text-only functionality remains unchanged
- [ ] Documentation updated with examples

### Scope Boundaries

**In Scope:**
- Image attachment support (file upload)
- Image URL embedding
- Text + image combination
- CLI argument design for image specification
- Error handling and validation
- Dependency evaluation

**Out of Scope:**
- Other media types (videos, audio, documents)
- Complex Discord embeds (author, footer, fields, etc.)
- Message editing or deletion
- Discord webhook support
- Interactive components (buttons, select menus)
- Rich formatting beyond images

---

## Architectural Approaches

### Approach 1: Extend Existing Message Command

**Summary**: Add optional image attachment flags to the current single-command interface.

**Design:**
```
# Single command with optional flags
discli "Text message" --attach path/to/image.png
discli "Text message" --attach img1.png --attach img2.jpg
discli --attach image.png  # No text, just image
discli --embed-url https://example.com/image.jpg  # URL embedding
```

**Architecture:**
```
Main Entry Point
├── Parse arguments with enhanced flag parsing
├── Detect image attachment flags
│   ├── --attach / -a: File path to upload
│   ├── --embed-url / -e: Image URL to embed
│   └── --caption: Optional caption for images
├── Validate image files (existence, size, format)
├── Construct request based on attachments present
│   ├── No images: JSON (existing behavior)
│   └── Has images: multipart/form-data
└── Send to Discord API
```

**Required Changes:**

| File | Changes |
|------|---------|
| `Cargo.toml` | Add `clap` for CLI argument parsing |
| `src/main.rs` | Refactor argument parsing, add attachment handling |
| `src/discord.rs` (new) | Extract Discord API logic into separate module |
| `src/message.rs` (new) | Message construction logic |
| `src/attachment.rs` (new) | Image validation and preparation |

**Dependencies to Add:**
- `clap` ~4.5 for CLI argument parsing
- Optionally: `image` crate for image validation

**Pros:**
- ✅ Minimal learning curve for existing users
- ✅ Simple, single-command interface maintained
- ✅ Backward compatible - existing usage unchanged
- ✅ Fast to implement (smaller codebase changes)
- ✅ Low cognitive load - one command to learn
- ✅ Follows Unix philosophy of composing tools

**Cons:**
- ❌ Command can become complex with many flags
- ❌ Harder to extend beyond images (future features)
- ❌ Mixing concerns (text-only vs. rich messages)
- ❌ Argument validation logic becomes complex
- ❌ Help text becomes verbose
- ❌ Harder to deprecate or change image-specific flags later

**Best When:**
- Simplicity is the highest priority
- Future expansion needs are minimal
- User base prefers single-command tools
- Development resources are limited

---

### Approach 2: Separate Dedicated Command

**Summary**: Create a new command specifically for rich messages, leaving the original unchanged.

**Design:**
```
# Original command (unchanged)
discli "Simple text message"

# New command for rich messages
discli-image "Caption text" --attach path/to/image.png
discli-image "Caption" --attach img1.png --attach img2.jpg
discli-image --attach image.png  # Caption optional
discli-image --embed-url https://example.com/image.jpg
```

**Architecture:**
```
Workspace Structure
├── discli/
│   └── src/main.rs (unchanged - text only)
└── discli-image/
    └── src/main.rs (new - image support)
```

Or (single binary with subcommands):

```
Main Entry Point
├── Parse command: discli vs discli-image
├── Route to appropriate handler
│   ├── discli: Original text-only logic
│   └── discli-image: New image-rich logic
└── Execute respective handler
```

**Required Changes:**

| File | Changes |
|------|---------|
| `Cargo.toml` | Add `[[bin]]` for discli-image, add dependencies |
| `src/main.rs` | Keep mostly unchanged, possibly add command routing |
| `src/image_main.rs` (new) | Entry point for discli-image command |
| `src/discord.rs` (new) | Shared Discord API logic |
| `src/attachment.rs` (new) | Image-specific logic |
| `README.md` | Update with both commands |

**Dependencies to Add:**
- `clap` ~4.5 for CLI argument parsing
- Optionally: `image` crate for validation

**Pros:**
- ✅ Complete backward compatibility (zero changes to existing command)
- ✅ Clear separation of concerns
- ✅ Original command remains simple and focused
- ✅ Can evolve independently (text vs. rich messages)
- ✅ Help text stays clean for each command
- ✅ Easy to understand: simple vs. rich messaging

**Cons:**
- ❌ Two binaries increase installation footprint
- ❌ Users must learn two commands
- ❌ Code duplication (shared logic needs refactoring)
- ❌ Confusing which command to use in some cases
- ❌ Maintenance burden for two entry points
- ❌ Doesn't scale well for future features (embeds, reactions?)

**Best When:**
- Zero risk tolerance for existing functionality
- Clear use case distinction (always simple vs. always rich)
- Team prefers clear, separated concerns
- Limited future expansion planned

---

### Approach 3: Hybrid Subcommand Architecture

**Summary**: Implement a modern subcommand structure that supports both simple and rich messages, with extensibility for future features.

**Design:**
```
# Text-only message (explicit or default)
discli send "Hello, World!"
discli message "Hello, World!"  # Aliased command

# Rich message with images
discli send "Caption" --attach path/to/image.png
discli send --attach image.png  # No text

# Image-specific subcommand (convenience)
discli image --attach photo.jpg --caption "My photo"
discli image --embed-url https://example.com/img.jpg

# Rich message with embeds (future expansion)
discli rich --title "Title" --image https://url.com/img.png
```

**Architecture:**
```
┌─────────────────────────────────────────────────────────────┐
│                    Main Entry Point                          │
│  Parse subcommands using clap                               │
└────────────────────┬────────────────────────────────────────┘
                     │
        ┌────────────┼────────────┐
        │            │            │
        ▼            ▼            ▼
┌──────────────┐ ┌──────────┐ ┌──────────────┐
│  send/message│ │  image   │ │   rich*      │
│  Subcommand  │ │Subcommand│ │ Subcommand   │
└──────┬───────┘ └────┬─────┘ └──────┬───────┘
       │              │                │
       │         ┌────┴────┐           │
       │         ▼         ▼           │
       │    --attach  --embed-url      │
       │              │                │
       └──────────────┼────────────────┘
                      ▼
          ┌───────────────────────┐
          │  Message Builder     │
          │  - Compose content   │
          │  - Add attachments   │
          │  - Add embeds        │
          └───────────┬───────────┘
                      │
                      ▼
          ┌───────────────────────┐
          │  Discord API Module   │
          │  - JSON request      │
          │  - Multipart upload  │
          │  - Response handling │
          └───────────────────────┘
```

**Module Structure:**
```
src/
├── main.rs              # Entry point, CLI parsing
├── cli.rs               # CLI argument definitions
├── commands/
│   ├── mod.rs
│   ├── send.rs          # Send message command
│   ├── image.rs         # Image-specific command
│   └── rich.rs          # Rich message command (future)
├── discord/
│   ├── mod.rs
│   ├── api.rs           # Discord API client
│   ├── types.rs         # Discord API types
│   └── multipart.rs     # Multipart form handling
├── message/
│   ├── mod.rs
│   ├── builder.rs       # Message builder pattern
│   ├── attachment.rs    # Attachment handling
│   └── validation.rs    # Input validation
└── config/
    ├── mod.rs
    └── env.rs           # Environment configuration
```

**Required Changes:**

| File | Changes |
|------|---------|
| `Cargo.toml` | Add clap, reorganize dependencies |
| `src/main.rs` | Complete refactor to subcommand architecture |
| `src/cli.rs` (new) | CLI argument definitions |
| `src/commands/` (new) | Subcommand implementations |
| `src/discord/` (new) | Discord API layer |
| `src/message/` (new) | Message construction |
| `src/config/` (new) | Configuration handling |
| `README.md` | Update documentation |

**Dependencies to Add:**
- `clap` ~4.5 with `derive` feature for CLI parsing
- `mime` or `mime_guess` ~0.4 for MIME type detection
- Optionally: `image` crate for validation
- Optionally: `reqwest` multipart feature (might need version update)

**Pros:**
- ✅ Highly extensible for future features (embeds, reactions, etc.)
- ✅ Clean separation of concerns
- ✅ Modern CLI pattern (users expect subcommands)
- ✅ Can add commands without affecting existing ones
- ✅ Shared logic reduces duplication
- ✅ Better code organization and testability
- ✅ Professional architecture, easier to maintain
- ✅ Single binary, single installation
- ✅ Help text stays organized

**Cons:**
- ❌ More complex initial implementation
- ❌ Longer development time
- ❌ Requires refactoring existing code
- ❌ Steeper learning curve for contributors
- ❌ Might feel "over-engineered" for simple use cases
- ❌ Changes to existing command (from positional to subcommand)

**Best When:**
- Project is expected to grow in features
- Team values maintainability and extensibility
- Multiple developers working on codebase
- Future plans for more Discord features
- Professional, long-lived tool

---

### Approach 4: Flexible Message Builder Pattern

**Summary**: Introduce a declarative message specification format (JSON/YAML) for complex messages, with CLI shortcuts for common cases.

**Design:**
```
# Simple text (backward compatible)
discli "Hello"

# Image via flags (convenience)
discli "Caption" --image photo.png
discli --image photo.jpg

# Rich message via specification file
discli --spec message.json
discli --spec message.yaml

# Specification file example (JSON)
{
  "content": "My message",
  "attachments": [
    {"path": "photo.png", "description": "Caption"}
  ],
  "embeds": [
    {
      "title": "Title",
      "image": {"url": "https://example.com/img.png"}
    }
  ]
}
```

**Architecture:**
```
Main Entry Point
├── Parse CLI arguments
│   ├── Simple case: positional text message
│   ├── Flags case: --image, --embed-url
│   └── Spec case: --spec <file>
├── Route to appropriate handler
│   ├── Simple: Direct JSON payload
│   ├── Flags: Build message from flags
│   └── Spec: Parse spec, build message
├── Message Builder (unified)
│   ├── From simple string
│   ├── From CLI flags
│   └── From specification file
└── Send to Discord API
```

**Required Changes:**

| File | Changes |
|------|---------|
| `Cargo.toml` | Add clap, serde_yaml, optional clap |
| `src/main.rs` | Major refactor to support multiple input modes |
| `src/builder.rs` (new) | Message builder pattern |
| `src/spec.rs` (new) | Specification file parsing |
| `src/discord.rs` (new) | Discord API layer |
| `README.md` | Update with spec file documentation |

**Dependencies to Add:**
- `clap` ~4.5 for CLI parsing
- `serde_yaml` ~0.9 for YAML support
- Optionally: `image` crate for validation

**Pros:**
- ✅ Extremely flexible - any Discord message type possible
- ✅ Perfect for complex, pre-defined messages
- ✅ Spec files can be version controlled
- ✅ Great for CI/CD pipelines (commit message specs)
- ✅ Keeps CLI simple for basic use cases
- ✅ Future-proof for any Discord feature
- ✅ Allows programmatic message generation

**Cons:**
- ❌ Adds complexity (three ways to send messages)
- ❌ Requires users to learn specification format
- ❌ Spec files can be error-prone without validation
- ❌ Harder to inline in shell scripts
- ❌ Overkill for simple use cases
- ❌ More dependencies (YAML parsing)

**Best When:**
- Complex, varied message types needed
- Messages defined in code/CI/CD pipelines
- Team comfortable with configuration files
- Full Discord API feature support desired

---

## Evaluation Matrix

### Comparison of All Approaches

| Criterion | Approach 1: Extend | Approach 2: Separate | Approach 3: Subcommands | Approach 4: Builder |
|-----------|-------------------|---------------------|------------------------|---------------------|
| **Feasibility** | ✅ Strong | ✅ Strong | ✅ Strong | ✅ Strong |
| **Implementation Time** | ✅ Short | ⚠️ Medium | ❌ Longer | ⚠️ Medium |
| **Backward Compatibility** | ⚠️ Moderate | ✅ Complete | ❌ Breaks | ✅ Strong |
| **Extensibility** | ❌ Weak | ⚠️ Moderate | ✅ Strong | ✅ Strong |
| **User Experience** | ⚠️ Mixed (cluttered) | ⚠️ Mixed (confusing) | ✅ Good | ⚠️ Mixed (complex) |
| **Code Maintainability** | ⚠️ Moderate (spaghetti) | ⚠️ Moderate (duplication) | ✅ Strong | ✅ Strong |
| **Learning Curve** | ✅ Low | ⚠️ Medium (two commands) | ⚠️ Medium | ❌ High |
| **Single Binary** | ✅ Yes | ❌ No | ✅ Yes | ✅ Yes |
| **Testability** | ⚠️ Moderate | ⚠️ Moderate | ✅ Strong | ✅ Strong |
| **Future Expansion** | ❌ Poor | ⚠️ Limited | ✅ Excellent | ✅ Excellent |
| **Error Handling** | ⚠️ Moderate | ⚠️ Moderate | ✅ Strong | ✅ Strong |
| **CI/CD Friendly** | ✅ Yes | ⚠️ Medium (two binaries) | ✅ Yes | ✅ Excellent |

### Detailed Analysis

#### Implementation Effort

| Approach | New Modules | Lines to Modify | Dependencies | Complexity |
|----------|-------------|-----------------|--------------|------------|
| 1: Extend | 2-3 | ~50 | 1 (clap) | Low |
| 2: Separate | 3-4 | ~20 + new file | 1 (clap) | Low-Medium |
| 3: Subcommands | 6-8 | ~100 | 2-3 (clap, mime) | Medium-High |
| 4: Builder | 4-5 | ~80 | 3 (clap, serde_yaml) | Medium |

#### User Experience Scenarios

**Scenario 1: Quick notification from shell script**
```
# Approach 1: Quick, familiar
discli "Build failed" --attach error-log.png

# Approach 2: Need to know which command
discli-image "Build failed" --attach error-log.png

# Approach 3: Explicit but longer
discli send "Build failed" --attach error-log.png

# Approach 4: Overkill
discli "Build failed" --image error-log.png
```

**Scenario 2: Multiple images with text**
```
# Approach 1: Long command line
discli "Report" --attach fig1.png --attach fig2.png --attach fig3.png

# Approach 2: Still long
discli-image "Report" --attach fig1.png --attach fig2.png --attach fig3.png

# Approach 3: Clean, can group
discli send "Report" --attach fig1.png fig2.png fig3.png

# Approach 4: Use spec file
discli --spec report.yaml
```

**Scenario 3: CI/CD pipeline with rich message**
```
# Approach 1: Cumbersome
discli "Deployed v1.2.3" --attach coverage.png --attach perf.png --embed-url https://ci.example.com/artifact.png

# Approach 2: Still cumbersome
discli-image "Deployed v1.2.3" --attach coverage.png --attach perf.png --embed-url https://ci.example.com/artifact.png

# Approach 3: Better with subcommands
discli image --caption "Deployed v1.2.3" --attach coverage.png perf.png --embed-url https://ci.example.com/artifact.png

# Approach 4: Best - committed spec
discli --spec ci-notification.json
```

---

## Deep Analysis: Red Team Evaluation

### For Approach 1 (Extend Existing)

**Pre-Mortem**: 6 months from now, Approach 1 failed. What went wrong?
- The `--attach` flag had to support multiple formats, sizes, and edge cases
- Users wanted `--embed-url`, `--caption`, `--alt-text`, `--position`...
- Command line became unmanageable: `discli "text" --attach img1.png --attach img2.png --caption "Hello" --alt-text "..." --embed-url https://...`
- Adding new features required touching the single main.rs file repeatedly
- Code became a mess of if-else branches

**Assumption Audit**:
- ❓ Assumption: "Simple flags will be enough" → Untested: How complex will image options get?
- ❓ Assumption: "Users won't need more than basic images" → Untested: What about image compression, resizing?
- ❓ Assumption: "Discord API won't change much" → Untested: What if Discord adds new attachment features?

**Edge Cases**:
- User provides invalid image path → Current error handling may not be sufficient
- Image exceeds Discord size limit → Need validation before upload
- Multiple attachments with mixed upload/URL → Complex request construction
- Unicode in file paths → Need to ensure proper encoding
- Network timeout during large upload → Need retry logic

**Stakeholder Objections**:
- Developer: "This is hard to test, everything is in main.rs"
- User: "I can't remember all the flags for image options"
- Maintainer: "Adding embed support will break the single-file approach"

**Second-Order Effects**:
- Each new image feature makes CLI help text longer
- Documentation becomes complex
- Contributing becomes harder (risk of breaking simple text messages)

---

### For Approach 2 (Separate Command)

**Pre-Mortem**: 6 months from now, Approach 2 failed. What went wrong?
- Users confused: "Which command do I use for a message with an image?"
- Code duplication: Both commands implement Discord API logic separately
- Adding a third type of message required a third binary
- Package size doubled with two separate binaries

**Assumption Audit**:
- ❓ Assumption: "Clear use case separation" → Untested: What about borderline cases?
- ❓ Assumption: "Easy to share code between binaries" → Untested: Workspace complexity?
- ❓ Assumption: "Users will understand two commands" → Untested: User testing?

**Edge Cases**:
- User wants to convert from text to rich message → Which command to learn?
- Shared dependencies need updates → Both binaries affected
- Documentation duplication → Inconsistent help texts

**Stakeholder Objections**:
- User: "Why are there two commands? Why not just one?"
- Developer: "I fixed a bug in discli but forgot discli-image"
- Package maintainer: "Two binaries complicate packaging"

**Second-Order Effects**:
- Maintenance burden doubles
- Users learn wrong command and get frustrated
- Features added to one command but not the other

---

### For Approach 3 (Subcommand Architecture)

**Pre-Mortem**: 6 months from now, Approach 3 failed. What went wrong?
- Over-engineering for simple use cases
- Too many abstractions for a small tool
- Contributors struggle to understand module structure
- Breaking change for existing users (they need to change scripts)

**Assumption Audit**:
- ❓ Assumption: "Users expect subcommands" → Untested: Are current users CLI-savvy?
- ❓ Assumption: "Future features will justify this architecture" → Untested: What features?
- ❓ Assumption: "Team will maintain this structure" → Untested: Team size and skill level?

**Edge Cases**:
- Breaking existing scripts that use `discli "message"` → Need migration guide
- Adding new subcommands → Risk of inconsistency
- Module boundaries become blurry over time

**Stakeholder Objections**:
- User: "Why did my script break? I just updated discli"
- Developer: "This is too complex for what we need"
- Project lead: "We're spending more time on architecture than features"

**Second-Order Effects**:
- Longer time to first release with image support
- Higher barrier to entry for contributors
- Need for comprehensive documentation of architecture

---

### For Approach 4 (Builder Pattern)

**Pre-Mortem**: 6 months from now, Approach 4 failed. What went wrong?
- Users found spec files cumbersome for simple cases
- YAML/JSON errors were hard to debug
- Too many ways to do the same thing caused confusion
- Tool felt "bloaty" with three different input methods

**Assumption Audit**:
- ❓ Assumption: "Spec files are powerful" → Untested: Will users actually use them?
- ❓ Assumption: "CI/CD teams will love this" → Untested: Do they already use other tools?
- ❓ Assumption: "Can keep CLI simple as fallback" → Untested: Or will it get ignored?

**Edge Cases**:
- Invalid YAML/JSON → Need good error messages
- Spec file references missing images → Complex error reporting
- Mix of inline and spec-file usage → User confusion

**Stakeholder Objections**:
- User: "I just want to send an image, why do I need to write YAML?"
- DevOps: "This adds another config file to maintain"
- Developer: "Three code paths to test and maintain"

**Second-Order Effects**:
- Documentation becomes complex (three ways to do things)
- Testing matrix explodes (simple, flags, spec file)
- Users stick to one method and ignore others

---

## Recommendation

### Primary Recommendation: Approach 3 (Hybrid Subcommand Architecture)

**Rationale:**

After thorough analysis and red-teaming each approach, **Approach 3** emerges as the best choice for the following reasons:

#### 1. **Balances Competing Priorities**
- **Simplicity vs. Extensibility**: While more complex initially, it provides a clean foundation for future growth
- **Backward Compatibility**: Can maintain compatibility through backward-compatible CLI design (see migration strategy)
- **User Experience**: Professional CLI pattern that users of modern tools expect

#### 2. **Professional, Maintainable Architecture**
- Clear separation of concerns through module structure
- Easier to test each component independently
- Reduces technical debt as features are added
- Follows Rust ecosystem best practices

#### 3. **Future-Proof Design**
- Adding embed support? New subcommand or flag to `rich`
- Adding reaction support? Extend `send` command
- Adding webhook support? New subcommand `webhook`
- All without major refactoring

#### 4. **Addresses All Success Criteria**
- ✅ Image attachment support
- ✅ URL embedding capability
- ✅ Text + image combination
- ✅ Clear CLI interface
- ✅ Proper error handling
- ✅ Multiple images per message
- ✅ Backward compatibility (with migration strategy)
- ✅ Documentation support

#### 5. **Mitigates Risks from Red Team Analysis**
- **Over-engineering concern**: Start with minimal modules, add as needed
- **Breaking changes concern**: Implement with deprecation warnings for old syntax
- **Complexity concern**: Provide comprehensive documentation and examples

### Migration Strategy for Backward Compatibility

To address the backward compatibility concern:

```rust
// Phase 1: Support both old and new syntax
discli "Hello"                          // Old: works with deprecation warning
discli send "Hello"                     // New: recommended
discli message "Hello"                  // New: alias to send

// Phase 2 (after 6 months): Still support but warning more prominent
discli "Hello"                          // Works, warns strongly to use "send"

// Phase 3 (after 12 months): Remove old syntax
discli "Hello"                          // Error: use "discli send" instead
```

### Alternative: If Simplicity is Critical

If the project has strict constraints on:
- Development time/resources
- Team size (single developer)
- User base (few users, simple needs)

Then **Approach 1** (Extend Existing) may be appropriate as a first step, with a plan to migrate to Approach 3 later when the tool matures.

### Decision Points

The following questions could change this recommendation:

1. **"When do you need this feature released?"**
   - < 2 weeks: Consider Approach 1 or 2
   - > 1 month: Approach 3 is viable

2. **"What are the future feature plans?"**
   - Just images: Any approach works
   - Embeds, reactions, editing: Approach 3 or 4

3. **"What's the current user base size?"**
   - Large (100+ users): Approach 3 (don't break them)
   - Small (< 10 users): More flexibility

4. **"Team size and skill level?"**
   - Single, junior dev: Approach 1 (simpler)
   - Multiple, experienced: Approach 3

**Assumptions to Validate Before Proceeding:**
- Confirm project timeline and resource constraints
- Validate future feature roadmap with stakeholders
- Survey current users on command preference (if any)
- Assess team's comfort level with refactoring

---

## Risk Mitigation

### High Priority Risks

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| Breaking existing user scripts | Medium | High | Implement deprecation period, migration guide |
| Over-engineering for needs | Medium | Medium | Start minimal, add modules as needed |
| Multipart form data issues | Low | High | Leverage reqwest's proven multipart support |
| Image validation complexity | Low | Medium | Use existing image crate, validate only size/type |
| Discord API rate limiting | Low | Medium | Implement retry with exponential backoff |

### Medium Priority Risks

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| Large file upload timeouts | Low | Medium | Add progress indication, configurable timeout |
| MIME type detection errors | Low | Medium | Use mime_guess with fallback to application/octet-stream |
| Unicode in file paths | Low | Low | Ensure proper encoding throughout |
| Discord API changes | Low | High | Pin reqwest version, monitor Discord changelog |

### Low Priority Risks

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| Dependency conflicts | Low | Medium | Carefully vet dependencies, use Cargo.lock |
| Cross-platform path issues | Low | Low | Use Rust's Path and PathBuf abstractions |
| User confusion with subcommands | Low | Medium | Comprehensive help text and examples |

---

## Detailed Implementation Outline

### Phase 1: Foundation (Week 1-2)

#### 1.1 Dependency Management

**File**: [`Cargo.toml`](Cargo.toml:1)

Add required dependencies:
```toml
[dependencies]
# Existing dependencies remain
reqwest = { version = "0.12", features = ["json", "multipart"] }
tokio = { version = "1.40", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
dotenv = "0.15"

# New dependencies
clap = { version = "4.5", features = ["derive"] }
mime_guess = "2.0"

# Optional: for image validation
# image = { version = "0.25", optional = true }
```

**Rationale**:
- `clap` with `derive` feature for clean CLI parsing
- `mime_guess` for automatic MIME type detection from file extensions
- `reqwest` needs `multipart` feature for file uploads
- `image` crate optional - can add validation in future if needed

#### 1.2 Project Structure Reorganization

Create new directory structure:

```
src/
├── main.rs              # Entry point, CLI setup
├── lib.rs               # Library exports (optional, for testing)
├── cli.rs               # CLI argument definitions
├── error.rs             # Error types and handling
├── commands/
│   ├── mod.rs
│   ├── send.rs          # send/message subcommand
│   ├── image.rs         # image subcommand (convenience)
│   └── rich.rs          # rich subcommand (future)
├── discord/
│   ├── mod.rs
│   ├── api.rs           # Discord API client
│   ├── client.rs        # HTTP client wrapper
│   └── types.rs         # Discord API types
├── message/
│   ├── mod.rs
│   ├── builder.rs       # Message builder pattern
│   ├── attachment.rs    # Attachment struct and logic
│   └── validation.rs    # Input validation
└── config/
    ├── mod.rs
    └── env.rs           # Environment variable handling
```

#### 1.3 Error Handling Module

**File**: `src/error.rs`

Define comprehensive error types:
```rust
#[derive(Debug, thiserror::Error)]
pub enum DiscliError {
    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Discord API error: {0}")]
    DiscordApi(String),

    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Attachment error: {0}")]
    Attachment(String),

    #[error("Validation error: {0}")]
    Validation(String),
}

pub type Result<T> = std::result::Result<T, DiscliError>;
```

#### 1.4 Configuration Module

**File**: `src/config/env.rs`

Extract and improve environment handling:
```rust
pub struct Config {
    pub discord_token: String,
    pub channel_id: String,
}

impl Config {
    pub fn load() -> Result<Self> {
        dotenv::from_filename("discli.env").ok();

        let discord_token = env::var("DISCORD_TOKEN")
            .map_err(|_| DiscliError::Config("DISCORD_TOKEN not set".into()))?;

        let channel_id = env::var("DISCORD_CHANNEL_ID")
            .map_err(|_| DiscliError::Config("DISCORD_CHANNEL_ID not set".into()))?;

        Ok(Config { discord_token, channel_id })
    }
}
```

### Phase 2: Discord API Layer (Week 2-3)

#### 2.1 Discord API Client

**File**: `src/discord/client.rs`

Create Discord API client:
```rust
pub struct DiscordClient {
    http_client: reqwest::Client,
    token: String,
    base_url: String,
}

impl DiscordClient {
    pub fn new(token: String) -> Self {
        let http_client = reqwest::Client::new();
        Self {
            http_client,
            token,
            base_url: "https://discord.com/api/v10".to_string(),
        }
    }

    pub async fn send_message(
        &self,
        channel_id: &str,
        message: &DiscordMessage,
    ) -> Result<()> {
        // Implementation based on message type
        // - JSON for simple text
        // - Multipart for attachments
    }

    // Helper methods for constructing URLs, headers, etc.
}
```

#### 2.2 Message Types

**File**: `src/discord/types.rs`

Define Discord API types:
```rust
#[derive(Debug, Clone)]
pub enum DiscordMessage {
    Simple { content: String },
    WithAttachments {
        content: Option<String>,
        attachments: Vec<Attachment>,
    },
    WithEmbeds {
        content: Option<String>,
        embeds: Vec<Embed>,
    },
}

#[derive(Debug, Clone)]
pub struct Attachment {
    pub id: u64,
    pub description: Option<String>,
    pub file_path: Option<String>,
    pub url: Option<String>,
}
```

#### 2.3 Multipart Form Builder

**File**: `src/discord/api.rs`

Implement multipart form construction:
```rust
pub async fn send_multipart_message(
    client: &reqwest::Client,
    url: &str,
    token: &str,
    content: Option<&str>,
    attachments: &[FileAttachment],
) -> Result<()> {
    let mut form = reqwest::multipart::Form::new();

    // Add content if present
    if let Some(text) = content {
        form = form.text("content", text.to_string());
    }

    // Add attachments
    for (index, attachment) in attachments.iter().enumerate() {
        let file = tokio::fs::File::open(&attachment.path).await?;
        let file_len = file.metadata().await?.len();

        // Validate size (Discord limit: 25MB)
        if file_len > 25 * 1024 * 1024 {
            return Err(DiscliError::Attachment(format!(
                "File too large: {} exceeds 25MB limit",
                attachment.path.display()
            )));
        }

        let part = reqwest::multipart::Part::reader_with_length(file, file_len)
            .file_name(attachment.filename.clone())
            .mime_str(&attachment.mime_type)?;

        let key = format!("files[{}]", index);
        form = form.part(key, part);
    }

    // Add payload_json for descriptions
    // ...

    // Send request
    let response = client
        .post(url)
        .header("Authorization", format!("Bot {}", token))
        .multipart(form)
        .send()
        .await?;

    // Handle response
    // ...
}
```

### Phase 3: Message Builder Layer (Week 3)

#### 3.1 Message Builder

**File**: `src/message/builder.rs`

Implement builder pattern:
```rust
pub struct MessageBuilder {
    content: Option<String>,
    attachments: Vec<FileAttachment>,
    embeds: Vec<Embed>,
}

impl MessageBuilder {
    pub fn new() -> Self {
        Self {
            content: None,
            attachments: Vec::new(),
            embeds: Vec::new(),
        }
    }

    pub fn content(mut self, content: impl Into<String>) -> Self {
        self.content = Some(content.into());
        self
    }

    pub fn add_attachment(mut self, path: impl AsRef<Path>) -> Result<Self> {
        let attachment = FileAttachment::from_path(path.as_ref())?;
        self.attachments.push(attachment);
        Ok(self)
    }

    pub fn add_attachments<I, P>(mut self, paths: I) -> Result<Self>
    where
        I: IntoIterator<Item = P>,
        P: AsRef<Path>,
    {
        for path in paths {
            self = self.add_attachment(path)?;
        }
        Ok(self)
    }

    pub fn add_embed(mut self, embed: Embed) -> Self {
        self.embeds.push(embed);
        self
    }

    pub fn build(self) -> DiscordMessage {
        if self.attachments.is_empty() && self.embeds.is_empty() {
            DiscordMessage::Simple {
                content: self.content.unwrap_or_default(),
            }
        } else if !self.attachments.is_empty() {
            DiscordMessage::WithAttachments {
                content: self.content,
                attachments: self.attachments.into_iter().map(Into::into).collect(),
            }
        } else {
            DiscordMessage::WithEmbeds {
                content: self.content,
                embeds: self.embeds,
            }
        }
    }
}
```

#### 3.2 Attachment Handling

**File**: `src/message/attachment.rs`

Handle attachment logic:
```rust
pub struct FileAttachment {
    pub path: PathBuf,
    pub filename: String,
    pub mime_type: String,
    pub size: u64,
    pub description: Option<String>,
}

impl FileAttachment {
    pub fn from_path(path: &Path) -> Result<Self> {
        // Check file exists
        if !path.exists() {
            return Err(DiscliError::Attachment(format!(
                "File not found: {}",
                path.display()
            )));
        }

        // Get file metadata
        let metadata = std::fs::metadata(path)?;
        let size = metadata.len();

        // Validate size
        if size > 25 * 1024 * 1024 {
            return Err(DiscliError::Attachment(
                "File exceeds Discord's 25MB limit".into(),
            ));
        }

        // Determine filename
        let filename = path
            .file_name()
            .and_then(|n| n.to_str())
            .ok_or_else(|| DiscliError::Attachment("Invalid filename".into()))?
            .to_string();

        // Detect MIME type
        let mime_type = mime_guess::from_path(path)
            .first_or_octet_stream()
            .to_string();

        // Validate it's an image (basic check)
        if !mime_type.starts_with("image/") {
            return Err(DiscliError::Attachment(format!(
                "Not an image file: {}",
                mime_type
            )));
        }

        Ok(Self {
            path: path.to_path_buf(),
            filename,
            mime_type,
            size,
            description: None,
        })
    }

    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }
}
```

#### 3.3 Validation Module

**File**: `src/message/validation.rs`

Implement validation logic:
```rust
pub fn validate_attachment_count(count: usize) -> Result<()> {
    const MAX_ATTACHMENTS: usize = 10;
    if count > MAX_ATTACHMENTS {
        return Err(DiscliError::Validation(format!(
            "Cannot attach more than {} images (got {})",
            MAX_ATTACHMENTS, count
        )));
    }
    Ok(())
}

pub fn validate_content_length(content: &str) -> Result<()> {
    const MAX_LENGTH: usize = 2000;
    if content.len() > MAX_LENGTH {
        return Err(DiscliError::Validation(format!(
            "Message content exceeds Discord's {} character limit (got {})",
            MAX_LENGTH,
            content.len()
        )));
    }
    Ok(())
}
```

### Phase 4: CLI Layer (Week 3-4)

#### 4.1 CLI Arguments

**File**: `src/cli.rs`

Define CLI structure using clap:
```rust
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "discli")]
#[command(about = "A CLI tool for sending Discord notifications", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    // Backward compatibility: support direct message argument
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    legacy_message: Vec<String>,
}

#[derive(Subcommand)]
enum Commands {
    /// Send a message to Discord
    #[command(alias = "message")]
    Send {
        /// Message content
        #[arg(required = false)]
        content: String,

        /// Attach image files (can be specified multiple times)
        #[arg(short, long, value_name = "PATH")]
        attach: Vec<PathBuf>,

        /// Embed image URLs (can be specified multiple times)
        #[arg(long, value_name = "URL")]
        embed_url: Vec<String>,

        /// Alt text/description for attachments
        #[arg(short, long, value_name = "TEXT")]
        caption: Option<String>,
    },

    /// Send a message with images (convenience alias for send with images)
    Image {
        /// Image files to attach (can be specified multiple times)
        #[arg(short, long, required = true, value_name = "PATH")]
        attach: Vec<PathBuf>,

        /// Caption text for the images
        #[arg(short, long, value_name = "TEXT")]
        caption: Option<String>,

        /// Embed image URLs instead of uploading
        #[arg(long, value_name = "URL")]
        embed_url: Vec<String>,
    },

    /// Send a rich message with embeds (future)
    #[command(skip)]
    Rich {
        #[arg(short, long)]
        title: Option<String>,

        #[arg(short, long)]
        description: Option<String>,
    },
}
```

#### 4.2 Main Entry Point

**File**: `src/main.rs`

Refactor main to use new architecture:
```rust
mod cli;
mod error;
mod commands;
mod discord;
mod message;
mod config;

use error::{DiscliError, Result};

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

async fn run() -> Result<()> {
    let cli = cli::Cli::parse();

    // Load configuration
    let config = config::load()?;

    // Handle backward compatibility
    if !cli.legacy_message.is_empty() {
        // Warn about deprecation
        eprintln!("Warning: Direct message argument is deprecated. Use 'discli send' instead.");
        eprintln!("  Current: discli \"message\"");
        eprintln!("  New:     discli send \"message\"");
        eprintln!();

        // Use legacy behavior
        let content = cli.legacy_message.join(" ");
        return commands::send::execute(&config, content, Vec::new(), Vec::new(), None).await;
    }

    // Handle subcommands
    match cli.command {
        Some(commands::Commands::Send { content, attach, embed_url, caption }) => {
            commands::send::execute(&config, content, attach, embed_url, caption).await
        }
        Some(commands::Commands::Image { attach, caption, embed_url }) => {
            commands::image::execute(&config, attach, caption, embed_url).await
        }
        None => {
            // Show help if no command provided
            cli::Cli::command().print_help()?;
            Ok(())
        }
    }
}
```

### Phase 5: Command Implementations (Week 4)

#### 5.1 Send Command

**File**: `src/commands/send.rs`

```rust
use crate::error::Result;
use crate::config::Config;
use crate::message::MessageBuilder;

pub async fn execute(
    config: &Config,
    content: String,
    attach: Vec<std::path::PathBuf>,
    embed_url: Vec<String>,
    caption: Option<String>,
) -> Result<()> {
    // Validate attachment count
    crate::message::validation::validate_attachment_count(attach.len() + embed_url.len())?;

    // Validate content length if present
    if !content.is_empty() {
        crate::message::validation::validate_content_length(&content)?;
    }

    // Build message
    let mut builder = MessageBuilder::new();

    if !content.is_empty() {
        builder = builder.content(content);
    }

    // Add file attachments
    for path in attach {
        builder = builder.add_attachment(&path)?;
    }

    // Add URL embeds (future implementation)
    // for url in embed_url {
    //     builder = builder.add_embed_url(&url);
    // }

    let discord_message = builder.build();

    // Send message
    let client = crate::discord::DiscordClient::new(config.discord_token.clone());
    client.send_message(&config.channel_id, &discord_message).await?;

    println!("Message sent successfully to channel {}", config.channel_id);

    Ok(())
}
```

#### 5.2 Image Command

**File**: `src/commands/image.rs`

```rust
use crate::error::Result;
use crate::config::Config;

pub async fn execute(
    config: &Config,
    attach: Vec<std::path::PathBuf>,
    caption: Option<String>,
    embed_url: Vec<String>,
) -> Result<()> {
    // Validate
    crate::message::validation::validate_attachment_count(attach.len() + embed_url.len())?;

    // Build message (content is caption if provided)
    let content = caption.unwrap_or_default();

    // Reuse send command logic
    super::send::execute(config, content, attach, embed_url, None).await
}
```

### Phase 6: Testing and Documentation (Week 5)

#### 6.1 Unit Tests

Add tests for:
- Attachment validation
- Message builder
- CLI parsing
- Error handling

#### 6.2 Integration Tests

Test end-to-end:
- Simple text message
- Single image attachment
- Multiple image attachments
- Text with images
- Error cases (missing file, too large, etc.)

#### 6.3 Documentation Updates

**File**: [`README.md`](README.md:1)

Update with new usage examples:
```markdown
## Usage

### Basic Message
```bash
discli send "Hello, Discord!"
```

### Send with Images
```bash
# Single image
discli send "Check out this screenshot" --attach screenshot.png

# Multiple images
discli send "Report" --attach fig1.png --attach fig2.png --attach fig3.png

# Image only (no text)
discli send --attach photo.jpg
```

### Using Image Command (Convenience)
```bash
discli image --attach screenshot.jpg --caption "Error screenshot"
```

### Legacy Syntax (Deprecated)
```bash
# Old way (still works, but shows warning)
discli "Hello, Discord!"
```
```

---

## Example Usage Scenarios

### Scenario 1: CI/CD Pipeline Notification

**Context**: Automated deployment pipeline wants to send a notification with build screenshot.

```bash
#!/bin/bash
# deploy.sh

# ... deployment steps ...

# Take screenshot of deployment dashboard
scrot -u deployment-result.png

# Send notification with screenshot
discli send "✅ Deployment successful to production
Branch: $CI_COMMIT_REF_NAME
Commit: $CI_COMMIT_SHORT_SHA
Deployed by: $GITLAB_USER_NAME" \
  --attach deployment-result.png
```

### Scenario 2: Automated Monitoring Alert

**Context**: System monitoring detects high CPU usage and captures graph.

```bash
#!/bin/bash
# monitor_cpu.sh

THRESHOLD=80
CURRENT=$(top -bn1 | grep "Cpu(s)" | awk '{print $2}' | cut -d'%' -f1)

if (( $(echo "$CURRENT > $THRESHOLD" | bc -l) )); then
  # Generate CPU graph
  generate_cpu_graph.sh cpu-graph.png

  # Send alert
  discli send "⚠️ High CPU Alert
Current: ${CURRENT}%
Threshold: ${THRESHOLD}%
Timestamp: $(date)" \
    --attach cpu-graph.png

  # Send to different channel
  DISCORD_CHANNEL_ID=987654321098765432 \
  discli send "📊 CPU graph" --attach cpu-graph.png
fi
```

### Scenario 3: Multiple Images from Report

**Context**: Weekly report with multiple charts and images.

```bash
#!/bin/bash
# weekly_report.sh

DATE=$(date +%Y-%m-%d)

# Generate report images
python3 generate_charts.py

# Send comprehensive report
discli send "📈 Weekly Performance Report - $DATE
━━━━━━━━━━━━━━━━━━━━━━━━━
• Revenue charts
• User growth metrics
• Server performance
• Bug statistics
━━━━━━━━━━━━━━━━━━━━━━━━━
Generated: $(date)" \
  --attach revenue_chart.png \
  --attach users_growth.png \
  --attach server_perf.png \
  --attach bug_stats.png
```

### Scenario 4: Error Log Sharing

**Context**: Application crashes with error log screenshot.

```bash
#!/bin/bash
# error_handler.sh

# Capture error details
ERROR_MSG="Application crashed at $(date)"
LOG_FILE="error_$(date +%s).log"

# Write error log
echo "$ERROR_MSG" > "$LOG_FILE"
# ... write more error details ...

# Take screenshot of error UI
scrot -u error_screenshot.png

# Send to team
discli send "🚨 Application Crash Report

Error: $ERROR_MSG
Log file: $LOG_FILE

Please investigate immediately." \
  --attach error_screenshot.png \
  --attach "$LOG_FILE"
```

### Scenario 5: Daily Status Update

**Context**: Daily standup bot sends status update with current Sprint board image.

```bash
#!/bin/bash
# daily_status.sh

# Capture current Sprint board from Jira
curl -s "https://jira.example.com/sprint/board.png" -o sprint.png

# Send status
discli send "📋 Daily Status Update

Sprint Progress: Attached
Blockers: 2
In Progress: 5
Completed: 12
Remaining: 8

Standup at 10:00 AM today." \
  --attach sprint.png
```

### Scenario 6: Image URLs (Future)

**Context**: Images are already hosted externally.

```bash
# Using embedded URLs (future feature)
discli send "Check out our new logo!" \
  --embed-url https://example.com/logo.png

# Mixed approach (future)
discli send "Here are some images" \
  --attach local_screenshot.png \
  --embed-url https://cdn.example.com/chart.png
```

---

## Success Criteria

### Functional Requirements

- [ ] Users can attach up to 10 image files to a Discord message
- [ ] Users can specify image paths via CLI arguments
- [ ] Users can combine text content with image attachments
- [ ] Users can send images without text (images only)
- [ ] System validates image files exist before attempting upload
- [ ] System validates image files don't exceed Discord's 25MB limit
- [ ] System validates total attachments don't exceed Discord's 10 attachment limit
- [ ] System provides clear error messages for invalid inputs
- [ ] System correctly handles various image formats (PNG, JPG, GIF, WebP)
- [ ] System detects MIME types from file extensions
- [ ] Existing text-only functionality continues to work (with deprecation warning)
- [ ] Help documentation includes all new features and examples

### Non-Functional Requirements

- [ ] Response time for small images (< 1MB) < 5 seconds
- [ ] Memory usage reasonable for large file uploads
- [ ] CLI interface is intuitive and discoverable
- [ ] Error messages are helpful and actionable
- [ ] Code is testable with > 80% coverage
- [ ] Code follows Rust best practices and idioms
- [ ] Documentation is comprehensive and up-to-date
- [ ] Backward compatibility maintained for existing users

### Integration Requirements

- [ ] Works with Discord API v10
- [ ] Respects Discord rate limits
- [ ] Handles network timeouts gracefully
- [ ] Works across platforms (Linux, macOS, Windows)
- [ ] Handles file paths with spaces and special characters
- [ ] Supports Unicode characters in filenames and content

---

## Potential Issues and Considerations

### Technical Challenges

#### 1. Multipart Form Data Complexity

**Issue**: Constructing proper multipart/form-data requests can be tricky.

**Mitigation**:
- Use reqwest's proven multipart API
- Write comprehensive tests for various attachment scenarios
- Reference Discord API documentation extensively

#### 2. File Path Handling Across Platforms

**Issue**: Windows vs. Unix path separators and Unicode handling.

**Mitigation**:
- Use Rust's `PathBuf` and `Path` abstractions
- Test on all target platforms
- Handle path edge cases (spaces, special characters)

#### 3. Large File Upload Timeouts

**Issue**: Uploading 25MB files may timeout on slow connections.

**Mitigation**:
- Use progress indication
- Implement configurable timeout with sensible defaults
- Consider streaming for large files

#### 4. Discord Rate Limiting

**Issue**: Sending many messages with attachments may trigger rate limits.

**Mitigation**:
- Implement exponential backoff retry logic
- Provide helpful error messages when rate limited
- Document recommended usage patterns

### User Experience Challenges

#### 1. Command Complexity

**Issue**: Users may find subcommand structure unfamiliar.

**Mitigation**:
- Comprehensive help text with examples
- Clear error messages guiding to correct syntax
- Migration guide for existing users

#### 2. Backward Compatibility Concerns

**Issue**: Changing CLI syntax may break existing scripts.

**Mitigation**:
- Support old syntax with deprecation warnings
- Provide clear migration timeline (6-12 months)
- Document breaking changes prominently

#### 3. Image Validation Confusion

**Issue**: Users may not understand why their file is rejected.

**Mitigation**:
- Provide specific error messages (size, format, etc.)
- Include suggestions for fixing issues
- Document supported formats clearly

### Maintenance Challenges

#### 1. Test Coverage

**Issue**: Testing file upload requires careful handling of test files.

**Mitigation**:
- Use temporary test files in tests
- Mock Discord API responses for unit tests
- Integration tests with test Discord server

#### 2. Dependency Management

**Issue**: Adding dependencies increases maintenance burden.

**Mitigation**:
- Choose well-maintained, widely-used crates
- Pin versions in Cargo.lock
- Regular dependency audits

#### 3. Discord API Changes

**Issue**: Discord may change their API.

**Mitigation**:
- Monitor Discord changelogs
- Pin reqwest version to stable releases
- Abstract Discord API calls behind interface

---

## Future Enhancement Roadmap

### Phase 1: Core Image Support (Planned)

- ✅ File attachment support
- ✅ Multiple attachments per message
- ✅ Text + image combination
- ✅ Basic validation

### Phase 2: Enhanced Image Features

- Image URL embedding (without upload)
- Image description/alt-text
- Image resizing before upload
- Image format conversion
- Thumbnail generation

### Phase 3: Rich Message Support

- Discord embeds with rich formatting
- Embed titles, descriptions, colors
- Embed fields (key-value pairs)
- Embed author, footer, timestamp
- Multiple embeds per message

### Phase 4: Advanced Features

- Message editing
- Message deletion
- Reaction addition
- Thread support
- Webhook support
- Message templates

### Phase 5: UX Improvements

- Interactive mode
- Message preview
- Batch message sending
- Configuration file support
- Multiple channel support
- Message scheduling

### Phase 6: Developer Features

- Plugin system
- Custom message processors
- Webhook listener mode
- REST API mode
- Library mode (use as crate)

---

## Migration Guide

### For Existing Users

#### Before (Current)

```bash
# Simple text message
discli "Hello, Discord!"

# Multi-line message
discli "Line 1
Line 2
Line 3"
```

#### After (Recommended)

```bash
# Simple text message
discli send "Hello, Discord!"

# Multi-line message
discli send "Line 1
Line 2
Line 3"

# Message with image
discli send "Check this out" --attach screenshot.png

# Image only
discli send --attach photo.jpg

# Multiple images
discli send "Report" --attach img1.png img2.png img3.png

# Using image command (convenience)
discli image --attach screenshot.jpg --caption "Error screenshot"
```

#### Transition Period

**Phase 1 (0-6 months)**:
- Old syntax still works
- Deprecation warning shown
- Documentation recommends new syntax

**Phase 2 (6-12 months)**:
- Old syntax still works
- Stronger deprecation warning
- Documentation emphasizes new syntax

**Phase 3 (12+ months)**:
- Old syntax removed
- Error message guides to new syntax
- Documentation only shows new syntax

### For Script Authors

If you have scripts using discli:

```bash
# Old script
#!/bin/bash
discli "Build completed"

# Update to
#!/bin/bash
discli send "Build completed"
```

For CI/CD pipelines:

```yaml
# GitHub Actions (old)
- run: discli "Build successful"

# GitHub Actions (new)
- run: discli send "Build successful"
```

---

## Conclusion

This comprehensive plan provides a clear path forward for adding image support to the discli Discord CLI tool. The recommended **Hybrid Subcommand Architecture (Approach 3)** balances:

- **Simplicity** with **extensibility**
- **Backward compatibility** with **modern CLI patterns**
- **Short-term delivery** with **long-term maintainability**

By following this plan, the discli tool will evolve from a simple text-messaging utility to a full-featured Discord notification tool capable of sending rich media messages, while maintaining the ease of use that makes it valuable for automation and CI/CD pipelines.

The modular architecture allows for incremental implementation, starting with basic image support and expanding to more complex features as needed. The clear separation of concerns makes the codebase easier to test, maintain, and extend over time.

**Next Steps:**
1. Review this plan with stakeholders
2. Confirm timeline and resource availability
3. Begin Phase 1 implementation (foundation)
4. Iterate based on feedback and testing
5. Release with backward compatibility period

---

## Appendix

### A. Discord API Reference

#### Create Message Endpoint

**Endpoint**: `POST /channels/{channel.id}/messages`

**Authentication**: Requires `BOT` token with `SEND_MESSAGES` permission

**Request Body Formats**:

**Format 1: JSON (Simple Messages)**
```json
{
  "content": "Your message here"
}
```

**Format 2: JSON (With Embeds)**
```json
{
  "content": "Message text",
  "embeds": [
    {
      "title": "Embed Title",
      "description": "Embed description",
      "image": {
        "url": "https://example.com/image.png"
      }
    }
  ]
}
```

**Format 3: Multipart/Form-Data (With Attachments)**
```
Content-Type: multipart/form-data; boundary=boundary

--boundary
Content-Disposition: form-data; name="content"

Message text
--boundary
Content-Disposition: form-data; name="payload_json"
Content-Type: application/json

{
  "content": "Message text",
  "attachments": [
    {
      "id": 0,
      "description": "Image description"
    }
  ]
}
--boundary
Content-Disposition: form-data; name="files[0]"; filename="image.png"
Content-Type: image/png

[binary image data]
--boundary--
```

**Limits**:
- Max 2000 characters for content
- Max 10 attachments per message
- Max 25 MB per attachment
- Max 6000 characters per embed description

### B. Rust Crates Evaluation

#### CLI Parsing Libraries

| Crate | Pros | Cons | Recommendation |
|-------|------|------|----------------|
| `clap` 4.x | Feature-rich, derive macro, excellent docs | Larger dependency | ✅ **Recommended** |
| `lexopt` | Zero dependencies, simple | Less features, manual parsing | ⚠️ Alternative |
| `argh` | Simple, minimal | Less mature | ❌ Not recommended |
| `pico-args` | Minimal, fast | No derive macro | ❌ Not recommended |

#### MIME Type Detection

| Crate | Pros | Cons | Recommendation |
|-------|------|------|----------------|
| `mime_guess` 2.x | Good coverage, simple API | Can return wrong types | ✅ **Recommended** |
| `infer` | More accurate via magic bytes | Slower for large files | ⚠️ Alternative |

#### Image Validation (Optional)

| Crate | Pros | Cons | Recommendation |
|-------|------|------|----------------|
| `image` 0.25 | Comprehensive format support | Heavy dependency | ⚠️ Optional |

### C. Testing Strategy

#### Unit Tests

- `Attachment::from_path()` validation
- `MessageBuilder` construction
- `validate_attachment_count()` logic
- `validate_content_length()` logic
- Error message formatting

#### Integration Tests

- Send simple text message
- Send single image
- Send multiple images
- Send text with images
- Invalid file path handling
- File too large handling
- Too many attachments handling

#### Manual Testing Checklist

- [ ] `discli send "text"` works
- [ ] `discli send --attach img.png` works
- [ ] `discli send "text" --attach img.png` works
- [ ] `discli send --attach img1.png img2.png img3.png` works
- [ ] `discli image --attach img.png` works
- [ ] Error on missing file
- [ ] Error on file > 25MB
- [ ] Error on > 10 attachments
- [ ] Deprecation warning for `discli "text"`

### D. Dependencies Summary

#### Required Dependencies

```toml
[dependencies]
reqwest = { version = "0.12", features = ["json", "multipart"] }
tokio = { version = "1.40", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
dotenv = "0.15"
clap = { version = "4.5", features = ["derive"] }
mime_guess = "2.0"
thiserror = "1.0"
```

#### Optional Dependencies

```toml
[dependencies]
# For image validation (add if needed)
image = { version = "0.25", optional = true }

[features]
default = []
image-validation = ["image"]
```

#### Development Dependencies

```toml
[dev-dependencies]
tempfile = "3.10"  # For testing with temp files
mockito = "1.4"    # For mocking HTTP requests (optional)
```

---

*Document Version: 1.0*
*Last Updated: 2026-02-16*
*Author: Kilo Code Architect Mode*
