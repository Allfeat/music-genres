# 🎵 `music-genres` — Allfeat Genre Registry

> Rust crate that exposes a unified taxonomy of musical genres and subgenres for use on the Allfeat blockchain.

---

## 📦 Purpose

This crate provides a **flat `GenreId` enum** that includes all standardized music genres and subgenres. It is **auto-generated** from a central `genres.json` file.

It enables:

- Direct use of `GenreId` on-chain in Substrate pallets (Allfeat blockchain)
- A single source of truth shared with SDKs (TypeScript, UI, APIs)
- Seamless compatibility between frontend hierarchical structures and flat on-chain storage

---

## 📁 Key Files

- `genres.json`: the central taxonomy file (main genres and subgenres)
- `src/generated.rs`: auto-generated Rust enum `GenreId`
- `build.rs`: generates `generated.rs` from `genres.json`

---

## ⚙️ Code Generation (`build.rs`)

Run the following to trigger generation (in dev only):

```bash
BUILD_GENRES=1 cargo build
```

This produces `src/generated.rs`, which includes:

- A flat `GenreId` enum (main + subgenres)
- Grouping comments separating each main genre
- Full Substrate runtime compatibility via:
  `Clone, Copy, PartialEq, Eq, PartialOrd, Ord, RuntimeDebug, Encode, Decode, DecodeWithMemTracking, TypeInfo, MaxEncodedLen`

🔒 This file is **committed to version control** and **must not be edited manually**.

---

## 🧠 Design Philosophy: Flat On-Chain, Hierarchical Off-Chain

### ✅ On-Chain (Substrate)

```rust
// Generated enum:
GenreId::Rock
GenreId::HardRock
GenreId::Trap
// All genres (main or sub) are equal from the blockchain's perspective
```

### ✅ Off-Chain (SDK / Frontend)

```ts
// Used in the UI/SDK as hierarchical structure
{
  id: "rock",
  name: "Rock",
  subgenres: [
    { id: "hard_rock", name: "Hard Rock" },
    { id: "classic_rock", name: "Classic Rock" },
  ]
}
```

The SDK is responsible for displaying and validating hierarchical logic. On-chain, only a flat `GenreId` is stored.

---

## 💠 Updating the Genre List

1. Modify `genres.json`
2. Regenerate the Rust file:

   ```bash
   BUILD_GENRES=1 cargo build
   ```

3. Review and commit the changes to `src/generated.rs`
4. **Do not manually edit `generated.rs`**
