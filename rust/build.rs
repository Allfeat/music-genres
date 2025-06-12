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
}

fn main() {
    let build_condition = env::var("BUILD_GENRES").is_ok();
    let out_file = PathBuf::from("src/generated.rs");

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
pub enum GenreId {{
{variants}
}}",
        header = header.trim_end(),
        variants = all_variants.join("\n")
    );

    fs::write(&out_file, enum_definition).expect("Failed to write generated.rs");

    println!("cargo:rerun-if-env-changed=BUILD_GENRES");
    println!("cargo:rerun-if-changed=../genres.json");
    println!("cargo:rerun-if-changed=HEADER");
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
