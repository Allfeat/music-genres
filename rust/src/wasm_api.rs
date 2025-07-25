// This file is part of Allfeat.

// Copyright (C) 2022-2025 Allfeat.
// SPDX-License-Identifier: GPL-3.0-or-later

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};

#[cfg(feature = "js")]
include!("wasm_api_generated.rs");

/// Initialize WASM panic hook for better error messages
#[wasm_bindgen(start)]
pub fn init() {
    console_error_panic_hook::set_once();
}

/// Type that distinguishes if a genre entry is a parent genre or a subgenre
#[wasm_bindgen]
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GenreType {
    Genre,
    Subgenre,
}

/// Represents a genre or subgenre entry with metadata
#[wasm_bindgen]
#[derive(Clone)]
pub struct GenreEntry {
    id: String,
    name: String,
    genre_type: GenreType,
    parent_id: Option<String>,
    genre_id: crate::GenreId,
}

/// Serde-compatible version for serialization
#[derive(Clone, Serialize, Deserialize)]
struct GenreEntrySerializable {
    id: String,
    name: String,
    genre_type: GenreType,
    parent_id: Option<String>,
    genre_id_str: String,
}

impl From<&GenreEntry> for GenreEntrySerializable {
    fn from(entry: &GenreEntry) -> Self {
        Self {
            id: entry.id.clone(),
            name: entry.name.clone(),
            genre_type: entry.genre_type,
            parent_id: entry.parent_id.clone(),
            genre_id_str: format!("{:?}", entry.genre_id),
        }
    }
}

#[wasm_bindgen]
impl GenreEntry {
    #[wasm_bindgen(getter)]
    pub fn id(&self) -> String {
        self.id.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.name.clone()
    }

    #[wasm_bindgen(getter, js_name = genreType)]
    pub fn genre_type(&self) -> GenreType {
        self.genre_type
    }

    #[wasm_bindgen(getter, js_name = parentId)]
    pub fn parent_id(&self) -> Option<String> {
        self.parent_id.clone()
    }

    #[wasm_bindgen(getter, js_name = genreId)]
    pub fn genre_id(&self) -> crate::GenreId {
        self.genre_id
    }

    #[wasm_bindgen(js_name = toNativeType)]
    pub fn to_native_type(&self) -> crate::GenreId {
        self.genre_id
    }
}

/// Main API class for genre operations
#[wasm_bindgen]
pub struct GenreApi;

#[wasm_bindgen]
impl GenreApi {
    #[wasm_bindgen(constructor)]
    pub fn new() -> GenreApi {
        GenreApi
    }

    /// Get all genres and subgenres as a flat array
    #[wasm_bindgen(js_name = getAllGenres)]
    pub fn get_all_genres(&self) -> Result<JsValue, JsValue> {
        let entries = get_all_genre_entries();
        let serializable: Vec<GenreEntrySerializable> = entries.iter().map(|e| e.into()).collect();
        serde_wasm_bindgen::to_value(&serializable)
            .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
    }

    /// Get only top-level genres (excluding subgenres)
    #[wasm_bindgen(js_name = getGenres)]
    pub fn get_genres(&self) -> Result<JsValue, JsValue> {
        let entries: Vec<GenreEntry> = get_all_genre_entries()
            .into_iter()
            .filter(|entry| entry.genre_type == GenreType::Genre)
            .collect();
        let serializable: Vec<GenreEntrySerializable> = entries.iter().map(|e| e.into()).collect();
        serde_wasm_bindgen::to_value(&serializable)
            .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
    }

    /// Get all subgenres of a given genre ID
    #[wasm_bindgen(js_name = getSubgenresOf)]
    pub fn get_subgenres_of(&self, parent_id: &str) -> Result<JsValue, JsValue> {
        let entries: Vec<GenreEntry> = get_all_genre_entries()
            .into_iter()
            .filter(|entry| entry.parent_id.as_ref() == Some(&parent_id.to_string()))
            .collect();
        let serializable: Vec<GenreEntrySerializable> = entries.iter().map(|e| e.into()).collect();
        serde_wasm_bindgen::to_value(&serializable)
            .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
    }

    /// Get genre entry by ID
    #[wasm_bindgen(js_name = getGenreById)]
    pub fn get_genre_by_id(&self, id: &str) -> Option<GenreEntry> {
        get_all_genre_entries()
            .into_iter()
            .find(|entry| entry.id == id)
    }

    /// Get genre name by ID
    #[wasm_bindgen(js_name = getGenreName)]
    pub fn get_genre_name(&self, id: &str) -> Option<String> {
        self.get_genre_by_id(id).map(|entry| entry.name)
    }

    /// Check if the given ID corresponds to a valid genre or subgenre
    #[wasm_bindgen(js_name = isValidGenreId)]
    pub fn is_valid_genre_id(&self, id: &str) -> bool {
        get_all_genre_entries()
            .iter()
            .any(|entry| entry.id == id)
    }

    /// Convert a GenreId enum variant to its string representation
    #[wasm_bindgen(js_name = genreIdToString)]
    pub fn genre_id_to_string(&self, genre_id: crate::GenreId) -> String {
        format!("{:?}", genre_id)
    }

    /// Get hierarchical structure with genres and their subgenres
    #[wasm_bindgen(js_name = getHierarchicalGenres)]
    pub fn get_hierarchical_genres(&self) -> Result<JsValue, JsValue> {
        #[derive(Serialize)]
        struct HierarchicalGenre {
            id: String,
            name: String,
            genre_id_str: String,
            subgenres: Vec<GenreEntrySerializable>,
        }

        let all_entries = get_all_genre_entries();
        let genres: Vec<HierarchicalGenre> = all_entries
            .iter()
            .filter(|entry| entry.genre_type == GenreType::Genre)
            .map(|genre| {
                let subgenres: Vec<GenreEntry> = all_entries
                    .iter()
                    .filter(|entry| entry.parent_id.as_ref() == Some(&genre.id))
                    .cloned()
                    .collect();
                
                HierarchicalGenre {
                    id: genre.id.clone(),
                    name: genre.name.clone(),
                    genre_id_str: format!("{:?}", genre.genre_id),
                    subgenres: subgenres.iter().map(|e| e.into()).collect(),
                }
            })
            .collect();

        serde_wasm_bindgen::to_value(&genres)
            .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
    }
}

/// Internal function to get all genre entries (uses auto-generated data)
#[cfg(feature = "js")]
fn get_all_genre_entries() -> Vec<GenreEntry> {
    get_all_genre_entries_generated()
}

/// Fallback for when js feature is not enabled
#[cfg(not(feature = "js"))]
fn get_all_genre_entries() -> Vec<GenreEntry> {
    vec![]
}