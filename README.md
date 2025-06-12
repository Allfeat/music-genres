# Allfeat Genre Registry

> Unified music genre metadata shared across Allfeat's Rust (blockchain) and JS (frontend) stacks.

This repository serves as the **canonical registry** of genres and subgenres used across the Allfeat ecosystem, ensuring strong consistency between:

- ✅ The Rust runtime (via build.rs and generated enums)
- ✅ The TypeScript SDK (used by web apps and tools)

---

## 🔗 Structure

```
/
├── genres.json             # Canonical source of genre & subgenre definitions
├── rust/                   # Rust crate for Substrate runtime integration
│   ├── build.rs            # Enum and validation code generator
│   └── src/lib.rs          # Auto-generated genre enum and helpers
├── js/                     # TypeScript package for frontends & SDK
│   ├── generate.ts         # Reads genres.json and emits unified genre list
│   └── src/index.ts        # Auto-generated genre utilities
```

---

## ⚙️ Canonical Format: `genres.json`

All genre definitions live in a single shared file at the root:

```json
{
  "genres": [
    {
      "id": "rock",
      "name": "Rock",
      "subgenres": [
        { "id": "hard_rock", "name": "Hard Rock" },
        { "id": "punk_rock", "name": "Punk Rock" }
      ]
    },
    ...
  ]
}
```

---

## 🔄 Regenerate code

Every time you modify `genres.json`, regenerate both sides:

### For Rust (Substrate enum)

```bash
cd rust && BUILD_GENRES=1 cargo build
```

### For JS (TS SDK and helpers)

```bash
cd js && pnpm generate
```

---

## ⚖️ Shared Principles

- A **single source of truth**: `genres.json`
- Deterministic builds with build.rs and generate.ts
- No runtime logic duplication
- Auto-generated enum values use `PascalCase` for on-chain usage

---

## 🏠 Use cases

- ✅ Blockchain: validation, storage, certification of metadata
- ✅ Frontend: form dropdowns, filtering, tagging
- ✅ SDK: programmatic access to genre IDs with validation
