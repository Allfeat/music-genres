# @allfeat/music-genres

> Unified music genres and subgenres for Allfeat frontends and the Substrate-based Allfeat blockchain.

This package provides a flat, auto-generated list of music genres and subgenres, each mapped to a valid enum value used on-chain (`AllfeatMusicGenresGeneratedGenreId`). It ensures consistency between frontend applications and blockchain logic, while exposing useful helper functions for selection and validation.

---

## ✨ Features

- Flat list of genres and subgenres
- `genre` vs `subgenre` distinction via `type`
- Each entry maps to on-chain enum using `.toNativeType()`
- Lookup helpers for validation and name resolution
- Pure TypeScript structure usable in any frontend

---

## 📅 Installation

```bash
pnpm add @allfeat/music-genres
```

---

## 📖 Usage

```ts
import {
  allGenresUnified,
  getGenres,
  getSubgenresOf,
  isValidGenreId,
  getGenreName,
} from "@allfeat/music-genres";

const genres = getGenres();
const subgenresOfPop = getSubgenresOf("pop");

if (isValidGenreId("trap")) {
  const display = getGenreName("trap"); // "Trap"
}
```

---

## 🔎 UnifiedGenreEntry

```ts
interface UnifiedGenreEntry {
  id: string; // snake_case ID from genres.json
  name: string; // Human-readable name
  type: "genre" | "subgenre"; // Classification
  parentId?: string; // Present only for subgenres
  toNativeType(): GenreEnum; // Converts to on-chain enum
}
```

---

## ⚙️ Generation

This package is generated from a shared `../genres.json` file using a `generate.ts` script.

```bash
pnpm run generate
```

This ensures that genres remain synchronized between your frontends and the Substrate runtime definitions.

---

## 👷 Scripts

```bash
pnpm run generate     # Regenerate the output from genres.json
pnpm run format       # Format with Prettier
```
