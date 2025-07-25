use serde::Deserialize;
use std::{env, fs, path::PathBuf};

#[derive(Deserialize)]
struct GenreJson {
    genres: Vec<Genre>,
}

#[derive(Deserialize)]
struct Genre {
    id: String,
    name: String,
    subgenres: Vec<SubGenre>,
}

#[derive(Deserialize)]
struct SubGenre {
    id: String,
    name: String,
}

fn main() {
    let build_condition = env::var("BUILD_GENRES").is_ok();
    let out_file = PathBuf::from("src/generated.rs");
    let wasm_api_file = PathBuf::from("src/wasm_api_generated.rs");

    if !build_condition {
        println!("cargo:warning=Skipping build.rs execution — BUILD_GENRES not set");
        return;
    }

    let root_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let json_path = root_dir.join("../genres.json");
    let header_path = root_dir.join("HEADER");

    let content = fs::read_to_string(&json_path).expect("Cannot read genres.json");
    let parsed: GenreJson = serde_json::from_str(&content).expect("Invalid JSON structure");

    let header = if header_path.exists() {
        fs::read_to_string(header_path).expect("Failed to read HEADER file")
    } else {
        String::new()
    };

    let mut all_variants = vec![];

    for genre in &parsed.genres {
        let genre_name = format!("// ===== Genre: {} =====", genre.name);
        all_variants.push(genre_name);

        // Genre principal
        all_variants.push(format!("    {},", to_camel_case(&genre.id)));

        // Sous-genres
        for sub in &genre.subgenres {
            all_variants.push(format!("    {},", to_camel_case(&sub.id)));
        }

        all_variants.push(String::new()); // ligne vide
    }

    let enum_definition = format!(
        "{header}
// AUTO-GENERATED FILE. DO NOT EDIT MANUALLY.

use parity_scale_codec::{{Encode, Decode, MaxEncodedLen, DecodeWithMemTracking}};
use scale_info::TypeInfo;
use sp_runtime::RuntimeDebug;

#[cfg(feature = \"js\")]
use serde::{{Serialize, Deserialize}};

#[cfg(feature = \"js\")]
use wasm_bindgen::prelude::*;

/// Flat enum containing all main genres and subgenres.
/// Subgenres are grouped under genre-level comments.
/// This enum is used directly in the blockchain to identify any genre type.
#[derive(
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    RuntimeDebug,
    Encode,
    Decode,
    DecodeWithMemTracking,
    TypeInfo,
    MaxEncodedLen,
)]
#[cfg_attr(feature = \"js\", derive(Serialize, Deserialize))]
#[cfg_attr(feature = \"js\", wasm_bindgen)]
pub enum GenreId {{
{variants}
}}",
        header = header.trim_end(),
        variants = all_variants.join("\n")
    );

    fs::write(&out_file, enum_definition).expect("Failed to write generated.rs");

    // Generate WASM API entries
    generate_wasm_api(&parsed, &header, &wasm_api_file);

    println!("cargo:rerun-if-env-changed=BUILD_GENRES");
    println!("cargo:rerun-if-changed=../genres.json");
    println!("cargo:rerun-if-changed=HEADER");
}

fn generate_wasm_api(parsed: &GenreJson, header: &str, out_file: &PathBuf) {
    let mut entries = vec![];

    for genre in &parsed.genres {
        // Main genre entry
        entries.push(format!(
            "        super::GenreEntry {{
            id: \"{}\".to_string(),
            name: \"{}\".to_string(),
            genre_type: super::GenreType::Genre,
            parent_id: None,
            genre_id: GenreId::{},
        }},",
            genre.id,
            genre.name.replace("\"", "\\\""),
            to_camel_case(&genre.id)
        ));

        // Subgenre entries
        for subgenre in &genre.subgenres {
            entries.push(format!(
                "        super::GenreEntry {{
            id: \"{}\".to_string(),
            name: \"{}\".to_string(),
            genre_type: super::GenreType::Subgenre,
            parent_id: Some(\"{}\".to_string()),
            genre_id: GenreId::{},
        }},",
                subgenre.id,
                subgenre.name.replace("\"", "\\\""),
                genre.id,
                to_camel_case(&subgenre.id)
            ));
        }
    }

    let wasm_api_content = format!(
        "{header}
// AUTO-GENERATED FILE. DO NOT EDIT MANUALLY.

use crate::GenreId;

/// Auto-generated function to get all genre entries
pub(crate) fn get_all_genre_entries_generated() -> Vec<super::GenreEntry> {{
    vec![
{entries}
    ]
}}",
        header = header.trim_end(),
        entries = entries.join("\n")
    );

    fs::write(out_file, wasm_api_content).expect("Failed to write wasm_api_generated.rs");
}

fn to_camel_case(id: &str) -> String {
    const SEPARATORS: &[char] = &['_', '-', ' '];

    id.split(SEPARATORS)
        .filter(|s| !s.is_empty())
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                None => String::new(),
            }
        })
        .collect()
}
