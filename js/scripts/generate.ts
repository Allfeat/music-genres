import fs from "fs";
import path from "path";

import { AllfeatMusicGenresGeneratedGenreId } from "@allfeat/chaintypes/allfeat-melodie";
import type { INativeTypeConverter } from "@allfeat/midds";

/**
 * Converts a string to PascalCase.
 */
function toPascalCase(str: string): string {
  return str
    .split(/[_\-\s]+/)
    .map((s) => s.charAt(0).toUpperCase() + s.slice(1))
    .join("");
}

/**
 * Type that distinguishes if a genre entry is a parent genre or a subgenre.
 */
export type GenreType = "genre" | "subgenre";

/**
 * Represents a genre or subgenre entry with metadata and conversion to native chain type.
 */
export interface UnifiedGenreEntry
  extends INativeTypeConverter<AllfeatMusicGenresGeneratedGenreId> {
  id: string; // snake_case identifier used internally and on-chain
  name: string; // Human-readable genre label
  type: GenreType; // Either 'genre' or 'subgenre'
  parentId?: string; // Only set if it's a subgenre
  toNativeType(): AllfeatMusicGenresGeneratedGenreId; // Converts to Substrate enum variant
}

const rawGenres = JSON.parse(
  fs.readFileSync(path.resolve("../genres.json"), "utf-8")
) as {
  genres: {
    id: string;
    name: string;
    subgenres: { id: string; name: string }[];
  }[];
};

const allGenresUnified: UnifiedGenreEntry[] = [];

for (const genre of rawGenres.genres) {
  allGenresUnified.push({
    id: genre.id,
    name: genre.name,
    type: "genre",
    toNativeType: () =>
      toPascalCase(genre.id) as AllfeatMusicGenresGeneratedGenreId,
  });

  for (const sub of genre.subgenres) {
    allGenresUnified.push({
      id: sub.id,
      name: sub.name,
      type: "subgenre",
      parentId: genre.id,
      toNativeType: () =>
        toPascalCase(sub.id) as AllfeatMusicGenresGeneratedGenreId,
    });
  }
}

const out = `// AUTO-GENERATED FILE – DO NOT EDIT MANUALLY
// Generated from genres.json

import { AllfeatMusicGenresGeneratedGenreId } from "@allfeat/chaintypes/allfeat-melodie";

/**
 * Type that distinguishes if a genre entry is a parent genre or a subgenre.
 */
export type GenreType = "genre" | "subgenre";

/**
 * Represents a genre or subgenre entry with metadata and conversion to chain-compatible enum.
 */
export interface UnifiedGenreEntry {
  id: string;
  name: string;
  type: GenreType;
  parentId?: string;
  toNativeType(): AllfeatMusicGenresGeneratedGenreId;
}

/**
 * Flat list of all music genres and subgenres.
 */
export const allGenresUnified: UnifiedGenreEntry[] = [
${allGenresUnified
  .map(
    (g) => `  {
    id: "${g.id}",
    name: "${g.name}",
    type: "${g.type}",${
      g.parentId
        ? `
    parentId: "${g.parentId}",`
        : ""
    }
    toNativeType: () => "${toPascalCase(g.id)}",
  }`
  )
  .join(",\n")}
];

/**
 * Retrieve the display name of a genre or subgenre by its ID.
 */
export function getGenreName(id: string): string | null {
  const entry = allGenresUnified.find(g => g.id === id);
  return entry ? entry.name : null;
}

/**
 * Check if the given ID corresponds to a valid genre or subgenre.
 */
export function isValidGenreId(id: string): boolean {
  return allGenresUnified.some(g => g.id === id);
}

/**
 * Get all subgenres of a given genre ID.
 */
export function getSubgenresOf(parentId: string): UnifiedGenreEntry[] {
  return allGenresUnified.filter(g => g.parentId === parentId);
}

/**
 * Get only the top-level genres (excluding subgenres).
 */
export function getGenres(): UnifiedGenreEntry[] {
  return allGenresUnified.filter(g => g.type === "genre");
}
`;

fs.writeFileSync(path.resolve("src/index.ts"), out);
console.log("✅ Unified genre entries generated");
